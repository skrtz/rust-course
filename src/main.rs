#![allow(unused_variables)] //applies to the whole file because of the exclamation point and being places at the top line
type Meters = i32;

//#[allow(unused_variables)] applies to the whole fn
fn main() {
    //#[allow(unused_variables)] only applies to the following line of code
    let mile_race_meters: Meters = 1600;
    let two_mile_race_meters: Meters = 3200;
}


/*
COMPILER DIRECTIVES ARE ANNOTATIONS WE ADD TO OUR CODE THAT TELLS COMPILER HOW TO THE PARSE ARE FILE.

IN THIS SECTION WE USE THE BASIC ALLOW DIRECTIVE BUT WILL COVER MORE IN FOLLOWING CHAPTERS - THEY ALL FOLLOW THE SAME PATTERN.

THE BOOK CALLS THESE "ATTRIBUTES" RATHER THAN COMPILER DIRECTIVES.
 */