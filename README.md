# Introduction

This project is a graphical **visualization tool** for **maze generation** and **maze-solving** algorithms, built using the [Bevy](https://bevyengine.org/) game engine. It enables users to **generate random mazes**, **save generated mazes**, and **solve mazes** based on specified start and end points.



# Project features

- **Interactive User Interface**: Provides a dynamic, user-friendly environment for interacting with the maze generation and solving processes.
- **Real-time Simulation**: Visualizes maze generation and solving algorithms in real-time, allowing users to observe the step-by-step execution of the algorithms.



# How does it work ?

To **generate** and **solve** mazes, project uses concepts of [Graph theory](https://en.wikipedia.org/wiki/Graph_theory). 

## 1. Maze generation - Recursive backtracking with explicit stack

The maze grid is represented as a **graph**, where each **cell in the grid** is treated as a **vertex**, and **connections between adjacent cells** act as **edges**.

For generating the maze, we utilize the **Recursive Backtracking Algorithm**. Similar to the traditional [Depth-First Search (DFS)](https://en.wikipedia.org/wiki/Depth-first_search) algorithm, we use a **stack** to manage traversal. However, instead of pushing **all neighboring cells** onto the stack, we push only one **randomly chosen neighbor**. Additionally, we intentionally push the **parent cell** back onto the **stack** to allow the algorithm to revisit it and select another **unvisited neighbor**, effectively mimicking recursive backtracking in an iterative manner.

As the algorithm progresses, the **maze is constructed by removing walls between the current cell and the newly visited neighbor**, carving a path through the grid. This ensures that all cells remain connected, forming a **perfect maze** with no loops or isolated sections.



## 2. Maze solving - Breadth-first search

The maze is represented as a **graph**, where each **cell in the grid** represents a **vertex**, and edges exist only between adjacent cells that are **not separated** by a wall.

To solve the maze, we utilize the [Breadth-First Search (BFS)](https://en.wikipedia.org/wiki/Breadth-first_search) algorithm, which uses a **queue** instead of a **stack** to manage traversal. At each step, all **neighboring cells** are added to the **queue**, and the algorithm terminates as soon as the **target cell** is reached.

While BFS is not as fast as [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) due to its lack of [heuristic guidance](https://en.wikipedia.org/wiki/Heuristic_(computer_science)), it has lower memory requirements and is simpler to implement. Additionally, BFS guarantees the **shortest path** in an **unweighted graph**, making it a reliable choice for solving mazes where all moves have **equal cost**.



# Reason for choosing Bevy for this project

I chose [Bevy](https://bevyengine.org/) because I wanted to experiment with the [Entity-Component-System (ECS)](https://en.wikipedia.org/wiki/Entity_component_system) paradigm. In Bevy, every part of the application can be represented as a **component**, and relevant components are associated with **entities**. **Systems** are then used to process and modify these components, creating a highly modular and scalable structure.

One of the features that caught my attention was Bevy’s **query system**, which allows systems to efficiently retrieve and operate on specific components.  This approach reminded me of database queries in back-end development, where queries are used to fetch only the relevant data needed for a specific operation. The similarity between ECS-based development and back-end architecture made me curious to explore how these concepts could be applied to a graphical, real-time application for **maze generation and solving** visualization.

Additionally, Bevy’s **parallel execution model** and Rust’s **performance benefits** made it an appealing choice for handling procedural maze generation and path-finding efficiently.,especially when integrating real-time visualization.





# Project usage

### Running project from pre-compiled binary

1. Navigate to the [Releases](https://github.com/Filiup/bevy-path-finding/releases) page.
2. Download the appropriate binary for your operating system.
3. Run the downloaded binary by double-clicking it or executing it from the terminal.



### Running project from source

### Prerequisites

Before running the project, ensure you have the following installed:

- [Rust toolchain](https://rustup.rs/)

1. Clone the repository to your preferred location.
2. Open a terminal, navigate to the project directory, and execute the following command: `cargo run`

# Future plans

- Add support for [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) algorithm.
- Add functionality to compare the performance of [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) and [BFS](https://en.wikipedia.org/wiki/Breadth-first_search) in terms of speed.

