//! This file contains the CSS styles for the frontend.

/// The CSS stylesheet for the frontend.
pub const STYLESHEET: &str = r#"

  background-color: #0e0e2a;
  color: #e0e0e0;
  font-family: "Arial, sans-serif";
  min-height: 100vh;

h1,
h2 {
  text-align: center;
  margin: 20px 0;
}

h1 {
  color: #a056f3;
  font-size: 36px;
}

h2 {
  color: #08f7be;
  font-size: 28px;
}

a {
  text-decoration: none;
}

button {
  margin: 10px;
  padding: 10px 20px;
  font-size: 16px;
  color: #e0e0e0;
  background-color: #2e2e2e;
  border: 1px solid #a056f3;
  border-radius: 5px;
  cursor: pointer;
  transition: background-color 0.3s, border-color 0.3s;
}

button:hover {
  background-color: #3c3c3c;
  border-color: #08f7be;
}

footer {
  text-align: center;
  padding: 20px;
  border-top: 1px solid #a056f3;
}

footer p {
  margin: 10px 0;
}

footer p a {
  color: #08f7be;
}

footer p a:hover {
  text-decoration: underline;
}

/* Navbar */
.navbar {
  background-color: black;
  color: #e0e0e0;
  padding: 5px 0;
  position: sticky;
  top: 0;
  z-index: 1000;
}

.navbar-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin: 0 auto;
  max-width: 100vw;
  padding: 0 20px;
}

.navbar-container .navbar-logo {
  height: 70px;
  width: 70px;
  border-radius: 50%;
  display: inline;
  border: 2px solid #a056f3;
  transition: border-color 0.3s;
  padding: 2px;
  margin: 0;
  box-sizing: border-box;
}

.navbar-container .navbar-logo:hover {
  border: 2px solid #08f7be;
}

