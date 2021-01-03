<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>set_a_brand's_social_accounts</name>
   <tag></tag>
   <elementGuidId>63b7a184-fe75-4cd4-b6e0-b1d314abc0b6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;[\r\n    {\r\n        \&quot;platform\&quot;: {\r\n            \&quot;id\&quot;: \&quot;instagram\&quot;,\r\n            \&quot;name\&quot;: \&quot;Instagram\&quot;\r\n        },\r\n        \&quot;address\&quot;: \&quot;instagram.com/homerman\&quot;\r\n    },\r\n    {\r\n        \&quot;platform\&quot;: {\r\n            \&quot;id\&quot;: \&quot;facebook\&quot;,\r\n            \&quot;name\&quot;: \&quot;Facebook\&quot;\r\n        },\r\n        \&quot;address\&quot;: \&quot;facebook.com/homerman\&quot;\r\n    }\r\n]&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.8.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${server_url}/brands/:id/social-accounts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.server_url</defaultValue>
      <description></description>
      <id>5b73644a-574e-456c-b7f5-0f645125f2f7</id>
      <masked>false</masked>
      <name>server_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
