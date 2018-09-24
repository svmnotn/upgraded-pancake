# upgraded-pancake
[![Travis Build Status](https://travis-ci.com/svmnotn/upgraded-pancake.svg?branch=develop)](https://travis-ci.com/svmnotn/upgraded-pancake)
[![Appveyor Build status](https://ci.appveyor.com/api/projects/status/q0apgvx7rbcu79y3?svg=true)](https://ci.appveyor.com/project/svmnotn/upgraded-pancake)

`upgraded-pancake` is a library for creating tables that can be rolled on. These are commonly found in Tabletop Role Playing games. The project also includes a server and website that use the library to create a simple web app to manipulate and roll on user created tables.

## Requirements
* For the library: The Rust language and Cargo, this easily obtained via [rustup](https://rustup.rs).
* For the server: you need to be able to build the library, and also the latest nightly build of the rust compiler.
* For the web app: you need to build the server, and install `npm` as well as `react-scripts`.

## Getting Started
To build the web app, you will need to:
1. Clone the repo
    * Via `git clone https://github.com/svmnotn/upgraded-pancake.git`
    * The clone button at the top if using Github Desktop.
2. Inside the `front-end` directory, run `npm install react-scripts` if you don't have `react-scripts` already installed
3. Then run `npm run build`
4. Go to the parent directory and run `cargo server`
5. Open your prefered browser and go to `localhost:8000`

## Goal & Status
The goal of Upgraded Pancakes is to make a webapp that allows for easy creation of rollable tables. The current development status is sporadic as both main contributors have Life(tm) to do.

## Contributing
We welcome new contributors looking to clean up the rust code, implement new quality of life features in the webapp, or something else. However it is best to create an issue first to discuss the best way to go about creating what you want to add.

## License
This project is licensed under [MIT](https://tldrlegal.com/license/mit-license), see the [LICENSE](LICENSE) file for details.
