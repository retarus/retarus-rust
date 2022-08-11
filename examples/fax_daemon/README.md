## Example Application: Fax Daemon

This application should be an exmaple how you could use the retarus sdk.

## What does the application
The application is a fax daemon that watches for changes in the "out" directory, if someone creates a pdf with a specify name schema (recipient_number_filename.pdf) it will create and send a fax to the retarus servers. After the job has been processed it will create a fax report in the "in" folder.

