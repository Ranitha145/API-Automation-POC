import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl(findTestData('1-Web').getValue(1, 1))

WebUI.setText(findTestObject('1-Web/Page_Sentrifugo - Open Source HRMS/input_Login_username'), findTestData('1-Web').getValue(
        2, 1))

WebUI.setText(findTestObject('1-Web/Page_Sentrifugo - Open Source HRMS/input_Login_password'), findTestData('1-Web').getValue(
        3, 1))

WebUI.click(findTestObject('1-Web/Page_Sentrifugo - Open Source HRMS/input_Login_loginsubmit'))

WebUI.waitForPageLoad(1)

WebUI.waitForElementPresent(findTestObject('Object Repository/1-Web/Page_Sentrifugo - Open Source HRMS/img'), 0)

Date today = new Date()

String todaysDate = today.format('MM_dd_yy')

String nowTime = today.format('hh_mm_ss')

WebUI.takeScreenshot(((('D:/Katalon/screenshot/image_' + todaysDate) + '-') + nowTime) + '.PNG')

WebUI.click(findTestObject('Object Repository/1-Web/Page_Sentrifugo - Open Source HRMS/a_Super Admin'))

WebUI.click(findTestObject('Object Repository/1-Web/Page_Sentrifugo - Open Source HRMS/a_Logout'))

WebUI.closeBrowser()

