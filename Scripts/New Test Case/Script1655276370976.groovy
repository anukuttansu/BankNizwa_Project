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

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Login/input_User Name_j_username'))

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Login/input_User Name_j_username'), '1276')

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Login/input_Password_j_password_to_remove'), 'Test@123')

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Login/td_Login'))

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Company and Branch Login/span_Continue'))

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_iMAL Welcome Message/span_Continue'))

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_CSM CUSTOMER/h3_Front Office Management'))

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_CSM CUSTOMER/a_Maintenance'))

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input__fomCO.cifVO.CIF_TYPE'), 
    '1')

WebUI.click(findTestObject('CIF_individual/Page_Front Office Management  Maintenance/input__fomCO.cifVO.ID_TYPE'))

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input__fomCO.cifVO.ID_TYPE'), 
    '3')

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input__fomCO.cifVO.ID_NO'), 
    '234546666')

WebUI.click(findTestObject('CIF_individual/Page_Front Office Management  Maintenance/input__fomCO.cifVO.BIRTH_DATE'))

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input__fomCO.cifVO.BIRTH_DATE'), 
    '15/04/1998')

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input__fomCO.cifVO.ID_EXPIRY_DATE'), 
    '14/09/2030')

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/span__ui-icon ui-icon-search live-search-ui'))

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input_ISO Code_BRIEF_DESC_ENG'), 
    'om')

WebUI.doubleClick(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/td_OMN'))

WebUI.selectOptionByValue(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/select_SingleDivorcedWidowedMarriedUnknown'), 
    'S', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/select_FemaleMaleNA'), 
    'M', true)

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input_Short Name_fomCO.cifVO.SHORT_NAME_ENG'), 
    'Test')

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input_Long Name_fomCO.cifVO.LONG_NAME_ENG'), 
    '1')

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input_Full Name_fomCO.cifExtendedVO.FULL_NAME_ENG'), 
    'Test1')

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input_Address Description_addressCO.ADDRESS_DESC'), 
    'dsd')

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input_House No  Flat No_addressCO.ADDRESS1_ENG'), 
    'er')

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input_Building Name  Number_addressCO.ADDRESS2_ENG'), 
    'df')

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input_Way No_addressCO.ADDRESS3_ENG'), 
    'df')

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input_Phone No Res_addressCO.TEL'))

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input_Office Phone No_addressCO.WORK_TEL'))

WebUI.setText(findTestObject('Page_Front Office Management  Maintenance/input_Mobile No (1)_addressCO.MOBILE'), '1232134567855')

WebUI.setText(findTestObject('Page_Front Office Management  Maintenance/input_Mobile No (2)_addressCO.OTHER_TEL'), '12345678')

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/span_PO BOX Area_ui-icon ui-icon-search liv_e29427'))

WebUI.doubleClick(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/td_Muscat'))

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/span_Postal Code_ui-icon ui-icon-search liv_dc6434'))

WebUI.doubleClick(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/td_Central Post Office'))

WebUI.setText(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/textarea_Email_addressCO.EMAIL'), 
    'anu@gmail.com')

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/span_Save'))

WebUI.verifyElementPresent(findTestObject('Page_Front Office Management  Maintenance/div_CIF No 10133983 Saved Successfully'), 
    0)

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input_2046 Telephone Details are missing__p_0335d7'))

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/button_FATCA'))

WebUI.selectOptionByValue(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/select_YesNo'), 
    '0', true)

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/span_OK'))

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/button_PEP'))

WebUI.selectOptionByValue(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/select_YesNo_1'), 
    '0', true)

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/label_OK'))

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/label_Validate'))

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input_CONFIRM__popup_path_sol_confirm_ok'))

WebUI.verifyElementPresent(findTestObject('Page_Front Office Management  Maintenance/div_Record was Validated Successfully'), 
    0)

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/input_2046 Telephone Details are missing__p_0335d7'))

WebUI.click(findTestObject('Object Repository/CIF_individual/Page_Front Office Management  Maintenance/span_Dismiss'))

