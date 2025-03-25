# Examples

This folder contains several examples of how to utilize `envoke`

</br>

Below is a table of the examples you can find in this folder
| Name                                                    | Description                                                                                                                                                                          |
| ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [loading_envs](./loading_envs/)                         | How to load environment variables into struct fields                                                                                                                                 |
| [using_enums](./using_enums/)                           | How to use enums as a field type                                                                                                                                                     |
| [applying_defaults](./applying_defaults/)               | How to use the default attribute as a fallback                                                                                                                                       |
| [renaming_envs](./renaming_envs/)                       | How to modify the name of environment variables to look for                                                                                                                          |
| [parsing_envs](./parsing_envs/)                         | How to add a custom parser which the value is ran through before assignment                                                                                                          |
| [validating_envs](./parsing_envs/)                      | How to validate the loaded values before and after parsing to ensure they are as you expect                                                                                          |
| [nesting_structs](./nesting_structs/)                   | How to nest multiple structs together for easier management                                                                                                                          |
| [loading_map_and_set_envs](./loading_map_and_set_envs/) | How to load map or set environment variables, e.g., load a vector into a vec field                                                                                                   |
| [conditional_load](./conditional_load/)                 | How to load different structs based on a single environment variable                                                                                                                 |
| [from_dotenv](./from_dotenv/)                           | How to load environment variables from a dotenv file. Note that this example has to been ran from inside of the `from_dotenv` example folder due to the location of the dotenv file. |

</br>

#### Disclaimer

<sup>
There might not be an example for every use case. As more features are added and/or refined the examples will be added and updated
</sup>

</br>

<sub>
To have actual environment variables to test with the crate <a href="https://crates.io/crates/temp-env">tmp-env</a> is used. In an actual scenario this is not necessary as the environment variables will be found on your processe's environment
</sub>