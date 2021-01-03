<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>register_a_user+retailer</name>
   <tag></tag>
   <elementGuidId>75cff73e-9ae6-4ecb-a64d-6a36a597c3ce</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;user\&quot;: {\r\n        \&quot;email\&quot;: \&quot;891907232@qq.com\&quot;,\r\n        \&quot;password\&quot;: \&quot;password10\&quot;,\r\n        \&quot;givenName\&quot;: \&quot;jane\&quot;,\r\n        \&quot;familyName\&quot;: \&quot;qinghe\&quot;,\r\n        \&quot;title\&quot;: \&quot;Professor\&quot;\r\n    },\r\n    \&quot;retailer\&quot;: {\r\n        \&quot;name\&quot;: \&quot;jane.qinghe\&quot;,\r\n        \&quot;profile\&quot;: {\r\n            \&quot;website\&quot;: \&quot;www.xmen.com\&quot;,\r\n            \&quot;instagram\&quot;: \&quot;www.instagram.com/avengers\&quot;\r\n        }\r\n    }\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.8.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${server_url}/retailers/registration</restUrl>
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
      <id>a914f94b-6c24-4cc8-93ae-e8798e2927c7</id>
      <masked>false</masked>
      <name>server_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
