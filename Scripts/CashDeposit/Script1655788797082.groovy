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

WebUI.setText(findTestObject('Object Repository/CashDepositPage/Page_Login/input_User Name_j_username'), '1371')

WebUI.setText(findTestObject('Object Repository/CashDepositPage/Page_Login/input_Password_j_password_to_remove'), 'Test123@')

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Login/td_Login'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_iMAL Welcome Message/span_Continue'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_CSM CUSTOMER/td_Transactions'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_CSM CUSTOMER/span_Maintenance'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/span_TRX Type_ui-icon ui-icon-search live-s_a76d60'))

WebUI.setText(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/input_Reference_SHORT_DESC_ENG'), 
    'cash de')

WebUI.doubleClick(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/td_Cash Deposit'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/span__ui-icon ui-icon-search live-search-ui'))

WebUI.setText(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/input_Status_amfVO.CIF_SUB_NO'), 
    '10133986')

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/td_512'))

WebUI.doubleClick(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/td_512'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/input__trxMgntCO.ctstrsVO.AMOUNT'))

WebUI.setText(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/input__trxMgntCO.ctstrsVO.AMOUNT'), 
    '100,000.000')

WebUI.sendKeys(findTestObject('CashDepositPage/Page_Transactions  Maintenance/input__trxMgntCO.ctstrsVO.AMOUNT'), Keys.chord(
        Keys.ENTER))

WebUI.click(findTestObject('CashDepositPage/Page_CSM CUSTOMER/Details_Field'))

WebUI.delay(6)

WebUI.click(findTestObject('CashDepositPage/Page_CSM CUSTOMER/button_ok'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/span_Additional Fields'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/input_ID Number_trxMgntCO.ctstrsVO.DEPOSITER_ID'))

WebUI.setText(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/input__additionalFieldsByTypeCOList0.ADD_VC'), 
    '04181228')

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/div_Additional Fieldsclose'))

WebUI.setText(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/input__additionalFieldsByTypeCOList0.ADD_VC'), 
    '04181228')

WebUI.setText(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/input__additionalFieldsByTypeCOList1.ADD_VC'), 
    'Anu')

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/span_ok'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/span_Save'))

WebUI.delay(5)

WebUI.click(findTestObject('CashDepositPage/Page_CSM CUSTOMER/Button_ok_Final'))

WebUI.click(findTestObject('CashDepositPage/Page_CSM CUSTOMER/button_ok'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/span_Dismiss'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Maintenance/label_Log Out'))

WebUI.setText(findTestObject('Object Repository/CashDepositPage/Page_Login/input_User Name_j_username'), '1392')

WebUI.setText(findTestObject('Object Repository/CashDepositPage/Page_Login/input_Password_j_password_to_remove'), 'Test@456')

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Login/label_Login'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_iMAL Welcome Message/span_Continue'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_CSM CUSTOMER/span_Cards Management_ui-icon ui-icon-triangle-1-e'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_CSM CUSTOMER/span_Approve'))

WebUI.setText(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Approve/input_reference3_key_STATUS_DESC'), 
    'Active')

WebUI.sendKeys(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Approve/input_reference3_key_STATUS_DESC'), 
    Keys.chord(Keys.ENTER))

WebUI.doubleClick(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Approve/td_Active'))

WebUI.click(findTestObject('Object Repository/CashDepositPage/Page_Transactions  Approve/span_Approve'))

