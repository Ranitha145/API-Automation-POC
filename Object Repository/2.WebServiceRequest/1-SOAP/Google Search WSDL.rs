<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Google Search WSDL</name>
   <tag></tag>
   <elementGuidId>16f34735-6a1a-45cb-aaa6-794595b7f4b4</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;SOAP-ENV:Envelope xmlns:SOAP-ENV=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>

&lt;SOAP-ENV:Body>
&lt;ns1:doGoogleSearch xmlns:ns1=&quot;urn:GoogleSearch&quot; SOAP-ENV:encodingStyle=&quot;http://schemas.xmlsoap.org/soap/encoding/&quot;>

&lt;q xsi:type=&quot;xsd:string&quot;>anitha&lt;/q>
&lt;start xsi:type=&quot;xsd:int&quot;>0&lt;/start>
&lt;maxResults xsi:type=&quot;xsd:int&quot;>4&lt;/maxResults>
&lt;filter xsi:type=&quot;xsd:boolean&quot;>true&lt;/filter>
&lt;restrict xsi:type=&quot;xsd:string&quot;/>
&lt;safeSearch xsi:type=&quot;xsd:boolean&quot;>false&lt;/safeSearch>

&lt;/ns1:doGoogleSearch>
&lt;/SOAP-ENV:Body>
&lt;/SOAP-ENV:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>doGoogleSearch</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress>http://cs.au.dk/~amoeller/WWW/webservices/GoogleSearch.wsdl</wsdlAddress>
</WebServiceRequestEntity>
