// get the loggedIn field from the localStorage
const loggedIn = localStorage.getItem('RUMARKIO_USER_LOGGED_IN');

console.log("Initializing extension");

if (loggedIn === 'true') {
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

        console.log("Dom updated");

        // Connect the login script to the page
        const loginScript = document.createElement('script');
        loginScript.src = "../js/script.js";
        document.body.appendChild(loginScript);

      } else {
        console.log("appContainer is null");
      }
      // document.getElementById('app-container').innerHTML = html;
    })
    .catch(error => {
      console.error('Error loading HTML:', error);
    });
} else {
  const notLoggedInUrl = browser.runtime.getURL('html/loggedOut.html');
  fetch(notLoggedInUrl)
    .then(response => response.text())
    .then(html => {
      const appContainer = document.getElementById('app-container');
      if (appContainer) {
        const innerElement = document.createElement('div');
        innerElement.innerHTML = html;
        appContainer.appendChild(innerElement);
      } else {
        console.log("appContainer is null");
      }
      // document.getElementById('app-container').innerHTML = html;
    })
    .catch(error => {
      console.error('Error loading HTML:', error);
    });
}

