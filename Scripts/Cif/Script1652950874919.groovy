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

WebUI.setText(findTestObject('Object Repository/Page_Login/input_User Name_j_username (1)'), '1276')

WebUI.setText(findTestObject('Object Repository/Page_Login/input_Password_j_password_to_remove (1)'), 'Test@123')

WebUI.click(findTestObject('Object Repository/Page_Login/td_Login'))

WebUI.click(findTestObject('Object Repository/Page_Company and Branch Login/span_Continue'))

WebUI.click(findTestObject('Object Repository/Page_iMAL Welcome Message/span_Continue'))

WebUI.refresh()

WebUI.click(findTestObject('Object Repository/Page_CSM CUSTOMER/td_Front Office Management'))

WebUI.click(findTestObject('Object Repository/Page_CSM CUSTOMER/span_Maintenance'))

WebUI.waitForPageLoad(3)

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/span__ui-icon ui-icon-search live-search-ui'))

WebUI.delay(3)

WebUI.doubleClick(findTestObject('Object Repository/Page_Front Office Management  Maintenance/td_Value'))

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/span__ui-icon ui-icon-search live-search-ui_1'))

WebUI.delay(3)

WebUI.doubleClick(findTestObject('Object Repository/Page_Front Office Management  Maintenance/td_Passport'))

WebUI.setText(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input__fomCO.cifVO.ID_NO'), '22243429445395')

WebUI.delay(3)

WebUI.click(findTestObject('Page_Front Office Management  Maintenance/input__fomCO.cifVO.BIRTH_DATE'))

WebUI.setText(findTestObject('Page_Front Office Management  Maintenance/input__fomCO.cifVO.BIRTH_DATE'), '15/05/1997')

WebUI.setText(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input__fomCO.cifVO.ID_EXPIRY_DATE'), 
    '15/05/2029')

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/span__ui-icon ui-icon-search live-search-ui_1_2'))

WebUI.doubleClick(findTestObject('Object Repository/Page_Front Office Management  Maintenance/td_KAZ'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Front Office Management  Maintenance/select_SingleDivorcedWidowedMarriedUnknown'), 
    'S', true)

WebUI.setText(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input_Short Name_fomCO.cifVO.SHORT_NAME_ENG'), 
    'Anu')

WebUI.setText(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input_Long Name_fomCO.cifVO.LONG_NAME_ENG'), 
    'Kuttan')

WebUI.setText(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input_Full Name_fomCO.cifExtendedVO.FULL_NAME_ENG'), 
    'anukuttan')

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/span_Region_ui-icon ui-icon-search live-search-ui'))

WebUI.doubleClick(findTestObject('Object Repository/Page_Front Office Management  Maintenance/td_Muscat'))

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/span_Place of Birth_ui-icon ui-icon-search _e958c0'))

WebUI.doubleClick(findTestObject('Object Repository/Page_Front Office Management  Maintenance/td_Muscat_1'))

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/span_'))

WebUI.setText(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input_Address Description_addressCO.ADDRESS_DESC'), 
    'XYZ')

WebUI.setText(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input_House No  Flat No_addressCO.ADDRESS1_ENG'), 
    'XYZ')

WebUI.setText(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input_Building Name  Number_addressCO.ADDRESS2_ENG'), 
    'XYZ')

WebUI.setText(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input_Way No_addressCO.ADDRESS3_ENG'), 
    '123')

WebUI.setText(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input_Area  Willayat_addressCO.ADDRESS4_ENG'), 
    'xyz')

WebUI.setText(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input_City_addressCO.CITY_ENG'), 
    'ass')

WebUI.setText(findTestObject('Page_Front Office Management  Maintenance/input_Phone No Res_addressCO.TEL'), '12213455')

WebUI.setText(findTestObject('Page_Front Office Management  Maintenance/input_Office Phone No_addressCO.WORK_TEL'), '22332434')

WebUI.setText(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input_Mobile No (1)_addressCO.MOBILE'), 
    '(00968)-(22332434)')

WebUI.setText(findTestObject('Page_Front Office Management  Maintenance/input_Mobile No (2)_addressCO.OTHER_TEL'), '23453456')

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/span_PO BOX Area_ui-icon ui-icon-search liv_e29427'))

WebUI.doubleClick(findTestObject('Object Repository/Page_Front Office Management  Maintenance/td_Muscat_1_2'))

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/span_Postal Code_ui-icon ui-icon-search liv_dc6434'))

WebUI.doubleClick(findTestObject('Object Repository/Page_Front Office Management  Maintenance/td_Central Post Office'))

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/label_Save'))

WebUI.click(findTestObject('Page_Front Office Management  Maintenance/OK_Button'))

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/button_FATCA'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Front Office Management  Maintenance/select_YesNo'), '0', 
    true)

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/span_OK'))

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/button_PEP'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Front Office Management  Maintenance/select_YesNo_1'), 
    '0', true)

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/span_OK'))

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/label_Validate'))

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input_CONFIRM__popup_path_sol_confirm_ok'))

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/input_Postal Code__popup_path_sol_ok'))

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/span_Dismiss'))

WebUI.click(findTestObject('Object Repository/Page_Front Office Management  Maintenance/td_Log Out'))

WebUI.setText(findTestObject('Object Repository/ApproveCif/Page_Login/input_User Name_j_username'), '1144')

WebUI.setText(findTestObject('Object Repository/ApproveCif/Page_Login/input_Password_j_password_to_remove'), 'Test@123')

WebUI.click(findTestObject('Object Repository/ApproveCif/Page_Login/label_Login'))

WebUI.setText(findTestObject('Object Repository/ApproveCif/Page_Company and Branch Login/input_Branch Code_branchesVO.BRANCH_CODE'), 
    '900')

WebUI.click(findTestObject('Object Repository/ApproveCif/Page_Company and Branch Login/span_Continue'))

WebUI.click(findTestObject('Object Repository/ApproveCif/Page_iMAL Welcome Message/span_Continue'))

WebUI.click(findTestObject('Object Repository/ApproveCif/Page_CSM CUSTOMER/td_Front Office Management'))

WebUI.click(findTestObject('Object Repository/ApproveCif/Page_CSM CUSTOMER/span_Approve'))

WebUI.setText(findTestObject('Object Repository/ApproveCif/Page_Front Office Management  Approve/input__BRANCH_CODE'), '900')

WebUI.sendKeys(findTestObject('Object Repository/ApproveCif/Page_Front Office Management  Approve/input__BRANCH_CODE'), 
    Keys.chord(Keys.ENTER))

WebUI.setText(findTestObject('Object Repository/ApproveCif/Page_Front Office Management  Approve/input__SHORT_NAME_ENG'), 
    'Anu')

WebUI.sendKeys(findTestObject('Object Repository/ApproveCif/Page_Front Office Management  Approve/input__SHORT_NAME_ENG'), 
    Keys.chord(Keys.ENTER))

WebUI.doubleClick(findTestObject('Object Repository/ApproveCif/Page_Front Office Management  Approve/td_900'))

WebUI.click(findTestObject('Object Repository/ApproveCif/Page_Front Office Management  Approve/label_Approve'))

WebUI.click(findTestObject('Object Repository/ApproveCif/Page_Front Office Management  Approve/input_Confirm Approve Process__popup_path_s_8a6091'))

WebUI.click(findTestObject('Object Repository/ApproveCif/Page_Front Office Management  Approve/input_Record was Approved Successfully__pop_5bae63'))

WebUI.click(findTestObject('Page_Front Office Management  Maintenance/td_Log Out'))

