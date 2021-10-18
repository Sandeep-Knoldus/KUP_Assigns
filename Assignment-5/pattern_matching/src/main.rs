#[derive(PartialEq, Eq, Debug)]
///enum Coordinate
///
/// #variant
///
/// Abscissa:-Abscissa is variant of enum Coordinate and associated with integer type
///
/// Ordinate:-Ordinate is variant of enum Coordinate and associated with integer type
pub enum Coordinate {
    Abscissa(i32),
    Ordinate(i32),
}

///enum Position which used to describe the Position of Quadrant
///
/// #variant
///
/// First:- First is variant of enum Position and associated with Coordinate type
///
/// Second:- Second is variant of enum Position and associated with Coordinate type
///
/// Third:-  Third is variant of enum Position and associated with Coordinate type
///
/// Fourth: Fourth is variant of enum Position and associated with Coordinate type
///
/// XAxis: XAxis is variant of enum Position and associated with Coordinate type
///
/// YAxis: YAxis is variant of enum Position and associated with Coordinate type
///
/// Origin: Origin is variant of enum Position and associated with Coordinate type
pub enum Position {
    First(Coordinate, Coordinate),
    Second(Coordinate, Coordinate),
    Third(Coordinate, Coordinate),
    Fourth(Coordinate, Coordinate),
    XAxis(Coordinate, Coordinate),
    YAxis(Coordinate, Coordinate),
    Origin(Coordinate, Coordinate),
}

/// Function 'check_coordinate' is used check the Quadrant of the given point
///
/// #Arguments
///
/// point: A point is tuple object of integer type
///
/// #Return
///
/// Returns the Position lied point
pub fn check_coordinate((point_1, point_2): (i32, i32)) {
    match (point_1, point_2) {
        (point_1, point_2) if point_1 > 0 && point_2 > 0 => println!(
            "Position::First(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 < 0 && point_2 > 0 => println!(
            "Position::First(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 < 0 && point_2 < 0 => println!(
            "Position::Third(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 > 0 && point_2 < 0 => println!(
            "Position::Fourth(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 == 0 && point_2 != 0 => println!(
            "Position::YAxis(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 != 0 && point_2 == 0 => println!(
            "Position::XAxis(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        (point_1, point_2) if point_1 == 0 && point_2 == 0 => println!(
            "Position::Origin(Coordinate::Abscissa({}),Coordinate::Ordinate({}))",
            point_1, point_2
        ),
        _ => println!("Wrong input"),
    }
}

/// Function main
fn main() {
    // initializing point 1
    let first_point = (-2, -2);

    // initializing point 2
    let second_point = (0, 0);

    // calling 'check_coordinate' function with variable 'first_point'
    check_coordinate(first_point);

    // calling 'check_coordinate' function with variable 'second_point'
    check_coordinate(second_point);
}
