import service_provider from 'ic:canisters/service_provider';

service_provider.greet(window.prompt("Enter your name:")).then(greeting => {
  window.alert(greeting);
});
