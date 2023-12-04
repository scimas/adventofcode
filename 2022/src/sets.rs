pub trait Membership<T> {
    fn contains(&self, element: &T) -> bool;
}

pub trait Intersection<Rhs = Self> {
    type Output;

    fn intersect(self, rhs: Rhs) -> Self::Output;
}

pub trait Difference<Rhs = Self> {
    type Output;

    fn diff(self, rhs: Rhs) -> Self::Output;
}

pub trait Union<Rhs = Self> {
    type Output;

    fn union(self, rhs: Rhs) -> Self::Output;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Bound<T> {
    Open(T),
    Closed(T),
}

impl<T> Bound<T> {
    fn value(&self) -> &T {
        match self {
            Bound::Open(b) => b,
            Bound::Closed(b) => b,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Interval<T> {
    lower: Bound<T>,
    upper: Bound<T>,
}

impl<T: PartialOrd> Membership<T> for Interval<T> {
    fn contains(&self, element: &T) -> bool {
        if element > self.lower.value() && element < self.upper.value() {
            true
        } else if element < self.lower.value() || element > self.upper.value() {
            false
        } else if element == self.lower.value() {
            matches!(self.lower, Bound::Closed(_))
        } else if element == self.upper.value() {
            matches!(self.upper, Bound::Closed(_))
        } else {
            false
        }
    }
}

fn order_lower_bounds<T: PartialOrd>(
    lower_bound_1: Bound<T>,
    lower_bound_2: Bound<T>,
) -> (Bound<T>, Bound<T>) {
    match (&lower_bound_1, &lower_bound_2) {
        (Bound::Open(sl), Bound::Open(rl)) => {
            if sl <= rl {
                (lower_bound_1, lower_bound_2)
            } else {
                (lower_bound_2, lower_bound_1)
            }
        }
        (Bound::Closed(sl), Bound::Closed(rl)) => {
            if sl <= rl {
                (lower_bound_1, lower_bound_2)
            } else {
                (lower_bound_2, lower_bound_1)
            }
        }
        (Bound::Open(sl), Bound::Closed(rl)) => {
            if sl == rl {
                (lower_bound_2, lower_bound_1)
            } else if sl < rl {
                (lower_bound_1, lower_bound_2)
            } else {
                (lower_bound_2, lower_bound_1)
            }
        }
        (Bound::Closed(sl), Bound::Open(rl)) => {
            if sl <= rl {
                (lower_bound_1, lower_bound_2)
            } else {
                (lower_bound_2, lower_bound_1)
            }
        }
    }
}

fn order_upper_bounds<T: PartialOrd>(
    upper_bound_1: Bound<T>,
    upper_bound_2: Bound<T>,
) -> (Bound<T>, Bound<T>) {
    match (&upper_bound_1, &upper_bound_2) {
        (Bound::Open(su), Bound::Open(ru)) => {
            if su <= ru {
                (upper_bound_1, upper_bound_2)
            } else {
                (upper_bound_2, upper_bound_1)
            }
        }
        (Bound::Closed(su), Bound::Closed(ru)) => {
            if su <= ru {
                (upper_bound_1, upper_bound_2)
            } else {
                (upper_bound_2, upper_bound_1)
            }
        }
        (Bound::Open(su), Bound::Closed(ru)) => {
            if su <= ru {
                (upper_bound_1, upper_bound_2)
            } else {
                (upper_bound_2, upper_bound_1)
            }
        }
        (Bound::Closed(su), Bound::Open(ru)) => {
            if su == ru {
                (upper_bound_2, upper_bound_1)
            } else if su < ru {
                (upper_bound_1, upper_bound_2)
            } else {
                (upper_bound_2, upper_bound_1)
            }
        }
    }
}

impl<T: PartialOrd> Intersection for Interval<T> {
    type Output = Set<T>;

    fn intersect(self, rhs: Self) -> Self::Output {
        if (self.lower.value() == rhs.lower.value() || self.upper.value() == rhs.upper.value())
            || (self.contains(rhs.lower.value()) || self.contains(rhs.upper.value()))
        {
            let ordered_lower_bounds = order_lower_bounds(self.lower, rhs.lower);
            let ordered_upper_bounds = order_upper_bounds(self.upper, rhs.upper);
            Set::Interval(Interval {
                lower: ordered_lower_bounds.1,
                upper: ordered_upper_bounds.0,
            })
        } else if self.lower.value() == rhs.upper.value() {
            match (&self.lower, &rhs.upper) {
                (Bound::Open(_), Bound::Open(_))
                | (Bound::Open(_), Bound::Closed(_))
                | (Bound::Closed(_), Bound::Open(_)) => Set::Empty,
                (Bound::Closed(_), Bound::Closed(_)) => Set::Interval(Interval {
                    lower: self.lower,
                    upper: rhs.upper,
                }),
            }
        } else if self.upper.value() == rhs.lower.value() {
            match (&self.upper, &rhs.lower) {
                (Bound::Open(_), Bound::Open(_))
                | (Bound::Open(_), Bound::Closed(_))
                | (Bound::Closed(_), Bound::Open(_)) => Set::Empty,
                (Bound::Closed(_), Bound::Closed(_)) => Set::Interval(Interval {
                    lower: self.upper,
                    upper: rhs.lower,
                }),
            }
        } else {
            Set::Empty
        }
    }
}

impl<T: PartialOrd + Clone> Difference for Interval<T> {
    type Output = Set<T>;

    fn diff(self, rhs: Self) -> Self::Output {
        let intersection = self.clone().intersect(rhs);
        match intersection {
            Set::Empty => Set::Interval(self),
            Set::Interval(int) => {
                let ordered_lower_bounds = order_lower_bounds(self.lower, int.lower);
                let ordered_upper_bounds = order_upper_bounds(self.upper, int.upper);
                let int1 = Set::Interval(Interval { lower: ordered_lower_bounds.0, upper: ordered_lower_bounds.1 });
                let int2 = Set::Interval(Interval { lower: ordered_upper_bounds.0, upper: ordered_upper_bounds.1 });
                Set::Union(Box::new(int1), Box::new(int2))
            },
            Set::Union(_, _) => unreachable!("an interval's intersection with another intersection cannot be a set union - by construction"),
        }
    }
}

impl<T: PartialOrd + Clone> Union for Interval<T> {
    type Output = Set<T>;

    fn union(self, rhs: Self) -> Self::Output {
        let intersection = self.clone().intersect(rhs.clone());
        match intersection {
            Set::Empty => {
                Set::Union(
                    Box::new(Set::Interval(self)),
                    Box::new(Set::Interval(rhs)),
                )
            },
            Set::Interval(_) => {
                let ordered_lower_bounds = order_lower_bounds(self.lower, rhs.lower);
                let ordered_upper_bounds = order_upper_bounds(self.upper, rhs.upper);
                Set::Interval(Interval { lower: ordered_lower_bounds.0, upper: ordered_upper_bounds.1 })
            },
            Set::Union(_, _) => unreachable!("an interval's intersection with another intersection cannot be a set union - by construction"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Set<T> {
    Empty,
    Interval(Interval<T>),
    Union(Box<Set<T>>, Box<Set<T>>),
}

impl<T: PartialOrd> Membership<T> for Set<T> {
    fn contains(&self, element: &T) -> bool {
        match self {
            Set::Empty => false,
            Set::Interval(int) => int.contains(element),
            Set::Union(set1, set2) => set1.contains(element) || set2.contains(element),
        }
    }
}

impl<T: PartialOrd + Clone> Intersection for Set<T> {
    type Output = Self;

    fn intersect(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Set::Empty, _) | (_, Set::Empty) => Set::Empty,
            (Set::Interval(s_int), Set::Interval(r_int)) => s_int.intersect(r_int),
            (s_set @ Set::Interval(_), Set::Union(r_set1, r_set2)) => s_set
                .clone()
                .intersect(*r_set1)
                .union(s_set.intersect(*r_set2)),
            (Set::Union(s_set1, s_set2), r_set @ Set::Interval(_)) => s_set1
                .intersect(r_set.clone())
                .union(s_set2.intersect(r_set)),
            (Set::Union(s_set1, s_set2), Set::Union(r_set1, r_set2)) => s_set1
                .clone()
                .intersect(*r_set1.clone())
                .union(s_set1.intersect(*r_set2.clone()))
                .union(
                    s_set2
                        .clone()
                        .intersect(*r_set1)
                        .union(s_set2.intersect(*r_set2)),
                ),
        }
    }
}

impl<T: PartialOrd + Clone> Difference for Set<T> {
    type Output = Self;

    fn diff(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Set::Empty, _) => Set::Empty,
            (s_set, Set::Empty) => s_set,
            (Set::Interval(s_int), Set::Interval(r_int)) => s_int.diff(r_int),
            (s_set @ Set::Interval(_), Set::Union(r_set1, r_set2)) => {
                s_set.diff(*r_set1).diff(*r_set2)
            }
            (Set::Union(s_set1, s_set2), r_set @ Set::Interval(_)) => {
                s_set1.diff(r_set.clone()).union(s_set2.diff(r_set))
            }
            (Set::Union(s_set1, s_set2), Set::Union(r_set1, r_set2)) => s_set1
                .clone()
                .diff(*r_set1.clone())
                .union(s_set1.diff(*r_set2.clone()))
                .union(s_set2.clone().diff(*r_set1).union(s_set2.diff(*r_set2))),
        }
    }
}

impl<T: PartialOrd + Clone> Union for Set<T> {
    type Output = Self;

    fn union(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Set::Empty, r_set) => r_set,
            (s_set, Set::Empty) => s_set,
            (Set::Interval(s_int), Set::Interval(r_int)) => s_int.union(r_int),
            (s_set @ Set::Interval(_), Set::Union(r_set1, r_set2)) => Set::Union(
                Box::new(s_set.clone().union(*r_set1)),
                Box::new(s_set.union(*r_set2)),
            ),
            (Set::Union(s_set1, s_set2), r_set @ Set::Interval(_)) => Set::Union(
                Box::new(s_set1.union(r_set.clone())),
                Box::new(s_set2.union(r_set)),
            ),
            (Set::Union(s_set1, s_set2), Set::Union(r_set1, r_set2)) => Set::Union(
                Box::new(Set::Union(
                    Box::new(s_set1.clone().union(*r_set1.clone())),
                    Box::new(s_set1.union(*r_set2.clone())),
                )),
                Box::new(Set::Union(
                    Box::new(s_set2.clone().union(*r_set1)),
                    Box::new(s_set2.union(*r_set2)),
                )),
            ),
        }
    }
}
