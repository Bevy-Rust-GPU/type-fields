{-# LANGUAGE MultiParamTypeClasses #-}
{-# LANGUAGE FlexibleInstances #-}
{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE GADTs #-}
{-# LANGUAGE StandaloneDeriving #-}
{-# LANGUAGE TypeOperators #-}
{-# LANGUAGE ScopedTypeVariables #-}
 
import Data.Monoid
import Data.Functor
import Data.Bifunctor (bimap)
import Control.Applicative
import Control.Monad
import Control.Arrow
import Algebra.Lattice
import Codec.Picture
import Codec.Picture.Types
import Codec.Picture.Saving
import Data.ByteString.Lazy.Base64
import Data.Text.Lazy hiding (length, unlines)

-- Functional replacement of if-then-else sugar
if' :: Bool -> a -> a -> a
if' True a _ = a
if' False _ b = b

-- Infinity
inf :: (Fractional a) => a
inf = 1.0 / 0.0

-- Negative infinity
negInf :: (Fractional a) => a
negInf = -inf

--- Implement concrete bounded lattice instances for numeric types

instance Lattice Int where
    (/\) = min
    (\/) = max

instance BoundedMeetSemiLattice Int where
    top = maxBound

instance BoundedJoinSemiLattice Int where
    bottom = minBound

instance Lattice Float where
    (/\) = min
    (\/) = max

instance BoundedMeetSemiLattice Float where
    top = 1.0 / 0.0

instance BoundedJoinSemiLattice Float where
    bottom = -1.0 / 0.0

instance Lattice Double where
    (/\) = min
    (\/) = max

instance BoundedMeetSemiLattice Double where
    top = inf

instance BoundedJoinSemiLattice Double where
    bottom = negInf

-- Clamp value `a` to the provided range `f`..`t`
clamp :: (Ord a) => a -> a -> a -> a
clamp f t a = a `max` f `min` t

-- Clamp input to the range 0.0..1.0
saturate :: (Fractional a, Ord a) => a -> a
saturate = clamp 0.0 1.0

-- Negate the output of a function that returns a Num
neg :: (Num c) => (a -> c) -> a -> c
neg f = (0-) . f

-- Calculate the scalar product of a pair
dot :: (Num a) => (a, a) -> (a, a) -> a
dot (x1, y1) (x2, y2) = x1 * x2 + y1 * y2

-- Return the square length of the given pair
sqlen :: (Num a) => (a, a) -> a
sqlen a = dot a a

-- Return the length of the given pair
len :: (Floating a) => (a, a) -> a
len = sqrt . sqlen

-- Shrink the provided vector to length 1
normalize :: (Floating a) => (a, a) -> (a, a)
normalize v@(x, y) = let l = len v in (x / l, y / l)

-- Reflect the given vector about the given normal
reflect n@(nx, ny) d@(dx, dy) =
    (dx - 2 * dn * nx, dy - 2 * dn * ny)
    where dn = d `dot` n

-- Convenience wrapper to automatically lift input into the applicative
pureA2 f a = liftA2 f a . pure

-- Lift a binary function and its input into the applicative, and use it fold a list using meet (/\)
foldMeets :: (BoundedMeetSemiLattice a1, Foldable f, Applicative f) => (a2 -> b -> a1) -> f a2 -> b -> a1
foldMeets f a p = meets $ pureA2 f a p

-- Lift a binary function and its input into the applicative, and use it fold a list using join (\/)
foldJoins :: (BoundedJoinSemiLattice a1, Foldable f, Applicative f) => (a2 -> b -> a1) -> f a2 -> b -> a1
foldJoins f a p = joins $ pureA2 f a p


-- Subtraction operator
f |- g = f \/ neg g

--- Distance modifiers

-- Shift the isosurface along the field by a given amount.
-- Practically, this adds additional radius to an existing shape.
isosurface :: Num a => a -> a -> a
isosurface = flip (-)

-- Shift a constant field by the given distance
-- Practically, this inverts the right-hand field,
-- and saturates its interior to the constant distance.
--
-- Primarily useful for visualization.
coisosurface = (-)

-- Swap the field's interior and exterior
invert :: Num a => a -> a
invert = (0-)

-- Convert the interior of the field into an exterior
hollow :: Num a => a -> a
hollow = abs

annulate :: Num a => a -> a -> a
annulate r = isosurface r . hollow

-- Scale a signed distance by the provided factor
--
-- Note: Applied to a distance function, this will produce a distorted bound.
-- Primarily intended for visualization.
scaled = flip (*)

--- Position modifiers

elongate d (x, y) = (x'', y)
    where
        x' = abs x
        x'' = x' - (x' /\ d)

translate :: (Num a, Num b) => (a, b) -> (a, b) -> (a, b)
translate (x, y) (x2, y2) = (x + x2, y + y2)

--- Signed distance functions

point :: (Floating a) => (a, a) -> a
point = len

plane :: (Num a) => (a, a) -> (a, a) -> a
plane n = dot n

line d = elongate d <&> point

triangle r = (line' <$> down)
         /\ (line' <$> up <$> reflect axisR)
         /\ (line' <$> up <$> reflect axisL)
         where
         r2 = r * 2
         root = sqrt 3 / 2
         line' = line (r2 * root)
         down = translate (0, -r)
         up = translate (0, r)
         axisR = normalize (1, -0.5)
         axisL = normalize (-1, -0.5)

circle :: (Floating a) => a -> (a, a) -> a
circle r = point <&> isosurface r

ring :: (Floating a) => a -> (a, a) -> a
ring r = circle r <&> hollow

capsule d r = elongate d <&> circle r

--- Gradient functions

pointGrad :: Floating a => (a, a) -> (a, a)
pointGrad = normalize

--- ASCII rasterization

-- Output a list of rows, where a row is a list of 2-tuple positions
raster :: (Integral a, Floating b) => a -> a -> [[(b, b)]]
raster ex ey =
    [
        [
          ( i, j)
          | i <- [ 0..ex ] <&> fromIntegral <&> (+0.5) <&> (/ex') <&> toHom
        ] | j <- [ 0..ey ] <&> fromIntegral <&> (+0.5) <&> (/ey') <&> toHom
    ]
    where
    ex' = fromIntegral ex
    ey' = fromIntegral ey

-- Perceptual brightness ramp
asciiRamp :: String
asciiRamp = "█@%#*+=-:. "

-- Maximum valid ramp index
asciiMax :: Int
asciiMax = length asciiRamp - 1

-- Utility wrapper for indexing into the ascii ramp
ramp :: Int -> Char
ramp a = asciiRamp !! a

-- Given a decimal, produce a valid ascii ramp index
rampIdx :: (RealFrac a, Ord a, Integral b) => a -> b
rampIdx a = round $ saturate a * fromIntegral asciiMax

-- Index into the ascii ramp using a decimal value
ramp' :: RealFrac a => a -> Char
ramp' a = ramp $ rampIdx a

-- Print a list of strings to standard output
putStrLns :: [String] -> IO ()
putStrLns = putStrLn . unlines

-- Rasterize a 2D signed distance field and print the result to standard output
ascii ex ey f = putStrLns $ raster ex ey <&> \l -> l <&> f <&> ramp'

-- Pre-curried toAscii with fixed parameters
ascii' f = ascii 64 32 (f <&> scaled 1.7)

--- Bitmap rasterization

-- Convert from 0..1 to -1..1
toHom a = a * 2 - 1

-- Convert from -1..1 to 0..1
fromHom a = a / 2 + 0.5

tupleMap = join bimap

tupleToHom :: Num a => (a, a) -> (a, a)
tupleToHom = tupleMap toHom

tupleFromHom :: Fractional a => (a, a) -> (a, a)
tupleFromHom = tupleMap fromHom

coordNorm :: Fractional a => Int -> Int -> Int -> Int -> (a, a)
coordNorm w h x y = tupleToHom $ (x' / w', y' / h') where
    [x', y', w', h'] = fromIntegral <$> [x, y, w, h]

norm w h f x y = f p where p = coordNorm w h x y

toImage w h f = generateImage f' w h where f' = norm w h f

-- Given an (Image PixelRGBF), construct a DynamicImage, convert it to RGB, and encode it as a bitmap
toBitmap = encodeBitmap . convertRGB8 . ImageRGBF

-- Given a lazy ByteString, encode it as Base64 and unpack it into a String
toBase64 = unpack . encodeBase64

-- Generate an xterm escape sequence to display the given base64-encoded image
image :: Int -> [Char] -> [Char]
image s i = "\ESC]1337;File=[name=;width=" <> show s <> ";inline=1]:" <> i <> "\^G"

-- Print an xterm escape sequence to display the given base64-encoded image
putImage :: Int -> [Char] -> IO ()
putImage s i = putStrLn $ image s i

-- Print the given function to the terminal as an image
bitmap is w h f = putImage is $ toBase64 $ toBitmap $ toImage w h f

bitmapDist is w h f = bitmap is w h (promotePixel . (f <&> scaled 6.0 <&> coisosurface 1))

gradToPixel :: (Float, Float) -> PixelRGBF
gradToPixel (x, y) = PixelRGBF (promotePixel x) (promotePixel y) 1.0

bitmapGrad is w h f = bitmap is w h (f <&> tupleFromHom <&> gradToPixel)

-- Pre-curried utility bitmaps
bitmapDist' f = bitmapDist 32 128 128 f
bitmapGrad' f = bitmapGrad 32 128 128 f

--- Test code

sdf :: (Float, Float) -> Float
sdf = (foldMeets ring $ [3.0, 5.5, 8] <&> (/20.0))
   /\ ring 0.8
   /\ triangle 0.4

sdfa = ascii' sdf
sdfb = bitmapDist' sdf
sdfc = bitmapGrad' pointGrad

meetFirst a b = a /\ b == a
meetSecond a b = a /\ b == b

joinFirst a b = a \/ b == a
joinSecond a b = a \/ b == a

--- Attributes

-- Wrapper which implements Lattice based on its first parameter
-- Used to include extra data with a Lattice type,
-- which is discriminated alongside it during meet / join operations
data LatticeHead a b = LatticeHead a b deriving Show

instance (Lattice a, Eq a) => Lattice (LatticeHead a b) where
    a'@(LatticeHead a1 a2) /\ b'@(LatticeHead b1 b2) =
        if' (meetFirst a1 b1) a' b'

    a'@(LatticeHead a1 a2) \/ b'@(LatticeHead b1 b2) =
        if' (joinFirst a1 b1) a' b'

unLatticeHead (LatticeHead a b) = (a, b)

distAttrs a d = (d &&& id) >>> a >>> (uncurry LatticeHead)
distGrad = distAttrs (second pointGrad)

--- Field functions as datatypes
--- Associates multiple field I/O pairs with a single type
--- Intended to solve multi-attribute composition, but suffers from the expression problem

data Dist a where
    Dist :: (Num a) => a -> Dist a

deriving instance Show a => Show (Dist a)
deriving instance Eq a => Eq (Dist a)
deriving instance Ord a => Ord (Dist a)

instance Lattice a => Lattice (Dist a) where
    Dist a /\ Dist b = Dist (a /\ b)
    Dist a \/ Dist b = Dist (a \/ b)

data Grad a where
    Grad :: (Num a) => (a, a) -> Grad (a, a)

deriving instance Show a => Show (Grad (a, a))

class Field a b c where
    field :: a -> (b -> c)

data Point = Point deriving Show

instance (Floating a) => Field Point (a, a) (Dist a) where
    field Point = Dist . point

instance (Floating a) => Field Point (a, a) (Grad (a, a)) where
    field Point = Grad . pointGrad

data Circle a = Circle a deriving Show

instance (Floating a) => Field (Circle a) (a, a) (Dist a) where
    field (Circle r) = Dist . circle r

