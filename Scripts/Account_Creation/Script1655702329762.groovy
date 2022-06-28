import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://10.244.90.57/imal_core_portal/login/unSecureAction_prelogin')

WebUI.maximizeWindow()

WebUI.click(findTestObject('Page_Login/Advanced'))

WebUI.click(findTestObject('Page_Login/Proceed'))

WebUI.setText(findTestObject('Object Repository/Account_Creation/Page_Login/input_User Name_j_username'), '1477')

WebUI.setText(findTestObject('Object Repository/Account_Creation/Page_Login/input_Password_j_password_to_remove'), 'Test@123')

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Login/td_Login'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Company and Branch Login/span_Continue'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_iMAL Welcome Message/span_Continue'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_CSM CUSTOMER/td_Accounts'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_CSM CUSTOMER/td_General Accounts'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_CSM CUSTOMER/span_Opening'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Opening/span__ui-icon ui-icon-search live-search-ui'))

WebUI.setText(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Opening/input__BRIEF_DESC_ENG'), 
    'om')

WebUI.doubleClick(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Opening/td_26'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Opening/span__ui-icon ui-icon-search live-search-ui_1'))

WebUI.delay(3)

WebUI.click(findTestObject('Account_Creation/Page_Accounts  General Accounts  Opening/input_GlType'))

WebUI.setText(findTestObject('Account_Creation/Page_Accounts  General Accounts  Opening/input_GlType'), '202101')

WebUI.sendKeys(findTestObject('Account_Creation/Page_Accounts  General Accounts  Opening/input_GlType'), Keys.chord(Keys.ENTER))

WebUI.delay(3)

WebUI.click(findTestObject('Account_Creation/Page_Accounts  General Accounts  Opening/input_CifType'))

WebUI.setText(findTestObject('Account_Creation/Page_Accounts  General Accounts  Opening/input_CifType'), '10133986')

WebUI.click(findTestObject('Account_Creation/Page_Accounts  General Accounts  Opening/span__ui-icon ui-icon-search live-search-ui'))

WebUI.delay(3)

WebUI.click(findTestObject('Account_Creation/Page_Accounts  General Accounts  Opening/span_Mailing details'))

WebUI.setText(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Opening/textarea_Description_accountsManagementCO.a_ca0ebb'), 
    'test')

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Opening/span_Main Details'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Opening/label_Save'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Opening/input_Record Saved Successfully__popup_path_sol_ok'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Opening/span_Dismiss'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Opening/label_Log Out'))

WebUI.setText(findTestObject('Object Repository/Account_Creation/Page_Login/input_User Name_j_username'), '1144')

WebUI.setText(findTestObject('Object Repository/Account_Creation/Page_Login/input_Password_j_password_to_remove'), 'Test@123')

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Login/label_Login'))

WebUI.setText(findTestObject('Login_branch/Page_Company and Branch Login/input_Branch Code_branchesVO.BRANCH_CODE'), '900')

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Company and Branch Login/span_Continue'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_iMAL Welcome Message/span_Continue'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_CSM CUSTOMER/td_Accounts'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_CSM CUSTOMER/td_General Accounts'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_CSM CUSTOMER/span_Authorize'))

WebUI.setText(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Authorize/input__CIF_SUB_NO'), 
    '10133986')

WebUI.sendKeys(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Authorize/input__CIF_SUB_NO'), 
    Keys.chord(Keys.ENTER))

WebUI.doubleClick(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Authorize/td_512'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Authorize/label_Authorize'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Authorize/input_183 Do you want to Print confirmation_70fca4'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Authorize/input_Record was Authorized Successfully__p_066c70'))

WebUI.click(findTestObject('Object Repository/Account_Creation/Page_Accounts  General Accounts  Authorize/label_Log Out'))

