#include <cstdlib>
#include <iostream>
#include <string>
#include <thread>

#include "auth_client.h"
#include "config/auth_client_config.h"

using namespace std;

int main()
{
  AuthClientConfig authClientConfig;
  authClientConfig.updateEnvironment("staging");
  authClientConfig.m_refreshToken = "put an access token here";
  AuthClient authClient{authClientConfig};
  const std::string *accessToken = authClient.getAccessToken();

  if (accessToken != nullptr)
  {
    std::cout << "Got access token: " << *authClient.getAccessToken()
              << std::endl;
    cout << "getAccessToken worked successfully" << endl;
  }
}