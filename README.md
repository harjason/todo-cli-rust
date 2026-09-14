# A minimal markdown-based todo list in terminal for Arch Linux
## Installation
1. Clone the repo 
```bash
git clone https://github.com/harjason/todo-cli-rust
cd todo-cli-rust
```
2. Install using Cargo
```bash
cargo install --path .
```
3. Verify installation
```bash
todo --version
```
## How It Works
`todo` operates locally to your current working directory
* **Isolated Lists:** when `todo` is run, a local configuration file `.current` is created only in the directory you are currently working in (not globally).
* **No Global Spillover:** todos created in `~/x/y` are completely isolated from those created in  `~/x/z` or `~/a`.
* **Subdirectory independence** : Using `todo` in a subdirectory creates a brand new set of lists which will not share or overwrite todos from the parent directory.  
## Usage
* Create a todo list
```bash
todo create-list [list name]
```
* Delete todo list(s)
```bash
todo delete-list
```
* Clear all todos in  todo list(s)
```bash
todo clear-list
```
* Show names of all todo lists
```bash
todo lists
```
* Show name of selected todo list
```bash
todo list
```
* Show todos in selected todo list
```bash
todo show
```
* Select a todo list
```bash
todo set [list name]
```
* Add todo(s) to selected list
```bash
todo add "todo one" "todo two" "todo three"
```
* Complete todo(s)
```bash
todo complete
```
## Examples
* Create  todo lists in your `Home` directory
```bash
cd
todo create-list dev
todo create-list daily
todo set dev
todo add "work on modals" "set up ssh"
todo show
```
* Create a todo list in `Projects` located in `Home`
```bash
cd
cd Projects
todo create-list project-list
todo lists
```