
// get the loggedIn field from the localStorage

console.log("AFTER LOGIN SCRIPT");

const savePageButton = document.getElementById('savePageButton');

console.log('Save page button is', savePageButton);

if (savePageButton) {
  savePageButton.addEventListener('click', saveLink);
}

function getPage() {
  return url;
}

function saveLinkToCloud(data) {
  const linkSaveURL = "http://localhost:8080/api/v1/links/create";

  const token = localStorage.getItem('RUMARKIO_JWT_TOKEN');

  const logedIN = localStorage.getItem('RUMARKIO_USER_LOGGED_IN');
  const email = localStorage.getItem('RUMARKIO_USER_EMAIL');

  if (!token || token === 'undefined') {
    console.log("No token found");
    localStorage.setItem('RUMARKIO_USER_LOGGED_IN', false);
    return;
  }

  console.log("Token is", token);
  console.log(`Logged in: ${logedIN}`);
  console.log("Email is", email);


  fetch(linkSaveURL, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `JWT ${token}`
    },
    body: JSON.stringify(data)
  })
    .then(response => {
      if (response.ok) {
        console.log('Link saved successfully');

        const linkFormElem = document.getElementById('linkFormContainer');

        console.log(linkFormElem);

        const appContainer = document.getElementById('app-container');
        console.log(appContainer);
        appContainer.removeChild(linkFormElem);

        const newLinkSavedH1 = document.createElement('h1');
        newLinkSavedH1.textContent = 'Link saved successfully';

        appContainer.appendChild(newLinkSavedH1);

      } else {
        console.log(response);
        console.error('Error saving link:', response.status);
      }
    }
    )
    .catch(error => {
      console.error('Error saving link:', error);
    });

}

function saveLink() {
  console.log("Saving Page Button");
  const linkDescription = document.getElementById('textArea');

  const LinkDescriptionText = '';
  if (linkDescription) {
    linkDescriptionText = linkDescription.value;
  }

  const tagName = document.getElementById('tagName');

  let tagNameText = '';

  let tagList = [];

  if (tagName) {
    tagNameText = tagName.value;
    console.log(tagNameText);
    tagList = tagNameText.split(',');
  }


  const sessionName = document.getElementById('sessionName');
  let sessionNameText = '';

  if (sessionName) {
    sessionNameText = sessionName.value;
  }

  const contextName = document.getElementById('contextName');
  let contextNameText = '';

  if (contextName) {
    contextNameText = contextName.value;
  }

  let url = "";
  let title = "";
  browser.tabs.query({ currentWindow: true, active: true })
    .then((tabs) => {
      url = tabs[0].url;
      title = tabs[0].title;
      console.log(tabs[0]);
      const pageData = {
        'title': title,
        'url': url,
        'description': linkDescriptionText,
        'tag_names': tagList,
        // 'tags': tagNameText,
        'session_name': sessionNameText
      }
      console.log(pageData);
      console.log("Let's try and save this data to cloud");
      saveLinkToCloud(pageData);
    })
};

