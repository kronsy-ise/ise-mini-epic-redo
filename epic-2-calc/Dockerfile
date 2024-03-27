FROM maven:latest as build

ENV HOME=/usr/app
RUN mkdir -p $HOME 

WORKDIR $HOME 

ADD . $HOME 

RUN --mount=type=cache,target=/root/.m2 mvn -f $HOME/pom.xml clean package

FROM eclipse-temurin:17-jre-jammy


WORKDIR /app 

RUN apt update -y


ARG JAR_FILE=/usr/app/target/ise-epic2-calc.jar 
COPY --from=build $JAR_FILE /app/runner.jar 

ENTRYPOINT java -cp /app/runner.jar dev.kronsy.ise.epic2.App
