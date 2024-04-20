# JQ-clone

## Motivation

I've recently learned about JQ and its ability to process and manipulate JSON data. So to help myself understand the core concepts I decided to create a simplified version.

## Features

Here are some of the functionalty that I reproduced:

- **JSON Parsing**: Mini-JQ can parse JSON data and represent it in a structured format.
- **Filtering**: You can apply filters to select specific parts of JSON data.
- **Basic Operators**: Mini-JQ supports basic operators for data manipulation.
- **Output Formatting**: You can format the output in a readable manner.

## Getting Started

To get started with Mini-JQ, follow these steps:

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/your_username/mini-jq.git
   ```

2. Navigate to the project directory:

   ```bash
   cd mini-jq
   ```

3. Compile the source code:

   ```bash
   make
   ```

4. Run Mini-JQ with your JSON data:

   ```bash
   ./mini-jq 'filter_expression' input.json
   ```

Replace `'filter_expression'` with your desired filter expression and `input.json` with your JSON file.

## Examples

Here are some examples of how you can use Mini-JQ:

- To select all elements from a JSON array:

  ```bash
  ./mini-jq '.[]' data.json
  ```

- To select specific fields from JSON objects:

  ```bash
  ./mini-jq '{ name, age }' data.json
  ```

- To apply arithmetic operations:
  ```bash
  ./mini-jq '.age + 5' data.json
  ```

## Contributing

Contributions to Mini-JQ are welcome! If you have any ideas for improvements or new features, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

Mini-JQ wouldn't have been possible without the inspiration from the JQ project and the contributions of its community. Special thanks to the creators and maintainers of JQ for their incredible work.

---

Happy coding with Mini-JQ! 🚀
