FROM debian:bookworm-slim

#RUN apt-get update && apt-get -y install \
#    neofetch

WORKDIR /app

COPY ./target/release/dennis_lawter_resume_api_server ./dennis_lawter_resume_api_server
COPY ./.env ./
COPY ./static/ /app/static/
COPY ./neofetch.config ./

#RUN /bin/bash -c "/usr/bin/neofetch" > /app/static/neofetch.txt

# Command to run your application
#CMD ["source /app/.env", "./dennis_lawter_resume_api_server"]
CMD ["./dennis_lawter_resume_api_server"]
#CMD ["/bin/bash"]
#CMD /bin/bash -c "cd /app && ./dennis_lawter_resume_api_server"
