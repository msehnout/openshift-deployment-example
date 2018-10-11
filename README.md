# Steps:

 1. Create the application
 2. Write a Dockerfile
 3. Build the app and the container
 4. Test locally using `docker run -p 8080:8080 xxx`, `docker container ls`
 5. Tag the container
 6. Assuming you have DockerHub account, push it
 7. Execute `oc new-app msehnout/test:1 --name central-node`
 8. After update, execute `oc import-image istag/central-node:1`
 9. Add readiness and liveness probes
 10. Expose using `oc expose svc/central-node`
