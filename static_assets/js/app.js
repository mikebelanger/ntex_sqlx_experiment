/**
*
*/
function loadPageHtml(url) {
  return fetch(url)
    .then((payload) => {
      return payload.text()
    })
}