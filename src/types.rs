use std::cmp::Ordering;
use std::ops;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Width(pub u8);
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Column<T = u8>(pub T);

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Height(pub u8);
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Row<T = u8>(pub T);

#[derive(Copy, Clone, PartialEq)]
pub struct Size(pub Width, pub Height);
#[derive(Copy, Clone, PartialEq)]
pub struct Position<T = u8>(pub Column<T>, pub Row<T>);


#[derive(Copy, Clone, PartialEq)]
pub struct Index(pub usize);

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct NumberOfColors(pub u8);

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct NumberOfClicks(pub u16);


impl PartialEq<Column> for Width {
    #[inline]
    fn eq(&self, other: &Column) -> bool {
        self.0 == other.0
    }
}
impl PartialEq<Width> for Column {
    #[inline]
    fn eq(&self, other: &Width) -> bool {
        self.0 == other.0
    }
}
impl PartialOrd<Column> for Width {
    #[inline]
    fn partial_cmp(&self, other: &Column) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &other.0)
    }
}
impl PartialOrd<Width> for Column {
    #[inline]
    fn partial_cmp(&self, other: &Width) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

impl PartialEq<Row> for Height {
    #[inline]
    fn eq(&self, other: &Row) -> bool {
        self.0 == other.0
    }
}
impl PartialEq<Height> for Row {
    #[inline]
    fn eq(&self, other: &Height) -> bool {
        self.0 == other.0
    }
}
impl PartialOrd<Row> for Height {
    #[inline]
    fn partial_cmp(&self, other: &Row) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &other.0)
    }
}
impl PartialOrd<Height> for Row {
    #[inline]
    fn partial_cmp(&self, other: &Height) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.0, &other.0)
    }
}

impl<T> From<T> for Column<T> {
    #[inline]
    fn from(x: T) -> Column<T> {
        Column(x)
    }
}
impl<T> From<T> for Row<T> {
    #[inline]
    fn from(x: T) -> Row<T> {
        Row(x)
    }
}
impl From<(f64, f64)> for Position<f64> {
    #[inline]
    fn from((x, y): (f64, f64)) -> Self {
        Position(Column(x), Row(y))
    }
}



impl<T: ops::Sub<Output = T>> ops::Sub for Column<T> {
    type Output = Column<T>;
    fn sub(self, other: Self) -> Self::Output {
        Column(self.0 - other.0)
    }
}
impl<T: ops::Sub<Output = T>> ops::Sub for Row<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Row(self.0 - other.0)
    }
}
impl<T: ops::Sub<Output = T>> ops::Sub for Position<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Position(self.0 - other.0, self.1 - other.1)
    }
}

impl ops::Mul<f64> for Column<f64> {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Column(self.0 * other)
    }
}
impl ops::Mul<f64> for Row<f64> {
    type Output = Self;
    fn mul(self, other: f64) -> Self::Output {
        Row(self.0 * other)
    }
}
