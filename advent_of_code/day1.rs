fn main(){
    // haven't included all of them
    let input: [i16; 10] = [199,
    200,
    208,
    210,
    200,
    207,
    240,
    269,
    260,
    263
    ];

    // part 1
    let mut number_of_increased_depth_measurement: u8 = 0;

    for (i, _x) in input.iter().enumerate() {
        if i > 0 {
            if (input[i] - input[i - 1]) > 0 {
                number_of_increased_depth_measurement = number_of_increased_depth_measurement + 1;
            };
        };
    };

    println!("Total Number of increased depth measurement is {number_of_increased_depth_measurement}");

    // part 2
    let mut smoothen_input:Vec<i16> = Vec::new();

    for (i, _x) in input.iter().enumerate() {
        if i < (input.len() - 2) {
            smoothen_input.push(input[i] + input[i + 1] + input[i + 2]);
        }
    };
    println!("{:?}", smoothen_input);

    let mut smoothen_results: u8 = 0;

    for (i, _x) in smoothen_input.iter().enumerate() {
        if i > 0 {
            if (smoothen_input[i] - smoothen_input[i - 1]) > 0 {
                smoothen_results = smoothen_results + 1;
            };
        };
    };

    println!("Total Number of increased smoothen depth measurement is {smoothen_results}");
}