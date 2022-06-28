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

WebUI.setText(findTestObject('Object Repository/VaultToVault_Reverse/Page_Login/input_User Name_j_username'), '1281')

WebUI.setText(findTestObject('Object Repository/VaultToVault_Reverse/Page_Login/input_Password_j_password_to_remove'), 'Test@456')

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_Login/label_Login'))

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_iMAL Welcome Message/span_Continue'))

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_CSM CUSTOMER/h3_Transfer Cash'))

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_CSM CUSTOMER/td_Transfer Cash From Vault to Vault'))

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_Transfer Cash  Transfer Cash From Vaul_e80c25/span_Apply To Reverse'))

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_Transfer Cash  Transfer Cash From Vaul_864f76/input_Trade Date_statusDesc'))

WebUI.doubleClick(findTestObject('Object Repository/VaultToVault_Reverse/Page_Transfer Cash  Transfer Cash From Vaul_864f76/td_19216'))

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_Transfer Cash  Transfer Cash From Vaul_864f76/span_To Be Reversed'))

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_Transfer Cash  Transfer Cash From Vaul_864f76/label_Log Out'))

WebUI.setText(findTestObject('Object Repository/VaultToVault_Reverse/Page_Login/input_User Name_j_username'), '1219')

WebUI.setText(findTestObject('Object Repository/VaultToVault_Reverse/Page_Login/input_Password_j_password_to_remove'), 'Test@123')

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_Login/label_Login'))

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_iMAL Welcome Message/span_Continue'))

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_CSM CUSTOMER/span_Recording of Remittance_ui-icon ui-ico_84a6bf'))

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_CSM CUSTOMER/td_Transfer Cash From Vault to Vault'))

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_CSM CUSTOMER/span_Reverse'))

WebUI.doubleClick(findTestObject('Object Repository/VaultToVault_Reverse/Page_Transfer Cash  Transfer Cash From Vaul_e80c25/td_Apply To Reverse'))

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_Transfer Cash  Transfer Cash From Vaul_e80c25/span_Reverse'))

WebUI.click(findTestObject('Object Repository/VaultToVault_Reverse/Page_Transfer Cash  Transfer Cash From Vaul_e80c25/input_1908 Print Credit and Debit Advice__p_3cab3a'))

WebUI.click(findTestObject('Cheque_Book/Page_Chequebook Request  Maintenance/label_Log Out'))

