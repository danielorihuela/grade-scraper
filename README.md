# grade-scraper

Toy project to learn a bit of web scraping and Rust.

The code extracts the grades from a course on `https://www.talent.upc.edu/`.
This has been created for personal use and has only been tested for an account enrolled to only one course.


## Instructions

1. [Install rust](https://www.rust-lang.org/tools/install).
2. Build the project. Execute the following command in the root folder of this repository.
   A warning will be printed to the screen. I follow a positive strategy, no errors will be thrown so I do not handle them.
  ``` bash
  > cargo build --release
  ```
3. (Optional) Create an alias to execute the program from anywhere in your system.
    You will need to modify the path to point to the executable in your system.
```bash
#In linux you can do that by
> alias notes="export PATH=$PATH:path-to-repository-parent-folder/grade_scraper/target/release/ && grade_scraper"
```
4. Run the executable by executing the alias or using cargo.
```bash
#  in the root folder of the repository
> cargo run
# or anywhere in the system
> notes
```

## Images

The expected output can be seen in the following image.

![alt text](https://github.com/danielorihuela/grade_scraper/blob/main/images/example_output.png)
