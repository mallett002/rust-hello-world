# rust-hello-world
## High-level roadmap to build your to-do list application without diving straight into the code:

### Step 1: Project Setup
- **Create a New Project**: Use `cargo new todo_list` to create a new Rust project.
- **Navigate to Project Directory**: Use `cd todo_list` to go into your project folder.

### Step 2: Outline Basic Functionality
- **Add a Task**: Prompt the user to enter a task and store it in a list.
- **List Tasks**: Display all tasks currently in the list.
- **Save Tasks**: Write the list of tasks to a file (`todo_list.txt`) when the program exits.
- **Load Tasks**: Read the tasks from the file when the program starts.

### Step 3: Implement the Main Program Loop
- **Menu Display**: Show options (Add a task, List tasks, Exit).
- **User Input**: Capture user selection and perform the corresponding action.

### Step 4: Handle File Operations
- **Load Tasks**: Check if the file exists, if so, read tasks line by line and store them in your list.
- **Save Tasks**: Write each task in the list to the file when the program exits.

### Step 5: Putting it All Together
Combine the functionality into a cohesive program flow:
- Initialize an empty task list.
- Load existing tasks from the file into the list.
- Enter a loop to display the menu and capture user input.
- Execute the chosen action (add, list, or exit).
- When exiting, save the current list of tasks to the file.

### Tips for Implementation
- **User Input**: Use Rust’s standard library to handle input and output (`std::io`).
- **Vector for Tasks**: Use a `Vec<String>` to store tasks dynamically.
- **File Handling**: Use `std::fs` for reading from and writing to files.

### Bonus: Enhancements
Once you have the basic application working, you can consider adding additional features:
- **Task Completion**: Mark tasks as completed.
- **Task Deletion**: Remove tasks from the list.
- **Task Priorities**: Assign and sort tasks based on priority.
