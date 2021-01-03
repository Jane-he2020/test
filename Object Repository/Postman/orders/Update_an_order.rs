<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update_an_order</name>
   <tag></tag>
   <elementGuidId>8b2c9cf0-b3dc-42aa-96fc-d0ca9d1da888</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;id\&quot;: \&quot;72gbk6xNRQWKR9gXGYU0GA\&quot;, \n\&quot;batchId\&quot;: \&quot;vH4b-IDeQ1OC_TfrnXWRwQ\&quot;, \n\&quot;status\&quot;: {\n        \&quot;id\&quot;: \&quot;pending\&quot; \n   },\n\&quot;brand\&quot;: {\n        \&quot;id\&quot;: \&quot;FaY6G9VqQU6qtFXFeK7VoQ\&quot;\n},\n\&quot;retailer\&quot;: {\n       \&quot;id\&quot;: \&quot;KFNToZgwRoO_hUXXXXRayA\&quot;\n},\n\&quot;referenceId\&quot;: \&quot;A1\&quot;, \n\&quot;brandReferenceId\&quot;: \&quot;B2\&quot;, \n\&quot;retailerReferenceId\&quot;: \&quot;C3\&quot;, \n\&quot;recordedBrandPaymentTerm\&quot;: {\n        \&quot;id\&quot;: \&quot;net30\&quot; \n        },\n\&quot;recordedRetailerPaymentTerm\&quot;: { \n     \&quot;id\&quot;: \&quot;net30\&quot;\n},\n\&quot;orderDate\&quot;: \&quot;2019-10-22\&quot;,\n\&quot;shipDate\&quot;: \&quot;2019-10-29\&quot;,\n\&quot;cancelDate\&quot;: \&quot;2019-11-05\&quot;, \n\&quot;calculatedPaidByRetailerDate\&quot;: \&quot;2019-11-30\&quot;, \n\&quot;calculatedPaidToBrandDate\&quot;: \&quot;2019-12-15\&quot;, \n\&quot;calculatedRetailCostUsd\&quot;: \&quot;99.99\&quot;, \n\&quot;calculatedWholesaleCostUsd\&quot;: \&quot;65.99\&quot;, \n\&quot;shipToAddress\&quot;: {\n    \&quot;addressee\&quot;: \&quot;John Doe\&quot;, \n    \&quot;streetAddressLine_1\&quot;: \&quot;766 Brackbill Road\&quot;, \n    \&quot;streetAddressLine_2\&quot;: \&quot;Suite 1285\&quot;,\n\&quot;city\&quot;: \&quot;Philadelphia\&quot;,\n\&quot;region\&quot;: \&quot;PA\&quot;,\n\&quot;postalCode\&quot;: \&quot;17527\&quot;,\n\&quot;country\&quot;: {\n\&quot;id\&quot;: \&quot;us\&quot; \n}\n}, \n\&quot;contact\&quot;: {\n\&quot;name\&quot;: \&quot;John Doe\&quot;,\n\&quot;phone\&quot;: \&quot;+1 212-315-2861\&quot;\n},\n\&quot;items\&quot;: [ \n    {\n      \&quot;product\&quot;: {\n      \&quot;id\&quot;: \&quot;39X8EoAWRQyQZvG9SqOrhg\&quot;\n     }, \n\&quot;quantityType\&quot;: {\n       \&quot;id\&quot;: \&quot;purchase\&quot; \n       },\n\&quot;quantity\&quot;: \&quot;500\&quot;, \n\&quot;recordedLandingCommission\&quot;: \&quot;10.99\&quot;, \n\&quot;recordedMsrpUsd\&quot;: \&quot;10.99\&quot;, \n\&quot;recordedMargin\&quot;: \&quot;55\&quot;, \n\&quot;calculatedRetailCostUsd\&quot;: \&quot;5495\&quot;, \n\&quot;calculatedWholesalePriceUsd\&quot;: \&quot;4.95\&quot;, \n\&quot;calculatedWholesaleCostUsd\&quot;: \&quot;2475\&quot;\n} \n],\n\&quot;comments\&quot;: [ \n    {\n    \&quot;text\&quot;: \&quot;This is a comment\&quot;,\n    \&quot;createdAt\&quot;: \&quot;2019-12-28T04:52:32Z\&quot;, \n    \&quot;createdBy\&quot;: \&quot;6hIaqJ4JR9afp22X7vuxmg\&quot;\n} \n],\n\&quot;charges\&quot;: [\n     {\n\&quot;chargeType\&quot;: { \n    \&quot;id\&quot;: \&quot;commission\&quot;\n    },\n\&quot;value\&quot;: \&quot;25.0\&quot;,\n\&quot;description\&quot;: \&quot;Landing Commission\&quot;, \n\&quot;flat\&quot;: false,\n\&quot;toBuyer\&quot;: false\n} \n],\n\&quot;requirements\&quot;: [ \n    {\n\&quot;requirementType\&quot;: {\n     \&quot;id\&quot;: \&quot;ticketing\&quot;\n},\n\&quot;value\&quot;: \&quot;Yes\&quot;,\n\&quot;description\&quot;: \&quot;Some descriptive text about this requirement\&quot;\n} \n],\n\&quot;shipments\&quot;: [ \n    {\n\&quot;carrier\&quot;: {\n      \&quot;id\&quot;: \&quot;fedex\&quot; ,\n      \&quot;name\&quot;: \&quot;Fedex\&quot;\n},\n\&quot;trackingId\&quot;: \&quot;9341989678092542469995\&quot;, \n\&quot;trackingUrl\&quot;: \&quot;https://www.fedex.com/tracking/#\&quot;, \n\&quot;air\&quot;: true,\n\&quot;ground\&quot;: false,\n\&quot;ocean\&quot;: false\n} \n]\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>7.8.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${server_url}/orders/:id?_skipNulls=true</restUrl>
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
      <id>308fa715-d149-495a-a4d1-540da597155b</id>
      <masked>false</masked>
      <name>server_url</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
