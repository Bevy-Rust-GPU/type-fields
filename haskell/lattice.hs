import Algebra.Lattice

instance Lattice Int where
    a \/ b = a `max` b
    a /\ b = a `min` b

instance BoundedMeetSemiLattice Int where
    top = maxBound

instance BoundedJoinSemiLattice Int where
    bottom = minBound

instance Lattice Float where
    a \/ b = a `max` b
    a /\ b = a `min` b

instance BoundedMeetSemiLattice Float where
    top = 1.0 / 0.0

instance BoundedJoinSemiLattice Float where
    bottom = -1.0 / 0.0

instance Lattice Double where
    a \/ b = a `max` b
    a /\ b = a `min` b

instance BoundedMeetSemiLattice Double where
    top = 1.0 / 0.0

instance BoundedJoinSemiLattice Double where
    bottom = -1.0 / 0.0

neg a = -a

a |- b = a \/ neg b
