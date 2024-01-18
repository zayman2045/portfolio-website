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
  height: auto;
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

@media (max-width: 768px) {
  .home .projects ul {
    flex-direction: column;
  }

  .home .flex-container {
    flex-direction: column;
  }
}

.home .projects ul li {
  flex: 1 1 30%;
  text-align: center;
  margin: 10px;
  box-sizing: border-box;
}

.home .projects img {
  height: 300px;
  width: auto;
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

/* About Projects */

.about-projects .content-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0px;
  margin: 30px auto;
  flex-wrap: wrap;
  width: 80vw;
}

.about-projects .content-container img {
  width: 100%;
  height: auto;
  border-radius: 10px;
}

.about-projects .content-container a:hover {
  border-color: #08f7be;
  transform: scale(1.1);
}

.about-projects .content-container a {
  width: 30%;
  border-radius: 10px;
  border: 2px solid #0e0e2a;
  transition: border-color 0.3s, transform 1.5s;
  margin: 10px 0px;
}

/* Stargaze, Funder, and Ello */
.stargaze header,
.funder header,
.ello header {
  text-align: center;
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

.new-mission-container a {
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

.new-mission-container a:hover {
  border-color: #08f7be;
  background-color: #3c3c3c;
  color: #08f7be; 
  transform: scale(1.1);
}

.mission-container a {
  border: 1px solid #a056f3;
  border-radius: 5px;
  width: 60%;
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

.missions .logout-button {
    position: absolute;     
    right: 10px;
    top: 45%; 
    transform: translateY(-50%);
}

.missions button {
   color: #08f7be;
    background-color: black;
    margin-left: 20px;
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
    margin: 300px auto 5px auto;
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
