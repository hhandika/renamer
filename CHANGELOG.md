## Upcoming:
- Make dry run check if the file exists
- Make dry run check if there files exist with the proposed names
- Add features to allow users using wildcard
- Add features to specify file extensions.

## Current:
- v0.3.4
    - Improved dry run features.    
        - Check if the original files exist.
        - Check if files exist for for the proposed names.
        - Colorized terminal outputs.
    - Add multi-directory wildcard support.

- v0.2.4
    - Allow multicolumns csv, although the app only process the first two columns.
    - Warn users if the csv is more than two column.
    - Panic when the csv is less than one column.

- v0.2.3
    - No need to write full path for the new name.
    - Performance improvement. Reduce memory and cpu usages.

- v0.2.2
    - In the cases of permission errors, the program will make sure the
        user enter the correct input.

- v0.2.1
    - Fix errors in help messages.

- v0.2.0
    - Handle errors when the app can't rename a file
    - Add a dry-run option
    - Colorful console print

- v0.1.0
    - First release
