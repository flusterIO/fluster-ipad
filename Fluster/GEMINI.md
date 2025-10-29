# Fluster Project Overview

This document provides a high-level overview of the Fluster project, its structure, and development conventions.

## Project Overview

Fluster is a SwiftUI application for macOS. Based on the file structure and code, it appears to be a note-taking application with a focus on markdown editing. The application utilizes the coordinator pattern for navigation and SwiftData for data persistence.

### Key Technologies

*   **UI Framework:** SwiftUI
*   **Data Persistence:** SwiftData
*   **Navigation:** Coordinator Pattern
*   **Language:** Swift

### Architecture

The project is structured into `Core` and `Features` directories.

*   **`Core`:** This directory likely contains the core components of the application, such as models and views that are shared across multiple features.
*   **`Features`:** This directory contains the different features of the application, such as `bibliography`, `dashboard`, `markdown`, `navigation`, and `search`. Each feature is self-contained in its own directory, which makes the codebase modular and easy to maintain.

## Building and Running

To build and run the project, you will need Xcode installed on your macOS device.

1.  Clone the repository.
2.  Open the `Fluster.xcodeproj` file in Xcode.
3.  Select a target device and click the "Run" button.

*TODO: Add any specific build commands or configurations if necessary.*

## Development Conventions

### Coding Style

The codebase follows the standard Swift and SwiftUI coding conventions. The code is well-structured and easy to read.

### Testing

There are no tests in the project at the moment. It is recommended to add unit and UI tests to ensure the quality of the codebase.

*TODO: Add information about the testing strategy and frameworks to be used.*