.navbar-titles {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.navbar h1,
.navbar h2 {
  text-align: left;
  margin: 5px 0;
}

.navbar h1 {
  color: #a056f3;
  font-size: 26px;
}

.navbar h2 {
  color: #08f7be;
  font-size: 16px;
}

.navbar-left {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}

.navbar-left a {
  border-radius: 50px;
}

.navbar-right {
  display: flex;
  justify-content: center;
  flex-wrap: wrap;
}

.navbar-right a {
  margin: 10px;
  padding: 10px 20px;
  font-size: 16px;
  color: #e0e0e0;
  background-color: black;
  border: 1px solid #a056f3;
  border-radius: 5px;
  cursor: pointer;
  transition: border-color 0.3s;
}

.navbar-right a:hover {
  border-color: #08f7be;
}

/* Home Page */
.home .background {
  display: block;
  margin: 0;
  width: 100vw;
  height: 100vh;
  object-fit: cover; 
}

.home .flex-container {
  display: flex;
  flex-wrap: wrap;
}

.home .flex-container a {
  flex: 1;
  padding: 0;
  color: white;
  margin: 30px 50px;
  border-radius: 30px;
  border: 1px solid #a056f3;
  padding: 20px;
  transition: background-color 1.5s, border-color 0.3s, transform 1.5s;
}

.home .flex-container a:hover {
  border-color: #08f7be;
  transform: scale(1.1);
  background-color: black;
}

.home .flex-container section {
  margin: 0;
}

.home section.projects {
  margin: 30px 50px;
  padding: 20px;
  border: 1px solid #a056f3;
  border-radius: 30px;
}

.home section h2 {
  font-size: 2em;
  text-align: left;
}

.home section p {
  font-size: 1.25em;
}

.home .projects ul {
  list-style-type: none;
  display: flex;
  justify-content: space-between;
  flex-wrap: wrap;
  padding: 0;
  margin: 0;
}

.home .projects ul li {
  flex: 1 1 30%;
  text-align: center;
  margin: 10px;
  box-sizing: border-box;
}

.home .projects img {
  height: 300px;
  width: 300px;
  border: 2px solid #a056f3;
  border-radius: 50px;
  transition: border-color 0.3s, transform 1.5s;
}

.home .projects a {
  display: inline-block;
  height: 0px;
  width: auto;
}

.home .projects img:hover {
  border-color: #08f7be;
  transform: scale(1.1);
}

@media (max-width: 600px) {
  .home .projects ul {
    flex-direction: column;
  }

  .home .flex-container {
    flex-direction: column;
  }

  .home .projects img {
    height: 200px;
    width: auto;
  }
}

/* Login, Signup and Build */
.login form,
.signup form, 
.build form {
  width: 80%;
  margin: auto;
}

.login label,
.signup label,
.build label {
  display: inline-block;
  width: 100%;
  margin-bottom: 10px;
  font-weight: bold;
}

.login input[type="text"],
.login input[type="password"],
.signup input[type="text"],
.signup input[type="password"],
.build input[type="text"], 
.build textarea {
  width: 100%;
  padding: 10px;
  margin: 5px 0 20px;
  border: 1px solid #a056f3;
  border-radius: 5px;
}

.build input[type="text"], 
.build textarea {
    background-color: #0e0e2a;
    color: white;
}

.build textarea {
    height: 200px;
    background-color: #0e0e2a;
    color: white;
}

.build-content {
    width: 50vw;
    margin: 20px auto;
    padding: 20px;
    border: 1px solid #a056f3;
    border-radius: 5px;
    background-color: black;
}

.login-signup-content {
    width: 30vw;
    margin: 20px auto;
    padding: 20px;
}

.login button[type="submit"],
.signup button[type="submit"],
.build button[type="submit"] {
  width: 60%;
  margin: 15px auto;
  display: block;
  background-color: #000;
  color: #08f7be;
  padding: 14px 20px;
  border: 1px solid #a056f3;
  border-radius: 5px;
  cursor: pointer;
  transition: border-color 0.3s, transform 1.5s, background-color 0.3s, color 0.3s;
}

.login button[type="submit"]:hover,
.signup button[type="submit"]:hover,
.build button[type="submit"]:hover {
  border-color: #08f7be;
  background-color: #3c3c3c;
  color: #08f7be;
  transform: scale(1.1);
}

@media (max-width: 900px) {
  .login-signup-content {
    width: 50vw;
    margin: 20px auto;
    padding: 20px;
  }

  .build-content {
    width: 80%;
  }
}
/* About Me */

.about-me .content-container {
  display: flex;
  justify-content: space-evenly;
  align-items: center;
  gap: 20px;
  padding: 20px;
  margin: 20px 50px;
  flex-wrap: wrap;
}

.about-me img {
  height: auto;
  width: 35vw;
  border: 2px solid #a056f3;
  border-radius: 50px;
}

.about-me .content-container p {
  height: auto;
  width: 40vw;
  font-size: 18px;
}

@media (max-width: 900px) {
  .about-me .content-container {
    display: block;
    text-align: center;
    margin: 10px 10px;
  }
  
  .about-me img {
    height: auto;
    width: 85%;
    border: 2px solid #a056f3;
    border-radius: 50px;
  }
  
  .about-me .content-container p {
    height: auto;
    width: 100%;
    font-size: 18px;
    text-align: left;
  }
}

/* About Site */
.about-site .crate-container img {
  height: 150px;
  width: 150px;
  border: 2px solid #a056f3;
  border-radius: 50px;
  transition: border-color 0.3s, transform 1.5s;
}

.about-site .crate-container img:hover {
  border-color: #08f7be;
  transform: scale(1.1);
}

.about-site .crate-container a {
  border-radius: 50px;
}

.about-site .crate-container {
  display: flex;
  justify-content: space-evenly;
  align-items: center;
  gap: 20px;
  padding: 20px;
  margin: 20px 50px;
}

.about-site .crate-container .content-container {
  width: 50%;
}

@media (max-width: 600px) {
  .about-site .crate-container .content-container {
    width: 100%;
  }

  .about-site .crate-container {
    display: block;
    margin: 0px;
    text-align: center; /* Center-align inline or inline-block elements inside */
  }

  .about-site .crate-container a {
    display: inline-block; /* Make 'a' tags inline-block to center them */
    margin: 0 auto; /* Additional centering for block-level elements */
  }
}

/* About Projects */

.about-projects .content-container {
  display: flex;
  justify-content: space-around;
  align-items: center;
  padding: 0;
  margin: 30px auto;
  flex-wrap: wrap;
  width: 90vw;
  gap: 20px;
}

.about-projects .content-container a {
  border-radius: 10px;
  border: 2px solid #a056f3;
  transition: border-color 0.3s, transform 1.5s;
  display: block;
  overflow: hidden;
}

.about-projects .content-container a:hover {
  transform: scale(1.05);
  border-color: #08f7be;
}

.about-projects .content-container img {
  width: 300px;
  height: 533px;
  display: block;
  vertical-align: middle;
}

@media (max-width: 1100px) {
  .about-projects .content-container img {
    width: 230px;
    height: 408px;
  }
}

@media (max-width: 850px) {
  .about-projects .content-container {
    flex-direction: column;
  }
}

/* Missions */

.missions {
  text-align: left;
}

.missions .logged-out {
  border: 1px solid #a056f3;
  border-radius: 5px;
  width: 50vw;
  margin: 20px auto;
  padding: 10px 20px;
}

.missions .logged-out h3 {
  text-align: center;
  font-size: 1.5em;
  color: #a056f3
}

.missions .logged-out li {
  margin: 10px;
}

.missions .logged-in {
  width: 70vw;
  margin: 20px auto;
}

.missions .user-welcome {
  margin: 20px 0 0 0;
}

.new-mission-container a, .new-mission-container button {
  border: 1px solid #a056f3;
  border-radius: 5px;
  width: 60%;
  margin: 20px auto;
  display: block;
  color: #a056f3;
  padding: 7px 10px;
  cursor: pointer;
  text-align: center;
  transition: border-color 0.3s, transform 1.5s, background-color 0.3s, color 0.3s;
}

.new-mission-container button {
  font-size: 14px;
  width: 62%;
}

.new-mission-container a:hover, .new-mission-container button:hover {
  border-color: #08f7be;
  background-color: #3c3c3c;
  color: #08f7be; 
  transform: scale(1.1);
}

.mission-container a {
  border: 1px solid #a056f3;
  border-radius: 5px;
  width: 80%;
  margin: 20px auto;
  display: block;
  background-color: black;
  color: #08f7be;
  padding: 15px 10px;
  cursor: pointer;
  text-align: center;
  transition: border-color 0.3s, transform 1.5s, background-color 0.3s, color 0.3s;
}

.mission-container a:hover {
  border-color: #08f7be;
  background-color: #3c3c3c;
  color: #08f7be; 
  transform: scale(1.1);
}

.missions header {
    position: relative; 
    display: flex;              
    justify-content: center; 
    align-items: center;                    
}

.missions header h1 {
    flex: 0 1 auto;              
}

.missions button {
   color: #08f7be;
    background-color: black;
}

.mission-container h3, .mission-container p {
  text-align: left;
  margin: 10px 30px;
}

.mission-container h3 {
  font-size: 1.5em;
}

.mission-container p {
  color: white;
}

.missions .btn-container {
  display: flex;
  justify-content: space-evenly;
  align-items: center;
  gap: 5px;
  padding: 20px;
  margin: 20px auto;
  flex-wrap: wrap;
}

.missions .btn-container a {
  width: 20%;
  background-color: black;
  color: #08f7be;
  padding: 14px 25px;
  margin: 10px;
  border: 1px solid #a056f3;
  border-radius: 5px;
  cursor: pointer;
  text-align: center;
  transition: border-color 0.3s, transform 1.5s, background-color 0.3s, color 0.3s;
}

.missions .btn-container a:hover {
  border-color: #08f7be;
  background-color: #3c3c3c;
  color: #08f7be;
  transform: scale(1.1);
}

@media (max-width: 950px) {
  .missions .logged-out {
    width: 80%;
  }

  .mission-container a {
    width: 90%;
  }
}

/* Inspect */

.inspect .mission-details {
    width: 50vw;
    margin: 20px auto;
    padding: 20px;
    border: 1px solid #a056f3;
    border-radius: 5px;
    background-color: black;
}

.inspect .mission-details h1 {
    margin-bottom: 20px;
}

.inspect .mission-details h2 {
    text-align: left;
    padding: 10px 20px;
    margin: 10px 0px;
    color: #08f7be;
    border-bottom: 1px solid #a056f3;
}

.inspect .mission-details p {
    text-align: left;
    padding: 10px 20px;
    margin: 10px 0px;
    color: white;
}

.inspect .mission-btn-container {
    display: flex;
    justify-content: space-evenly;
    align-items: center;
    gap: 5px;
    margin: 150px auto 5px auto;
    flex-wrap: wrap;
    }
  
  .inspect .mission-btn a {
      width: 20px;
      background-color: #000;
      color: #08f7be;
      padding: 10px 20px;
      border: 1px solid #a056f3;
      border-radius: 50px;
      cursor: pointer;
      transition: border-color 0.3s, transform 1.5s, background-color 0.3s, color 0.3s;
    }
  
    .inspect .mission-btn a:hover {
      border-color: #08f7be;
      background-color: #3c3c3c;
      color: #08f7be;
      transform: scale(1.1);
    }

    @media (max-width: 900px) {
      .inspect .mission-details {
        width: 80vw;
      }
    }

    /* Delete */

    .delete-content {
        width: 50vw;
        margin: 50px auto;
        padding: 20px;
        border: 1px solid #a056f3;
        border-radius: 5px;
        background-color: black;
    }

    .delete-btn-container {
        width: 50vw;
        margin: 20px auto;
        display: flex;
        justify-content: space-evenly;
        align-items: center;
        gap: 5px;
        flex-wrap: wrap;
        text-align: center;
    }

    .delete button[type="submit"], .delete-btn-container a {
        margin: 15px auto;
        background-color: #000;
        color: #08f7be;
        padding: 14px 20px;
        border: 1px solid #a056f3;
        border-radius: 5px;
        cursor: pointer;
        transition: border-color 0.3s, transform 1.5s, background-color 0.3s, color 0.3s;
    }

    .delete button[type="submit"]:hover, .delete-btn-container a:hover {
        border-color: #08f7be;
        background-color: #3c3c3c;
        color: #08f7be;
        transform: scale(1.1);
    }

"#;
