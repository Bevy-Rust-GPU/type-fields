{-# LANGUAGE GADTs #-}
{-# LANGUAGE Rank2Types #-}
{-# LANGUAGE ScopedTypeVariables #-}
{-# LANGUAGE MultiParamTypeClasses #-}
{-# LANGUAGE FlexibleInstances #-}
{-# LANGUAGE FlexibleContexts #-}
{-# LANGUAGE QuantifiedConstraints #-}
{-# LANGUAGE FunctionalDependencies #-}
module Main
        where

import Prelude hiding ((.), id)
import Control.Category
import Control.Arrow
import Control.Applicative
import Data.Monoid
import Data.Map as Map

data FreeA eff a b where
    Pure :: (a -> b) -> FreeA eff a b
    Effect :: eff a b -> FreeA eff a b
    Seq :: FreeA eff a b -> FreeA eff b c -> FreeA eff a c
    Par :: FreeA eff a₁ b₁ -> FreeA eff a₂ b₂ -> FreeA eff (a₁, a₂) (b₁, b₂)
    
analyze :: forall f eff a₀ b₀ r. (Applicative f, Monoid r)
        => (forall a b. eff a b -> f r)
        -> FreeA eff a₀ b₀ -> f r
analyze visit = go
  where
    go :: forall a b. FreeA eff a b -> f r
    go arr = case arr of
        Pure _ -> pure mempty
        Seq f₁ f₂ -> mappend <$> go f₁ <*> go f₂
        Par f₁ f₂ -> mappend <$> go f₁ <*> go f₂
        Effect eff -> visit eff

evalA :: forall eff arr a₀ b₀. (Arrow arr) => (forall a b. eff a b -> arr a b) -> FreeA eff a₀ b₀ -> arr a₀ b₀
evalA exec = go
  where
    go :: forall a b. FreeA eff a b -> arr a b
    go freeA = case freeA of
        Pure f -> arr f
        Seq f₁ f₂ -> go f₂ . go f₁
        Par f₁ f₂ -> go f₁ *** go f₂
        Effect eff -> exec eff

effect :: eff a b -> FreeA eff a b
effect = Effect

instance Category (FreeA eff) where
    id = Pure id
    (.) = flip Seq

instance Arrow (FreeA eff) where
    arr = Pure
    first f = Par f id
    second f = Par id f
    (***) = Par

data ConsoleOp a b where
  GetLine :: ConsoleOp (IO ()) (IO String)
  PutLine :: ConsoleOp (IO String) (IO ())
  Prompt :: String -> ConsoleOp (IO ()) (IO ())
  Dictionary :: Map String String -> ConsoleOp (IO String) (IO String)

instance Show (ConsoleOp a b) where
  show GetLine = "GetLine"
  show PutLine = "PutLine"
  show (Prompt message) = "Prompt \"" ++ message ++ "\""
  show (Dictionary dict) = "Dictionary " ++ show (Map.toList dict)

getLine' = effect GetLine
putLine = effect PutLine
prompt s = effect $ Prompt s
dictionary d = effect $ Dictionary d

translator =
    prompt "Hello"
    >>> prompt "Enter an English word to translate"
    >>> getLine'
    >>> dictionary (fromList [
        ("apple", "manzana"),
        ("blue", "azul"),
        ("hello", "hola"),
        ("goodbye", "adios")
    ])
    >>> putLine

showProg :: ConsoleOp a b -> () -> String
showProg GetLine = \_ -> show GetLine <> ", "
showProg PutLine = \_ -> show PutLine <> ", "
showProg (Prompt m) = \_ -> show (Prompt m) <> ", "
showProg (Dictionary d) = \_ -> show (Dictionary d) <> ", "

numGets :: ConsoleOp a b -> () -> Sum Int
numGets GetLine = \_ -> 1
numGets _ = \_ -> 0

interp :: ConsoleOp a b -> a -> b
interp GetLine = \io -> io >> getLine
interp PutLine = \io -> io >>= putStrLn
interp (Prompt m) = \io -> io >> putStrLn m
interp (Dictionary m) = \io -> io >>= \s -> return (Map.findWithDefault "I don't know that one" s m)

showProg' = analyze showProg translator
numGets' = analyze numGets translator
interp' = evalA interp translator

main = print $ (+2) &&& (*2) >>> (uncurry(/)) $ 7
