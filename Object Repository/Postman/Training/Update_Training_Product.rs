<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update_Training_Product</name>
   <tag></tag>
   <elementGuidId>28859400-1b10-4ac4-9d56-399fba46eda7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;id\&quot;: \&quot;prod-123\&quot;,\n    \&quot;version\&quot;: \&quot;${version}\&quot;,\n    \&quot;brandId\&quot;: \&quot;brand123\&quot;,\n    \&quot;brandShortName\&quot;: \&quot;Codafi\&quot;,\n    \&quot;audit\&quot;: {\n        \&quot;created\&quot;: \&quot;2020-12-10T08:57:05.249UTC\&quot;,\n        \&quot;createdBy\&quot;: \&quot;Hye-young Lee\&quot;,\n        \&quot;modified\&quot;: \&quot;2020-12-10T08:57:05.249UTC\&quot;,\n        \&quot;modifiedBy\&quot;: \&quot;Hye-young Lee\&quot;\n    },\n    \&quot;bestUsedFor\&quot;: [\n        {\n            \&quot;id\&quot;: \&quot;1\&quot;,\n            \&quot;name\&quot;: \&quot;Before Face wash\&quot;\n        }\n    ],\n    \&quot;category\&quot;: [\n        \&quot;Skin Cream\&quot;,\n        \&quot;Skin Lotion\&quot;\n    ],\n    \&quot;crossSellers\&quot;: [\n        {\n            \&quot;name\&quot;: \&quot;Dove Lotion\&quot;,\n            \&quot;productId\&quot;: \&quot;product-789-updated\&quot;\n        }\n    ],\n    \&quot;demoTips\&quot;: \&quot;demo tips\&quot;,\n    \&quot;howToDemo\&quot;: \&quot;how to demo\&quot;,\n    \&quot;isHero\&quot;: true,\n    \&quot;isPublic\&quot;: false,\n    \&quot;isPublished\&quot;: false,\n    \&quot;media\&quot;: {\n        \&quot;videos\&quot;: [\n            {\n                \&quot;rawFile\&quot;: \&quot;media/video/codafi/test.mp4\&quot;,\n                \&quot;thumbnail\&quot;: \&quot;media/image/codafi/test.jpeg\&quot;,\n                \&quot;title\&quot;: \&quot;My Video Title\&quot;\n            }\n        ]\n    },\n    \&quot;name\&quot;: \&quot;Cetaphil\&quot;,\n    \&quot;order\&quot;: 2147483647,\n    \&quot;productId\&quot;: \&quot;prod-123\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.8.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${secure_url}/secure/trainingPages/prod-123</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.secure_url</defaultValue>
      <description></description>
      <id>a804f296-e5c0-4e8d-8d98-f116e6aaa06f</id>
      <masked>false</masked>
      <name>secure_url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.version</defaultValue>
      <description></description>
      <id>6c07b8a9-989b-4d21-b867-34b1583495ec</id>
      <masked>false</masked>
      <name>version</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
