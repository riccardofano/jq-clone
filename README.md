# JQ-clone

## Motivation

I've recently learned about JQ and its ability to process and manipulate JSON data. So to help myself understand the core concepts I decided to create a simplified version.

## Features

Here are some of the functionalty that I reproduced:

-   **JSON Parsing**: Mini-JQ can parse JSON data and represent it in a structured format.
-   **Filtering**: You can apply filters to select specific parts of JSON data.
-   **Output Formatting**: You can format the output in a readable manner.

## Getting Started

1. Clone this repository to your local machine:

    ```bash
    git clone https://github.com/riccardofano/jq-clone.git
    ```

2. Navigate to the project directory:

    ```bash
    cd jq-clone
    ```

3. Run the program with your JSON data:

    ```bash
    echo {"hello": "world"} | cargo r '[.hello]'
    ```
