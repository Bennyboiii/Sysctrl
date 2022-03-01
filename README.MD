# SYSCTRL: Driver, Library and service manager for all platforms

SYSCTRL is a basic service manager written in Rust as a learning project. It has basic functionality, such as moving
service files around according to the users needs, creating and deleting services, starting and stopping them, as well 
as listing them out.

## Starting out:

The first thing you will need to do in SYSCTRL is create a directory where all your service files will go. This can
be done with mkdir. You can then use this command: <br/><br/>

- sysctrl path <path-you-created> <br/> <br/>

SYSCTRL will now use this path to store service files. <br/>
To add a service first do: <br/> <br/>

- sysctrl create <service-name> <path-to-system-folder> <br/> <br/>

The <path-to-system-folder> references the path the service files are usually installed in. <br/>
Next, go to the folder you created with mkdir. You will notice another folder has been created inside the one you made.
Drop the files in that folder. <br/> <br/>

You can then move the files you put into that folder into the system folder using: <br/> <br/>

- sysctrl start <service-name> <br/> <br/>

And move them back using: <br/> <br/>

- sysctrl stop <service-name> <br/> <br/>

If you want to delete a service, use: <br/> <br/>

- sysctrl delete <service-name> <br/> <br/>

This will not delete the service files in the storage. <br/>
Lastly, if you want to list all your services, do: <br/> <br/>

- sysctrl list <br/> <br/>

This will print out the assoc.db file. It has all the service names next to the paths that they lead to.

