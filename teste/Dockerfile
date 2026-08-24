FROM eclipse-temurin:17-jdk-jammy AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends ant && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

RUN ant dist

FROM tomcat:9.0-jdk17-corretto

RUN rm -rf /usr/local/tomcat/webapps/*

COPY --from=builder /app/dist/*.war /usr/local/tomcat/webapps/ROOT.war

EXPOSE 8080
CMD ["catalina.sh", "run"]