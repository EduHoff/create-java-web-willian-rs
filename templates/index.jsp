<%@page contentType="text/html" pageEncoding="UTF-8"%>
<%@page import="javax.naming.InitialContext"%>
<%@page import="javax.naming.Context"%>
<%@page import="javax.sql.DataSource"%>
<%@page import="java.sql.Connection"%>
<!DOCTYPE html>
<html lang="pt-BR">
    <head>
        <meta charset="UTF-8">
        <title>project_name</title>
        <link rel="icon" type="image/png" href="assets/img/logo.png">
        <link rel="stylesheet" href="assets/css/style.css">
    </head>
    <body>
        <h1>Hello World, Java Web!</h1>
        <%
            String mensagem = "Welcome to the Tomcat server!";
            String dbStatus = "";
                boolean isConnected = false;

                try {
                    Context initContext = new InitialContext();
                    Context envContext = (Context) initContext.lookup("java:/comp/env");
                    DataSource ds = (DataSource) envContext.lookup("jdbc/app_db");

                    try (Connection conn = ds.getConnection()) {
                        if (conn != null && !conn.isClosed()) {
                            isConnected = true;
                            dbStatus = "Conexão estabelecida com sucesso!";
                        }
                    }
                } catch (Exception e) {
                    Throwable cause = e.getCause() != null ? e.getCause() : e;
                    dbStatus = "Erro detalhado: " + cause.toString();
                }
        %>
        <p><%= mensagem %></p>

        <div style="margin-top: 20px; padding: 10px; border-radius: 5px; background-color: <%= isConnected ? "#d4edda" : "#f8d7da" %>; color: <%= isConnected ? "#155724" : "#721c24" %>;">
            <strong>Status do Banco:</strong> <%= dbStatus %>
        </div>
    </body>
</html>
