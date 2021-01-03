<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>register_a_user+brand</name>
   <tag></tag>
   <elementGuidId>8cfd4c3c-ec34-4237-b5f3-1461dee20262</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;user\&quot;: {\r\n        \&quot;email\&quot;: \&quot;captain.america@avengers.com\&quot;,\r\n        \&quot;password\&quot;: \&quot;Password10\&quot;,\r\n        \&quot;confirmPassword\&quot;: \&quot;Password10\&quot;,\r\n        \&quot;givenName\&quot;: \&quot;Steve\&quot;,\r\n        \&quot;familyName\&quot;: \&quot;Rogers\&quot;,\r\n        \&quot;title\&quot;: \&quot;The First Avenger\&quot;\r\n    },\r\n    \&quot;brand\&quot;: {\r\n        \&quot;name\&quot;: \&quot;The Avengers\&quot;,\r\n        \&quot;profile\&quot;: {\r\n            \&quot;website\&quot;: \&quot;en.wikipedia.org/wiki/Avengers_(comics)\&quot;,\r\n            \&quot;instagram\&quot;: \&quot;www.instagram.com/avengers\&quot;\r\n        },\r\n        \&quot;website\&quot;: \&quot;\&quot;,\r\n        \&quot;about\&quot;: \&quot;\&quot;,\r\n        \&quot;addresses\&quot;: [\r\n            {\r\n                \&quot;addressee\&quot;: \&quot;Test\&quot;,\r\n                \&quot;label\&quot;: \&quot;hq\&quot;,\r\n                \&quot;streetAddressLine_1\&quot;: \&quot;北京\&quot;,\r\n                \&quot;streetAddressLine_2\&quot;: \&quot;\&quot;,\r\n                \&quot;city\&quot;: \&quot;北京\&quot;,\r\n                \&quot;region\&quot;: \&quot;北京\&quot;,\r\n                \&quot;postalCode\&quot;: \&quot;1000000\&quot;,\r\n                \&quot;country\&quot;: {\r\n                    \&quot;id\&quot;: \&quot;cn\&quot;\r\n                },\r\n                \&quot;hq\&quot;: true,\r\n                \&quot;defaultShipping\&quot;: false\r\n            }\r\n        ]\r\n    }\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.8.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${server_url}/brands/registration</restUrl>
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
      <id>beb4f14a-681c-434b-81a7-d479ab1f2ca7</id>
      <masked>false</masked>
      <name>server_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
