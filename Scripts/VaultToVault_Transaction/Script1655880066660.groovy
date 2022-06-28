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

WebUI.click(findTestObject('Page_Login/Advanced'))

WebUI.click(findTestObject('Page_Login/Proceed'))

WebUI.setText(findTestObject('Object Repository/VaultToVault_page/Page_Login/input_User Name_j_username'), '1281')

WebUI.setText(findTestObject('Object Repository/VaultToVault_page/Page_Login/input_Password_j_password_to_remove'), 'Test@456')

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Login/label_Login'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_iMAL Welcome Message/span_Continue'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_CSM CUSTOMER/td_Transfer Cash'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_CSM CUSTOMER/td_Transfer Cash From Vault to Vault'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_CSM CUSTOMER/span_Maintenance'))

WebUI.click(findTestObject('VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/SearchIcon_Vault'))

WebUI.doubleClick(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/td_1'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/span_To Branch Code_ui-icon ui-icon-search _94c493'))

WebUI.setText(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/input_Remark_BRIEF_DESC_ENG'), 
    'musc')

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/td_MUSCAT MAIN BRANCH'))

WebUI.doubleClick(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/td_MUSCAT MAIN BRANCH'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/span__ui-icon ui-icon-search live-search-ui_1'))

WebUI.doubleClick(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/td_'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/span__ui-icon ui-icon-search live-search-ui_1_2'))

WebUI.doubleClick(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/td_Vault To Vault'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/td_Decimal Points_add_transferCashSubCashGr_9b8bba'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/td__1'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/span_Decimal Points_ui-icon ui-icon-search _47f4ce'))

WebUI.click(findTestObject('VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/input_Unit_BRIEF_DESC_ENG'))

WebUI.setText(findTestObject('VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/input_Unit_BRIEF_DESC_ENG'), 
    'om')

WebUI.doubleClick(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/td_OMR'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/a_Denomination'))

WebUI.click(findTestObject('VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/Bills_Td'))

WebUI.setText(findTestObject('VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/NoOfBills'), '1')

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/span_ok'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/span_Save'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_457913/label_Log Out'))

WebUI.setText(findTestObject('Page_Login/input_User Name_j_username'), '1219')

WebUI.setText(findTestObject('Page_Login/input_Password_j_password_to_remove'), 'Test@123')

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Login/label_Login'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_iMAL Welcome Message/span_Continue'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_CSM CUSTOMER/td_Transfer Cash'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_CSM CUSTOMER/td_Transfer Cash From Vault to Vault'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_CSM CUSTOMER/span_Approve'))

WebUI.setText(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_ab19f6/input_Trade Date_statusDesc'), 
    'Active')

WebUI.sendKeys(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_ab19f6/input_Trade Date_statusDesc'), 
    Keys.chord(Keys.ENTER))

WebUI.doubleClick(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_ab19f6/td_1'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_ab19f6/span_Approve'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_ab19f6/input_1905 Operation generated Transaction _8d9e4b'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_ab19f6/input_1908 Print Credit and Debit Advice__p_3cab3a'))

WebUI.click(findTestObject('Object Repository/VaultToVault_page/Page_Transfer Cash  Transfer Cash From Vaul_ab19f6/label_Log Out'))

