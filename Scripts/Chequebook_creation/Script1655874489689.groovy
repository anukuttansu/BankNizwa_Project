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

WebUI.setText(findTestObject('Object Repository/Cheque_Book/Page_Login/input_User Name_j_username'), '1477')

WebUI.setText(findTestObject('Object Repository/Cheque_Book/Page_Login/input_Password_j_password_to_remove'), 'Test@123')

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Login/label_Login'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Company and Branch Login/span_Continue'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_iMAL Welcome Message/span_Continue'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_CSM CUSTOMER/td_Chequebook Request'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_CSM CUSTOMER/span_Maintenance'))

WebUI.setText(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Maintenance/input__chequeBookCO.ctschqbookVO.NUM_OF_CHQBOOKS'), 
    '1')

WebUI.setText(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Maintenance/input_Serial Number_chequeBookCO.ctschqbook_a1b49d'), 
    '12')

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Maintenance/span__ui-icon ui-icon-search live-search-ui'))

WebUI.setText(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Maintenance/input__BRIEF_NAME_ENG'), 
    '50')

WebUI.doubleClick(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Maintenance/td_50 leaves Individual'))

WebUI.setText(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Maintenance/input_Serial Number_chequeBookCO.ctschqbook_a1b49d'), 
    '12')

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Maintenance/span_AC No_ui-icon ui-icon-search live-search-ui'))

WebUI.setText(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Maintenance/input_Status_amfVO.CIF_SUB_NO'), 
    '10133876')

WebUI.doubleClick(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Maintenance/td_Current Account'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Maintenance/label_Save'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Maintenance/input_3037 GL Code not specified for this c_c34c59'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Maintenance/span_Dismiss'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Maintenance/label_Log Out'))

WebUI.setText(findTestObject('Object Repository/Cheque_Book/Page_Login/input_User Name_j_username'), '1144')

WebUI.setText(findTestObject('Object Repository/Cheque_Book/Page_Login/input_Password_j_password_to_remove'), 'Test@123')

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Login/label_Login'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Company and Branch Login/input_Branch Code_branchesVO.BRANCH_CODE'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Company and Branch Login/span_Branch Code_ui-icon ui-icon-search liv_80748e'))

WebUI.setText(findTestObject('Object Repository/Cheque_Book/Page_Company and Branch Login/input_Brief Description_BRANCH_CODE'), 
    '900')

WebUI.doubleClick(findTestObject('Object Repository/Cheque_Book/Page_Company and Branch Login/td_900'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Company and Branch Login/span_Continue'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_iMAL Welcome Message/span_Continue'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_CSM CUSTOMER/span_Black List Management_ui-icon ui-icon-_ec660c'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_CSM CUSTOMER/a_Approve'))

WebUI.setText(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Approve/input_Status Iden_ACC_CIF'), 
    '10133876')

WebUI.sendKeys(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Approve/input_Status Iden_ACC_CIF'), 
    Keys.chord(Keys.ENTER))

WebUI.doubleClick(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Approve/td_10133876'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Approve/label_Approve'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Approve/input_ERROR IN RETRIEVING TELLER ACCOUNTPle_4daa29'))

WebUI.click(findTestObject('Object Repository/Cheque_Book/Page_Chequebook Request  Approve/label_Log Out'))

