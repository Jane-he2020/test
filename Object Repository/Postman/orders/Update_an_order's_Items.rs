<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update_an_order's_Items</name>
   <tag></tag>
   <elementGuidId>96d7e3e8-268f-42b3-bbca-4fb4e28e79be</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;product\&quot;: {\n\&quot;id\&quot;: \&quot;39X8EoAWRQyQZvG9SqOrhg\&quot; \n},\n\&quot;orderType\&quot;: { \n    \&quot;id\&quot;: \&quot;purchase\&quot; \n  },\n\&quot;recordedLandingCommission\&quot;: \&quot;10.99\&quot;, \n\&quot;recordedMsrpUsd\&quot;: \&quot;10.99\&quot;, \n\&quot;recordedMargin\&quot;: \&quot;55\&quot;, \n\&quot;calculatedRetailCostUsd\&quot;: \&quot;5495\&quot;, \n\&quot;calculatedWholesalePriceUsd\&quot;: \&quot;4.95\&quot;, \n\&quot;calculatedWholesaleCostUsd\&quot;: \&quot;2475\&quot;, \n\&quot;quantity\&quot;: \&quot;500\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.8.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${server_url}/orders/:id/items</restUrl>
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
      <id>1302b389-b078-47c0-8f02-7f28b62501f6</id>
      <masked>false</masked>
      <name>server_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
