# gpiotacticalbutton

A rust program that polls a GPIO button for button press. Once the button is pressed it will output text and execute a local program of your choice.
This program is preconfigured to execute the pushtotext rust program that sends a text message.

## Getting Started:

### INSTALL RUST;
- Install rust using the steps for your operating system here: https://www.rust-lang.org/tools/install
- Test your install by running; 'rustc --version'

### CREATE YOUR RUST PROJECTS FOLDER;
- Create a new folder for your awesome rust projects; 'mkdir projects'
- Change directory to your projects folder; 'cd projects'
- (optional) make sure you are in the right folder; 'pwd'

### CREATE YOUR RUST PROJECT;
- Create a new rust application by running; 'cargo new APPLICATION_NAME'
- Change directory to your new project; 'cd APPLICATION_NAME'

### UPDATE THE PROJECT WITH YOUR INFO;
- Replace the main.rs and toml file dependencies with the ones in this repository.

## RUN THE CODE;
- From terminal run the following command with *pin* being the pin number your want to poll; 'cargo run *pin*'

##SEE IF IT WORKS;
You should see the program start and print aline saying the button is set to "High". When you press the button the terminal will print a line saying "Low" which indicates the button was pressed. If you have placed the pre-compiled pushtotext code into the same folder then it will run the command to start that program as well.

