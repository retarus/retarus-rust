## Example application: Fax Daemon

This application is an example for how you could use the Retarus SDK.

## What does the application
The application is a fax daemon that watches for changes in the "out" directory. If someone creates a pdf with a specific name schema (recipient_number_filename.pdf), it will create and send a fax to the Retarus servers. After the job has been processed, it will create a fax report in the "in" folder.

