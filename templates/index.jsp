<%@page contentType="text/html" pageEncoding="UTF-8"%>
<!DOCTYPE html>
<html lang="pt-BR">
    <head>
        <meta charset="UTF-8">
        <title>project_name</title>
        <link rel="stylesheet" href="assets/css/style.css">
    </head>
    <body>
        <h1>Hello World, Java Web!</h1>
        <%
            String mensagem = "Welcome to the Tomcat server!";
        %>
        <p><%= mensagem %></p>
    </body>
</html>
