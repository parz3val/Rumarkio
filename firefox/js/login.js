document.addEventListener('submit', function (event) {
  event.preventDefault(); // Prevent the form from submitting the traditional way
  login(); // Call your login function
});

function login() {
  const email = document.getElementById('email').value;
  const password = document.getElementById('password').value;

  if (email === '' || password === '') {
    document.getElementById('login-msg-label').innerHTML = 'Email or password is empty.';
    return;
  }
  const loginUrl = "http://localhost:8080/api/v1/users/login";

  fetch(loginUrl, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      email: email,
      password: password
    }),
  }).then(response => {
    if (response.ok) {
      // get the response body
      response.json().then(data => {
        console.log("Login successful:", data);

        // Set up setuff in the localStorage
        localStorage.setItem('RUMARKIO_USER_LOGGED_IN', true);
        localStorage.setItem('RUMARKIO_USER_EMAIL', email);
        console.log("token is", data.data.token); // debug, to be removed
        localStorage.setItem('RUMARKIO_JWT_TOKEN', data.data.token)

        const loggedInUrl = browser.runtime.getURL('html/loggedIn.html');
        fetch(loggedInUrl)
          .then(response => response.text())
          .then(html => {
            const appContainer = document.getElementById('app-container');
            if (appContainer) {

              const innerElement = document.createElement('div');
              innerElement.setAttribute('id', 'linkFormContainer');
              innerElement.innerHTML = html;
              appContainer.appendChild(innerElement);

              // injecting script to the page 
              // Using ../js/script.js instead of runtime url like above
              // because it didn't work for some reason
              const loginScript = document.createElement('script');
              loginScript.src = "../js/script.js";
              document.body.appendChild(loginScript);


            } else {
              console.log("appContainer is null");
            }
          })
          .catch(error => {
            console.error('Error loading HTML:', error);
          });
      });
    } else {
      console.error('Login failed:', response);
    }
  })
}
