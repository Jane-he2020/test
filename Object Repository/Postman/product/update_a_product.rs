<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>update_a_product</name>
   <tag></tag>
   <elementGuidId>e3b83008-9e74-4d5c-b920-b4e4fbfe479a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;brand\&quot;: {\r\n        \&quot;id\&quot;: \&quot;FaY6G9VqQU6qtFXFeK7VoQ\&quot;\r\n    },\r\n    \&quot;category\&quot;: {\r\n        \&quot;id\&quot;: \&quot;skincare\&quot;\r\n    },\r\n    \&quot;status\&quot;: {\r\n        \&quot;id\&quot;: \&quot;available\&quot;\r\n    },\r\n    \&quot;name\&quot;: \&quot;Moisturizing Lotion\&quot;,\r\n    \&quot;brandSku\&quot;: \&quot;0918/1732510\&quot;,\r\n    \&quot;msrpUsd\&quot;: \&quot;19.99\&quot;,\r\n    \&quot;size\&quot;: \&quot;20oz\&quot;,\r\n    \&quot;description\&quot;: \&quot;...\&quot;,\r\n    \&quot;benefits\&quot;: \&quot;...\&quot;,\r\n    \&quot;ingredients\&quot;: \&quot;...\&quot;,\r\n    \&quot;countryOfOrigin\&quot;: {\r\n        \&quot;id\&quot;: \&quot;us\&quot;\r\n    },\r\n    \&quot;barcodes\&quot;: [\r\n        {\r\n            \&quot;code\&quot;: \&quot;6393820003933\&quot;\r\n        }\r\n    ],\r\n    \&quot;documents\&quot;: [\r\n        {\r\n            \&quot;assetId\&quot;: \&quot;pXTv4schRaaLxPS70E5_Bw\&quot;,\r\n            \&quot;label\&quot;: \&quot;fake-document-label\&quot;\r\n        }\r\n    ],\r\n    \&quot;images\&quot;: [\r\n        {\r\n            \&quot;assetId\&quot;: \&quot;ZhmlAsD3S-WigxxcLsk3PA\&quot;,\r\n            \&quot;main\&quot;: true\r\n        }\r\n    ],\r\n    \&quot;tags\&quot;: [\r\n        {\r\n            \&quot;id\&quot;: \&quot;prop-65\&quot;\r\n        }\r\n    ]\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.8.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${server_url}/products/:id?_skipNulls=true</restUrl>
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
      <id>a83349ba-4071-4346-bcc6-a068ea941dbb</id>
      <masked>false</masked>
      <name>server_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
