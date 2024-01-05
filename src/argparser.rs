use std::{env, fmt::Debug, str::FromStr};

pub fn get_parameter_switch_from_args(parameter: &str) -> bool {
    env::args().any(|x| x == parameter)
}

//  Gets the value of a parameter from the command line arguments
//  splits the arguments into a vector and then finds the index of the parameter
//  if the parameter is found then the next value is returned
pub fn get_parameter_value_from_args<T: std::str::FromStr>(parameter: &str, errormessage: &str) -> Result<T, String> where <T as FromStr>::Err: Debug {
    if !get_parameter_switch_from_args(parameter) {
        return Err(format!("Parameter not found {}", &parameter));
    }

    let args = env::args().collect::<Vec<String>>();
    let index = args.iter().position(|x| x == parameter);

    if index.is_none() {
        return Err(format!("Parameter not found {}", &parameter));
    }

    if index.unwrap() + 1 >= env::args().count() {
        return Err(format!("Parameter passed but value not found {}", &parameter));
    }

    let parametervalue = args.get(index.unwrap() + 1);
    match parametervalue {
        Some(x) => match x.parse::<T>(){
            Ok(x) => return Ok(x),
            Err(_) => return Err(format!("{}", errormessage.to_string())),
        },
        None => return Err(format!("Parameter value not found {}", &parameter)),

    }
}
