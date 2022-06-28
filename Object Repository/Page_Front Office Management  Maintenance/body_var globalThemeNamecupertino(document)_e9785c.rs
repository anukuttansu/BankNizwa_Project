<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_var globalThemeNamecupertino(document)_e9785c</name>
   <tag></tag>
   <elementGuidId>c0c58a22-429c-4a61-a27e-d879ae744caa</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body.ui-layout-container</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>51c46fca-fc63-4676-86db-40d2bacca68c</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>ui-layout-container</value>
      <webElementGuid>3735ffe8-8950-48af-8041-f968aabbeb4a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>role</name>
      <type>Main</type>
      <value>application</value>
      <webElementGuid>5dffdcd7-b23c-4c62-8e08-4809ba8c48a6</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>           var globalThemeName='cupertino';$(document).ready(function(){$(&quot;#ui_bar_div&quot;).css(&quot;height&quot;,&quot;100%&quot;);$(&quot;#ui_bar_div&quot;).niceScroll()});                                    User    : 1477          06 March 2022       Company    : Bank Nizwa            Branch   : Head office             
.ui-autocomplete {
	max-height: 180px;
	overflow-y: auto;
	overflow-x: hidden;
}
                           var session_is_now_opened_key=document.getElementById(&quot;session_is_now_opened_key_id&quot;).value;var switch_company_key=document.getElementById(&quot;switch_company_key_id&quot;).value;var switch_branch_key=document.getElementById(&quot;switch_branch_key_id&quot;).value;var closing_session_key=document.getElementById(&quot;closing_session_key_id&quot;).value;var session_is_now_closed_key=document.getElementById(&quot;session_is_now_closed_key_id&quot;).value;var show_btn_key=document.getElementById(&quot;show_btn_key_id&quot;).value;
var assign_widget_key=document.getElementById(&quot;assign_widget_key_id&quot;).value;var usr_allowed_portlets_key=document.getElementById(&quot;usr_allowed_portlets_key_id&quot;).value;var dynclientparams_key=document.getElementById(&quot;dynclientparams_key_id&quot;).value;var dynclientparams_approve_key=document.getElementById(&quot;dynclientparams_approve_key_id&quot;).value;                      Full Screen                                  Options                                     Home                                Log Out                               Help                      Language          $.struts2_jquery.require(&quot;AppMainHeader.js&quot;,null,&quot;/imal_core_portal/common/js/customization/&quot;);   _menLayOut({currRTL:&quot;ltr&quot;});               var tech_det_log_level_key=document.getElementById(&quot;log_level_key_id&quot;).value;var saveLevelLabel=document.getElementById(&quot;saveLabel_id&quot;).value;var record_was_Updated_Successfully_tech_key=document.getElementById(&quot;record_was_Updated_Successfully_key_id&quot;).value;var info_msg_tech_title_key=document.getElementById(&quot;info_msg_title_id&quot;).value;var tech_det_system_properties_key=document.getElementById(&quot;system_properties_key&quot;).value;                          Change Password                     Switch Company                     Switch Branch                        Final Sign Off         $.struts2_jquery.require(&quot;PortalHeaderOptions.js&quot;,null,&quot;/imal_core_portal/common/js/portaloptions/&quot;);                  Menu Options      

		
	var actionName=&quot;generateMenuOnRequest&quot;;
	var targetName=&quot;content-container&quot;;
	var appName=&quot;RET&quot;;
	var id = &quot;appMenu&quot;;
	var params = &quot;actionName=&quot;+actionName+&quot;&amp;targetName=&quot;+targetName+&quot;&amp;appName=&quot;+appName+&quot;&amp;id=&quot;+id;
	params = returnEncryptedData(params);
	var url=actionName + &quot;?&quot; + params ;
	$(document).ready(function(){
	$.struts2_jquery.requireCss(&quot;/common/style/menu/AccordionMenu.css&quot;,'/imal_core_portal');
	if(document.dir == &quot;rtl&quot;)
	{
		$.struts2_jquery.requireCss(&quot;/common/style/menu/AccordionMenu-rtl.css&quot;,'/imal_core_portal');
	}
	ddaccordion.initRoot(id,url,false);
	
	$.subscribe('setTitle', function(event, data) 
		{
			var titleVal = event.originalEvent.target.text;
			$('title').remove();
			$('head').append('&lt;title>'+titleVal+'&lt;/title>')
		}
	);
	function setHeaderTitle(ele,event)
	{	
		var hrefTag=$(ele).find('a').get(0)
		var title = hrefTag.id
		$('title').remove();
		$('head').append('&lt;title>'+title+'&lt;/title>')
	}
	});
	
	function accordionReloadMenuItem(menuVar)
	{
		var theMenuVar = menuVar.replace(' ',''); 
		theMenuVar = menuVar.replace('-','_');
		var $menuElt;
		if(menuVar == &quot;ROOT&quot;)
		{
			ddaccordion.initRoot(id ,url,false);
		}
		else
		{
			if( $(&quot;.h3_&quot;+theMenuVar).html() != null) //means header level 
			{
				$menuElt = $(&quot;.h3_&quot;+theMenuVar)
			}
			else if($(&quot;.a_&quot;+theMenuVar).html() != null) //sublevel
			{
			 	$menuElt = $(&quot;.a_&quot;+theMenuVar);
			}
			if($menuElt != null &amp;&amp; $menuElt.html() != null)
			{
				$menuElt.data('ajaxinfo').status = &quot;none&quot;
				$menuElt.trigger(&quot;evt_accordion&quot;,[false,true]);
			}
		} 
	}
	

360 Degrees view		AccountsFixed Maturity AccountsFixed Maturity AccountsGeneral AccountsOpening	Maintenance	Close	Close With Transfer	Reject	Approve Reject	Suspend	Reactivate	Accounts ListAccounts ListPrint	Account Query	Front Office ManagementMaintenance	Suspend	Query	Print CIF Confirmation	Print Account Confirmation	Chequebook RequestChequebook RequestMemo DetailMemo DetailLost &amp; Found ManagementLost &amp; Found ManagementCards ManagementCards ManagementTransactionsTransactionsReportsReportsFile ManagementFile ManagementQueriesQueriesCertificatesCertificates
  


var app_main_title = &quot;CSM CUSTOMER&quot;;


var currency_denomination_key=&quot;Currency Denomination&quot;;


var View_Chart_key=&quot;View Chart&quot;;
var Safebox_key=&quot;Safebox&quot;;
var Cannot_be_less_than_zero_key=&quot;Cannot be less than zero&quot;;
var Should_be_between_0_and_100_key=&quot;Should be between 0.0  and 100&quot;;
var Percentage_between_0_and_100_key=&quot;Percentage Should be between 0  and 100&quot;;
var ToDate_Cannot_be_less_then_FromDate_key=&quot;To Date Cannot be Less Than From Date&quot;;
var Retirement_Age_less_zero_key=&quot;Retirement Age Can Not be Less than 0&quot;;
var Other_Income_less_zero_key=&quot;Other Income Can Not be Less than 0&quot;;
var Annual_Salary_less_zero_key=&quot;Annual Salary Can Not be Less than 0&quot;;
var Monthly_Salary_less_zero_key=&quot;Monthly Basic Salary Can Not be Less than 0&quot;;
var Transportation_Fees_less_zero_key=&quot;Transportation Fees Can Not be Less than 0&quot;;
var Location_Fees_less_zero_key=&quot;Location Fees Can Not be Less than 0&quot;;
var Other_Fees_less_zero_key=&quot;Other Fees Can Not be Less than 0&quot;;

var cannot_be_less_than_or_equal_zero_key = &quot;Value cannot be less than or equal to zero&quot;;
var  keyNum_not_equal_oldValue  = &quot;Key number cannot be equal to the old value&quot;;


var CIF_Products_key = &quot;CIF Products&quot;;



var account_No = &quot;Account No&quot;;



var Please_Select_One_Record_key =&quot;Please Select One and Only One Record&quot;;
var file_has_been_automatically_saved_key = &quot;file has been automatically saved&quot;;
var Please_Select_Records_key =&quot;Please Select Records to Delete&quot;;


var The_CIF_is_blacklisted_key = &quot;The CIF is blacklisted&quot;;
var tax_regulation_status_key = &quot;Tax Regulation Status&quot;;
var inserted_data_will_be_cleared_do_you_wish_to_proceed_key = &quot;Inserted data will be cleared, do you wish to proceed&quot;;
var Passport_Info_key = &quot;Passport Info&quot;;
var residency_information_key = &quot;Residency Information&quot;;
var tax_payer_information_key = &quot;Tax Payer Information&quot;;
var telephone_info_key 		  = &quot;Telephone Information&quot;;
var compfinancialfigures_key = &quot;Company Financial Figures&quot;;

var Links_Management_key= &quot;Links Management&quot;;

var Account_Details_key=&quot;Account Details&quot;;
var chqbrkey=&quot;Chq Br&quot;;
var chqbknumkey = &quot;Chqbk Num&quot;;
var fillfromtochqnumfilterkey = &quot;Please fill From Cheque Number and To Cheque Number to proceed with filtering &quot;;
var invalidmissingchqnumkey = &quot;Invalid/Missing cheque number&quot;;
var tochqnumgreaterfromchqnumkey = &quot;To Chq Num should be greater than From Chq Num&quot;;
var pleaseselcheqtocancel = &quot;Please select the chequebook(s) to cancel&quot;;
var pleasefillcancelreasonallrows = &quot;Please fill cancel reason for all rows&quot;;

     var RTL_DIRECTION=&quot;ltr&quot;;$(document).ready(function(){intializeMainTabs(&quot;mainTabs&quot;,{url:jQuery.contextPath+&quot;/path/loadScreen?&quot;,reloadAlert:&quot;Do you need to reload the tab contents of&quot;,closeAlert:&quot;Close&quot;})});     Front Office Management / MaintenanceRemove Tab              
      Create Customer - Individual           F00I1MT       Smart           Audit                        Record               Record Log                 Entity               Entity log           Record        Search       New                 Loading...CIF   Branch Code CIF No Status Short Name Long Name Full Name Additional Reference Tel Eco Sector Relation Code Country Id No CIF Type One Obligor Email Id Gender Card Name Nick Name الإسم المختصر بالعربية الإسم الكامل بالعربية الاسم المفصّل ToggleClearPage  of 05101520No records to view


        
.hyperlink {font-style: italic !important; font-weight:bold !important; text-decoration:underline !important; cursor: pointer !important;}
                                                                                  
                                     CIF No    
   Branch     

   
   Additional Reference    
   Status    
    Status                 Reason     

   
   Score Value    
   
  Score       More Scores  



  FICA List   

           Reserved CIF  



    Segment Code     

   
    cif_segment_details_key  



        Reason    
                
Name &amp; AddressAdditional DetailsReferencesMarket DetailsAdditional FieldsCIF BIC CodeFund Transfer
 
 
 
 
 
 
 
        

   CIF Details 
    CIF Type *   

   
   The CIF is    CorporateIndividual

    Subscription To  



                             ID Type*   

     
   
 Id No     ANDORAND/OR

 join_type_key         Joint ID    

   
   
    IDs  



    Joint IDs  



     ID Delivery Date    
   Place Of Issue     

   
     ID Type 2    

   
   
    Reset  



     ID Delivered By       			             Date of Birth*
   			             ID Expiry Date*
   

 

 

   Biometric Approve       Country Of Issuance    

   
       Country Of Issuance *   

   
       Marital Status*   SingleDivorcedWidowedMarriedUnknown

     Gender*   FemaleMaleN/A

      Language   ArabicBothEnglish

             Intraday-Excess Limit    
                 

 

 

 Watch List          

 

 

 cif_minor_key      cif_minor_type_key    

   
       Number of Partners    
    
   CIF Brief and Long Name 
    Salutation    

   
        Nationality   

   
                  Short Name    
    Country      

   
     Long Name   
    Place of Birth     

   
      
   الإسم المختصر بالعربية   Race    

   
     Full Name    
    Region    

   
     Religion    

   
      
   الإسم الكامل بالعربية        
   الاسم المفصّل       Nick Name    
      Card Name   
    Joint    
    
   Name Details 
  
   Passport Info 
  
   CIF Address 
                                   

 

 

   Print Statement                 Address Description    
     Contact Name    
     Salutation    
     House No / Flat No   
     Building Name / Number   
     Way No   
     Area / Willayat   
     Street Details    
     City   
     Country    
                 إسم الإتصلات    
     الحالة    
        
         
        
        
     تفاصيل عن الشارع    
        
     البلد    
                   Phone No Res.     
     Office Phone No   
     Mobile No (1)       
     Mobile No (2)   
        
     Email           verify_email_key  



   

 

 

 email_verified_key      Telex                       

   
     Country   

   
        

   
     Sector   

   
     PO BOX Area    

   
      
      Postal Code     

    
     Address Description   

   
     Proof of Address    

   
                       Expiry Date   
   From Date    
   To Date    
   
	  
	  

       
 Permanent Address 
   

 

   
	  
	  

       
 Default Address 
   

 

   
	  
	  

       
 Wallet 
   

 

              More Addresses 



      
   Statement 
    Category of Statement Delivery    PrintedElectronic

    Multi Statements  



     Mode of Stmt Delivery   Collect at BranchDirect MailEmailFaxNot RequiredP.O. BoxSMSTelexStatement for StaffSwift

   Statement   HoldYesNo

     
	  
	  

       
 Consolidate Same Reference 
   

 

     Description      Periodicity    QuarterlySemi-AnnuallyDailyFortnightlyWeeklyYearlyMonthly

     Status Reason            
  Tax Exemption 
   

 

 

 Exempted from Filer  Exempted from Filer Reason   

 
  Exemption Expiry Date  
    
      Status Reason          
          

 Additional Details 
    CIF Category    PotentialClientVIPNeither

    Bank Ref    
   

 

 

   Send SMS on Account Opening    

 

 

   Chat Bank    

 

 

   Exempt Full Charges Waiver      Vat Zone    

   
   

 

 

   Exempted From VAT         

 

 

   exmpted_frm_pyments_pntly_yn                   
	  
	  

       
 Exempted From zakat 
   

 

 Reason    

 
                        Country of Birth   

   
   Register No    
       Economic_Agent_key    

   
   Legal Status*   

   
     Economic Sector*    

   
   GIIN    
    Req Doc  



    Education  



     Sub Eco Sector    

   
   profession_category_key    

   
     profession_segment_key    

   
   Profession    

   
     Ranking *    

   
      Educational Level    

   
   Profession     EmployedSelf-EmployedUnemployed

      No of Dependents    
       One Obligor    

   
    Additional One-Obligors  



      CIF Profile    

   
   

 

 

 Allow to Exceed One Obligor/CIF Capital Limit     CIF Branches  



       Exclude TOB Charges    YESNO

     Filer/ Non-Filer:    FilerN/ANon Filer

     CIF Limit Cap    
   
     Web Site    
   Proof of Occupation    

   
       Source of Funds    

   
   Source of Funds Description  
      Occupation    

   
   Position    

   
    Occupation  



     Employer CIF/Name     

   
   Monthly Income    
           Employment Status    

   
            Division *   

   
   Resident       ResidentNon Resident

      Department   

   
   Residence Type    

   
      Legal Action    
   Populate CIF    NoYes

   KYC     Not RequiredRecalcitrantCompleted By CallCompletedOutstanding

       KYC Details hidden for customization  



      BVN    
       CIF is Complete    YesNo

      KYC Expiry Date    
   kycreviewdate_key    
        agent    

   
                   
   Authorized Signatories/Power Of Attorney 
    Name    
      Ownership Details  



    More Signatory  



     ID Type    

   
    Id No    
     Country Of Issuance *   

   
        
 Introducer Details 
  
     CIF Rating    

   
    ... 



   Relation    

   
     Forbid Transaction    NoneCreditDebitBoth

    Forbid Reason         tax_regime_key    

   
        Swift Compliant Address    
    Capital Amount   
     
   BKE for BICs    NoYes

     
   Central Bank Secret No.    
     
   License Expiry Date    
        Declared Income    
         Nationality Risk    YESNO

   Politically Exposed    YESNO

     Business Risk    YESNO

   Group ID    
    

  Mobile wallet 
   
	  
	  

       
 Register 
   

 

 registration_number_key  
 Nickname  
    wallet Account  
 Mobile No.  
   
 
     
        

   References Details 
  
   Relationship Officer Details 
     Officer ID    

   Officer Name    
    More Officers  



       Role Code    

   
        Division Code    

   
        Dept Code    

   
         Account Manager    

   Manager Name    
        Group    

   
        Team    

   
        
   
              
              Additional Fields  



    

   Save  

             
      


  

                     @-webkit-keyframes marqueeAnimation-199387  { 100%  {margin-left:-0px}} $.struts2_jquery.require(&quot;jquery.marquee.js&quot;,null,jQuery.contextPath+&quot;/common/jquery/js/marquee/&quot;);$.struts2_jquery.require(&quot;jquery.pause.js&quot;,null,jQuery.contextPath+&quot;/common/jquery/js/marquee/&quot;);if($(&quot;#scrollingMarquee&quot;))$(function(){$(&quot;#scrollingMarquee&quot;).marquee({duration:5E3})});      Copyright ©2022 Azentio ®All Rights Reserved    iMAL*CSM 14.6.0.0 Build 418   Powered By                                                                                                                                                                                                                                                                                                                 var view_Memo_key=document.getElementById(&quot;View_Memo_key_id&quot;).value;var Break_of_Deposit_key=document.getElementById(&quot;Break_of_Deposit_key_id&quot;).value;var cannot_be_less_than_or_equal_zero_key=document.getElementById(&quot;cannot_be_less_than_or_equal_zero_key_id&quot;).value;var teg_is_greater_than_tie_key=document.getElementById(&quot;teg_is_greater_than_tie_key_id&quot;).value;var tie_rate_exeeded_key=document.getElementById(&quot;tie_rate_exeeded_key_id&quot;).value;var current_yield_is_key=document.getElementById(&quot;current_yield_is_key_id&quot;).value;
var and_related_teg_is_key=document.getElementById(&quot;and_related_teg_is_key_id&quot;).value;var the_optimum_teg_rate_should_be_key=document.getElementById(&quot;the_optimum_teg_rate_should_be_key_id&quot;).value;var and_the_max_yield_should_be_key=document.getElementById(&quot;and_the_max_yield_should_be_key_id&quot;).value;var teg_is_greater_than_the_tie_after_adjusting_yield_charges=document.getElementById(&quot;teg_is_greater_than_the_tie_after_adjusting_yield_charges_id&quot;).value;var the_optimum_teg_rate_changed_to_key=document.getElementById(&quot;the_optimum_teg_rate_changed_to_key_id&quot;).value;
var and_the_max_yield_is_key=document.getElementById(&quot;and_the_max_yield_is_key_id&quot;).value;var Market_Value_key=document.getElementById(&quot;Market_Value_key_id&quot;).value;var Book_Value_key=document.getElementById(&quot;Book_Value_key_id&quot;).value;var app_title_not_defined=document.getElementById(&quot;app_title_not_defined_id&quot;).value;var grid_addLabel=document.getElementById(&quot;grid_addLabel_id&quot;).value;var grid_editLabel=document.getElementById(&quot;grid_editLabel_id&quot;).value;var grid_delLabel=document.getElementById(&quot;grid_delLabel_id&quot;).value;
var grid_approveLabel=document.getElementById(&quot;grid_approveLabel_id&quot;).value;var grid_rejectLabel=document.getElementById(&quot;grid_rejectLabel_id&quot;).value;var grid_createFromLabel=document.getElementById(&quot;grid_createFromLabel_id&quot;).value;var saveLabel=document.getElementById(&quot;saveLabel_id&quot;).value;var msg_progDefaultLabel=document.getElementById(&quot;msg_progDefaultLabel_id&quot;).value;var Change_running_date_key=document.getElementById(&quot;Change_running_date_key_id&quot;).value;
var SMART_System_Maintenance_Additional_Reference_Table_key=document.getElementById(&quot;SMART_System_Maintenance_Additional_Reference_Table_key_id&quot;).value;var Smart_Details_not_defined_key=document.getElementById(&quot;Smart_Details_not_defined_key_id&quot;).value;var SMART_must_be_related_to_a_record_key=document.getElementById(&quot;SMART_must_be_related_to_a_record_key_id&quot;).value;var Error_retrieving_file_from_database_key=document.getElementById(&quot;Error_retrieving_file_from_database_key_id&quot;).value;
var Preview_image_key=document.getElementById(&quot;Preview_image_key_id&quot;).value;var Mandatory_Field_key=document.getElementById(&quot;Mandatory_Field_key_id&quot;).value;var msg_noRecordSelectedLabel=document.getElementById(&quot;msg_noRecordSelectedLabel_id&quot;).value;var Total_Amount_key=document.getElementById(&quot;Total_Amount_key_id&quot;).value;var Warning_key=document.getElementById(&quot;Warning_key_id&quot;).value;var total_All=document.getElementById(&quot;total_All_id&quot;).value;var Confirm_Approve_Process_key=document.getElementById(&quot;Confirm_Approve_Process_key_id&quot;).value;
var Confirm_Reject_Process_key=document.getElementById(&quot;Confirm_Reject_Process_key_id&quot;).value;var Confirm_Reinstate_Process_key=document.getElementById(&quot;Confirm_Reinstate_Process_key_id&quot;).value;var Confirm_Reactivate_Process_key=document.getElementById(&quot;Confirm_Reactivate_Process_key_id&quot;).value;var Confirm_Suspend_Process_key=document.getElementById(&quot;Confirm_Suspend_Process_key_id&quot;).value;var Confirm_Proceed_key=document.getElementById(&quot;Confirm_Proceed_key_id&quot;).value;
var Confirm_Delete_Process_key=document.getElementById(&quot;Confirm_Delete_Process_key_id&quot;).value;var confirm_Export_Process_key=document.getElementById(&quot;Confirm_Export_Process_key_id&quot;).value;var Confirm_Save_Process_key=document.getElementById(&quot;Confirm_Save_Process_key&quot;).value;var date_of_birth_label=document.getElementById(&quot;date_of_birth_label_id&quot;).value;var birthDate_label=document.getElementById(&quot;birthDate_label_id&quot;).value;var estbDate_label=document.getElementById(&quot;estbDate_label_id&quot;).value;
var error_msg_title=document.getElementById(&quot;error_msg_title_id&quot;).value;var success_msg_title=document.getElementById(&quot;success_msg_title_id&quot;).value;var confirm_msg_title=document.getElementById(&quot;confirm_msg_title_id&quot;).value;var warning_msg_title=document.getElementById(&quot;warning_msg_title_id&quot;).value;var info_msg_title=document.getElementById(&quot;info_msg_title_id&quot;).value;var proceed_msg_title=document.getElementById(&quot;proceed_msg_title_id&quot;).value;var changes_made_confirm_msg=document.getElementById(&quot;changes_made_confirm_msg_id&quot;).value;
var changes_not_available_key=document.getElementById(&quot;changes_not_available_key_id&quot;).value;var missing_selection_type_key=document.getElementById(&quot;missing_selection_type_key_id&quot;).value;var cache_cleared_success_key=document.getElementById(&quot;cache_cleared_success_key_id&quot;).value;var close_all_opened_screen_key=document.getElementById(&quot;close_all_opened_screen_key_id&quot;).value;var period_label=document.getElementById(&quot;period_label_id&quot;).value;var creditRating_label=document.getElementById(&quot;creditRating_label_id&quot;).value;
var trx_type_label=document.getElementById(&quot;trx_type_label_id&quot;).value;var reason_code_label=document.getElementById(&quot;reason_code_label_id&quot;).value;var charges_schema_details_label=document.getElementById(&quot;charges_schema_details_label_id&quot;).value;var cash_account_label=document.getElementById(&quot;cash_account_label_id&quot;).value;var missing_elt_msg_key=document.getElementById(&quot;missing_elt_msg_key_id&quot;).value;var account_Position_key=document.getElementById(&quot;account_Position_key_id&quot;).value;
var relogin_btn_key=document.getElementById(&quot;relogin_btn_key_id&quot;).value;var fill_old_pwd_msg_key=document.getElementById(&quot;fill_old_pwd_msg_key_id&quot;).value;var fill_both_msg_key=document.getElementById(&quot;fill_both_msg_key_id&quot;).value;var charges_Details_key=document.getElementById(&quot;charges_Details_key_id&quot;).value;var choose_a_schema_first_msg=document.getElementById(&quot;choose_a_schema_first_msg_id&quot;).value;var calculation_done_msg=document.getElementById(&quot;calculation_done_msg_id&quot;).value;
var charges_Schema_key=document.getElementById(&quot;charges_Schema_key_id&quot;).value;var account_Type_key=document.getElementById(&quot;account_Type_key_id&quot;).value;var acc_nb_key=document.getElementById(&quot;acc_nb_key_id&quot;).value;var record_was_Approved_Successfully_key=document.getElementById(&quot;record_was_Approved_Successfully_key_id&quot;).value;var record_was_Suspended_Successfully_key=document.getElementById(&quot;record_was_Suspended_Successfully_key_id&quot;).value;var record_was_Deleted_Successfully_key=document.getElementById(&quot;record_was_Deleted_Successfully_key_id&quot;).value;
var record_was_Updated_Successfully_key=document.getElementById(&quot;record_was_Updated_Successfully_key_id&quot;).value;var record_was_Reinstated_Successfully_key=document.getElementById(&quot;record_was_Reinstated_Successfully_key_id&quot;).value;var record_was_Validated_Successfully_key=document.getElementById(&quot;record_was_Validated_Successfully_key_id&quot;).value;var record_saved_Successfully_key=document.getElementById(&quot;record_saved_Successfully_key_id&quot;).value;var Authorized_Suspended_Successfully_key=document.getElementById(&quot;Authorized_Suspended_Successfully_key_id&quot;).value;
var record_reactivated_successfully_key=document.getElementById(&quot;record_reactivated_successfully_key_id&quot;).value;var authorized_reactivated_successfully_key=document.getElementById(&quot;authorized_reactivated_successfully_key_id&quot;).value;var record_rejected_Successfully_key=document.getElementById(&quot;record_rejected_Successfully_key_id&quot;).value;var authorized_rejected_successfully_key=document.getElementById(&quot;authorized_rejected_successfully_key_id&quot;).value;var record_closed_successfully_key=document.getElementById(&quot;record_closed_successfully_key_id&quot;).value;
var authorized_closed_successfully_key=document.getElementById(&quot;authorized_closed_successfully_key_id&quot;).value;var record_was_Authorized_Successfully_key=document.getElementById(&quot;record_was_Authorized_Successfully_key_id&quot;).value;var themes_transl=document.getElementById(&quot;themes_transl_id&quot;).value;var ok_label_trans=document.getElementById(&quot;ok_label_trans_id&quot;).value;var reset_label_trans=document.getElementById(&quot;reset_label_trans_id&quot;).value;var forward_label_trans=document.getElementById(&quot;forward_label_trans_id&quot;).value;
var print_label_trans=document.getElementById(&quot;print_label_trans_id&quot;).value;var dismiss_label_trans=document.getElementById(&quot;dismiss_label_trans_id&quot;).value;var openItem_label_trans=document.getElementById(&quot;openItem_label_trans_id&quot;).value;var cancel_label_trans=document.getElementById(&quot;cancel_label_trans_id&quot;).value;var max_val_trans=document.getElementById(&quot;max_val_trans_id&quot;).value;var min_val_trans=document.getElementById(&quot;min_val_trans_id&quot;).value;var max_length_trans=document.getElementById(&quot;max_length_trans_id&quot;).value;
var min_length_trans=document.getElementById(&quot;min_length_trans_id&quot;).value;var limit_key=document.getElementById(&quot;limit_key_id&quot;).value;var transaction_key=document.getElementById(&quot;transaction_key_id&quot;).value;var trx_Type_Charges_trans=document.getElementById(&quot;trx_Type_Charges_trans_id&quot;).value;var Trx_No_key=document.getElementById(&quot;Trx_No_key_id&quot;).value;var void_Reason_Charges_trans=document.getElementById(&quot;void_Reason_Charges_trans_id&quot;).value;var msg_duplicate_entry_Of_record_key=document.getElementById(&quot;msg_duplicate_entry_Of_record_key_id&quot;).value;
var msg_please_fill_Trx_Type_Code_for_already_added_record=document.getElementById(&quot;msg_please_fill_Trx_Type_Code_for_already_added_record_id&quot;).value;var msg_please_fill_Charge_Code_for_already_added_record=document.getElementById(&quot;msg_please_fill_Charge_Code_for_already_added_record_id&quot;).value;var msg_please_fill_Reason_Code_for_already_added_record=document.getElementById(&quot;msg_please_fill_Reason_Code_for_already_added_record_id&quot;).value;var msg_please_fill_Currency_for_already_added_record=document.getElementById(&quot;msg_please_fill_Currency_for_already_added_record_id&quot;).value;
var msg_please_fill_Limit_for_already_added_record=document.getElementById(&quot;msg_please_fill_Limit_for_already_added_record_id&quot;).value;var msg_please_fill_Account_Type_for_already_added_record=document.getElementById(&quot;msg_please_fill_Account_Type_for_already_added_record_id&quot;).value;var msg_please_fill_Code_for_added_record=document.getElementById(&quot;msg_please_fill_Code_for_added_record_id&quot;).value;var msg_please_fill_Line_No_for_already_added_record=document.getElementById(&quot;msg_please_fill_Line_No_for_already_added_record_id&quot;).value;
var msg_please_fill_Credit_Rating_for_already_added_record=document.getElementById(&quot;msg_please_fill_Credit_Rating_for_already_added_record_id&quot;).value;var msg_please_fill_From_Date_for_already_added_record=document.getElementById(&quot;msg_please_fill_From_Date_for_already_added_record_id&quot;).value;var msg_please_fill_To_Date_for_already_added_record=document.getElementById(&quot;msg_please_fill_To_Date_for_already_added_record_id&quot;).value;var msg_please_add_Account_Type_for_already_added_record=document.getElementById(&quot;msg_please_add_Account_Type_for_already_added_record_id&quot;).value;
var msg_enter_CIF_before_Proceeding=document.getElementById(&quot;msg_enter_CIF_before_Proceeding_id&quot;).value;var dr_suffix_key_trans=document.getElementById(&quot;dr_suffix_key_trans_id&quot;).value;var cr_suffix_key_trans=document.getElementById(&quot;cr_suffix_key_trans_id&quot;).value;var send_alert_title=document.getElementById(&quot;send_alert_title_id&quot;).value;var approve_item_key=document.getElementById(&quot;approve_item_key_id&quot;).value;var usr_inf_key=document.getElementById(&quot;usr_inf_key_id&quot;).value;var modify_button=document.getElementById(&quot;modify_button_id&quot;).value;
var open_multiple_item_confirmation=document.getElementById(&quot;open_multiple_item_confirmation_id&quot;).value;var ack_ok_warning=document.getElementById(&quot;ack_ok_warning_id&quot;).value;var trs_timeout_openitem_warning=document.getElementById(&quot;trs_timeout_openitem_warning_id&quot;).value;var void_button=document.getElementById(&quot;void_button_id&quot;).value;var scan_button=document.getElementById(&quot;scan_button_id&quot;).value;var deduct_Charges_From_Account_key=document.getElementById(&quot;deduct_Charges_From_Account_key_id&quot;).value;
var drawdown_key=document.getElementById(&quot;drawdown_key_id&quot;).value;var investment_Account_key=document.getElementById(&quot;investment_Account_key_id&quot;).value;var More_Addresses_key=document.getElementById(&quot;More_Addresses_key_id&quot;).value;var Current_Account_key=document.getElementById(&quot;Current_Account_key_id&quot;).value;var Additional_Fields_key=document.getElementById(&quot;Additional_Fields_key_id&quot;).value;var error_IRIS_Enrol_Key=document.getElementById(&quot;error_IRIS_Enrol_Key_id&quot;).value;var Choose_customer_key=document.getElementById(&quot;Choose_customer_key_id&quot;).value;
var Clear_customer_key=document.getElementById(&quot;Clear_customer_key_id&quot;).value;var imgNotAvailable=document.getElementById(&quot;imgNotAvailable_id&quot;).value;var Print_Swift_key=document.getElementById(&quot;Print_Swift_key_id&quot;).value;var Print_Swift_must_be_related_to_a_record_key=document.getElementById(&quot;Print_Swift_must_be_related_to_a_record_key_id&quot;).value;var signature_popup_title=document.getElementById(&quot;signature_popup_title_id&quot;).value;var signature_close_btn=document.getElementById(&quot;signature_close_btn_id&quot;).value;
var signature_action_title=document.getElementById(&quot;signature_action_title_id&quot;).value;var no_appr_sign_msg=document.getElementById(&quot;no_appr_sign_msg_id&quot;).value;var sign_signature_msg=document.getElementById(&quot;sign_signature_msg_id&quot;).value;var gen_ann_saved_msg=document.getElementById(&quot;gen_ann_saved_msg_id&quot;).value;var desc_mandatory_msg=document.getElementById(&quot;desc_mandatory_msg_id&quot;).value;var choose_doc_msg=document.getElementById(&quot;choose_doc_msg_id&quot;).value;var sign_copied=document.getElementById(&quot;sign_copied_id&quot;).value;
var signature_key=document.getElementById(&quot;signature_key_id&quot;).value;var jvs_list_key=document.getElementById(&quot;jvs_list_key_id&quot;).value;var Accept_key=document.getElementById(&quot;Accept_key_id&quot;).value;var Reject_Deleted_key=document.getElementById(&quot;Reject_Deleted_key_id&quot;).value;var Reject_Active_Deleted_key=document.getElementById(&quot;Reject_Active_Deleted_key_id&quot;).value;var english_key=document.getElementById(&quot;english_key_id&quot;).value;var arabic_key=document.getElementById(&quot;arabic_key_id&quot;).value;
var french_key=document.getElementById(&quot;french_key_id&quot;).value;var choose_language_key=document.getElementById(&quot;choose_language_key_id&quot;).value;var status_list_key=document.getElementById(&quot;status_list_key_id&quot;).value;var stat_of_account_key=document.getElementById(&quot;stat_of_account_key_id&quot;).value;var advice_key=document.getElementById(&quot;advice_key_id&quot;).value;var opening_acc_key=document.getElementById(&quot;opening_acc_key_id&quot;).value;var specify_report_key=document.getElementById(&quot;specify_report_key_id&quot;).value;
var whiteListed_key=document.getElementById(&quot;whiteListed_key_id&quot;).value;var company_key=document.getElementById(&quot;company_key_id&quot;).value;var branch_key=document.getElementById(&quot;branch_key_id&quot;).value;var cif_key=document.getElementById(&quot;cif_key_id&quot;).value;var acc_key=document.getElementById(&quot;acc_key_id&quot;).value;var savedMsg=document.getElementById(&quot;savedMsg_id&quot;).value;var show_picture_title=document.getElementById(&quot;show_picture_title_id&quot;).value;var No_Data_Found_key=document.getElementById(&quot;No_Data_Found_key_id&quot;).value;
var details_key=document.getElementById(&quot;details_key_id&quot;).value;var Repayment_Plan_details_not_available_key=document.getElementById(&quot;Repayment_Plan_details_not_available_key_id&quot;).value;var alerts_key=document.getElementById(&quot;alerts_key_id&quot;).value;var suspicious_list_key=document.getElementById(&quot;suspicious_list_key_id&quot;).value;var passbook_pages_exhausted_key=document.getElementById(&quot;passbook_pages_exhausted_key_id&quot;).value;var passbook_flip_pages_key=document.getElementById(&quot;passbook_flip_pages_key_id&quot;).value;
var brought_forward_amount_key=document.getElementById(&quot;brought_forward_amount_key_id&quot;).value;var Missing_File_Code_key=document.getElementById(&quot;Missing_File_Code_key_id&quot;).value;var File_Names_Do_Not_Match_key=document.getElementById(&quot;File_Names_Do_Not_Match_key_id&quot;).value;var Process_Executed_Successfully_key=document.getElementById(&quot;Process_Executed_Successfully_key_id&quot;).value;var File_Input_Parameters_Mandatory_key=document.getElementById(&quot;File_Input_Parameters_Mandatory_key_id&quot;).value;
var Input_Tag_values_Mandatory_key=document.getElementById(&quot;Input_Tag_values_Mandatory_key_id&quot;).value;var Missing_File_Location_key=document.getElementById(&quot;Missing_File_Location_key_id&quot;).value;var dyn_import_File_Loc_key=document.getElementById(&quot;Missing_File_Location_key_id&quot;).value;var fav_maint_title_key=document.getElementById(&quot;fav_maint_title_key_id&quot;).value;var select_fav_screen_key=document.getElementById(&quot;select_fav_screen_key_id&quot;).value;var selection_leaf_node_key=document.getElementById(&quot;selection_leaf_node_key_id&quot;).value;
var app_infos_title_key=document.getElementById(&quot;app_infos_title_key_id&quot;).value;var confirm_del_widget_key=document.getElementById(&quot;confirm_del_widget_key_id&quot;).value;var widget_not_avail_key=document.getElementById(&quot;widget_not_avail_key_id&quot;).value;var drag_widget_key=document.getElementById(&quot;drag_widget_key_id&quot;).value;var loading_key=document.getElementById(&quot;loading_key_id&quot;).value;var app_not_found_key=document.getElementById(&quot;app_not_found_key_id&quot;).value;var no_alert_found_key=document.getElementById(&quot;no_alert_found_key_id&quot;).value;
var resize_widget_key=document.getElementById(&quot;resize_widget_key_id&quot;).value;var custom_wkspce_key=document.getElementById(&quot;custom_wkspce_key_id&quot;).value;var add_custom_widget_key=document.getElementById(&quot;add_custom_widget_key_id&quot;).value;var trans_key=document.getElementById(&quot;trans_key_id&quot;).value;var branch_is_now_opened_key=document.getElementById(&quot;branch_is_now_opened_key_id&quot;).value;var branch_is_now_closed_key=document.getElementById(&quot;branch_is_now_closed_key&quot;).value;var no_widgets_key=document.getElementById(&quot;no_widgets_key_id&quot;).value;
var change_pass=document.getElementById(&quot;change_pass_id&quot;).value;var description_label_key=document.getElementById(&quot;description_label_key_id&quot;).value;var select_screen_help_key=document.getElementById(&quot;select_screen_help_key_id&quot;).value;var record_lvl_att_key=document.getElementById(&quot;record_lvl_att_key_id&quot;).value;var record_lvl_att_wlog_key=document.getElementById(&quot;record_lvl_att_wlog_key_id&quot;).value;var entity_lvl_att_key=document.getElementById(&quot;entity_lvl_att_key_id&quot;).value;
var entity_lvl_att_wlog_key=document.getElementById(&quot;entity_lvl_att_wlog_key_id&quot;).value;var record_Logs_Key=document.getElementById(&quot;Record_Logs_Key_id&quot;).value;var for_key=document.getElementById(&quot;for_key_id&quot;).value;var toolTipTrans_key=document.getElementById(&quot;toolTipTrans_key_id&quot;).value;var grid_user_pref_key=document.getElementById(&quot;grid_user_pref_key_id&quot;).value;var tentative_Schedule_Created_msg_key=document.getElementById(&quot;tentative_Schedule_Created_msg_key_id&quot;).value;
var finalSignOffChecking_key=document.getElementById(&quot;finalSignOffChecking_key_id&quot;).value;var report_Return_Data_key=document.getElementById(&quot;report_Return_Data_key_id&quot;).value;var server_request_completes_key=document.getElementById(&quot;server_request_completes_key_id&quot;).value;var download_key=document.getElementById(&quot;download_key_id&quot;).value;var elemIdAndNameChecking_key=document.getElementById(&quot;elemIdAndNameChecking_key_id&quot;).value;var Selected_key=document.getElementById(&quot;Selected_key_id&quot;).value;
var oldValKey=document.getElementById(&quot;oldValKey_id&quot;).value;var grid_user_def_key=document.getElementById(&quot;grid_user_def_key_id&quot;).value;var status_cust_key=document.getElementById(&quot;status_cust_key_id&quot;).value;var token_key=document.getElementById(&quot;token_key_id&quot;).value;var closeDynScreen_key=document.getElementById(&quot;closeDynScreen_key_id&quot;).value;var smartLabel_title_key=document.getElementById(&quot;smartLabel_title_key_id&quot;).value;var January_key=document.getElementById(&quot;January_key_id&quot;).value;
var February_key=document.getElementById(&quot;February_key_id&quot;).value;var March_key=document.getElementById(&quot;March_key_id&quot;).value;var April_key=document.getElementById(&quot;April_key_id&quot;).value;var May_key=document.getElementById(&quot;May_key_id&quot;).value;var June_key=document.getElementById(&quot;June_key_id&quot;).value;var July_key=document.getElementById(&quot;July_key_id&quot;).value;var August_key=document.getElementById(&quot;August_key_id&quot;).value;var September_key=document.getElementById(&quot;September_key_id&quot;).value;
var October_key=document.getElementById(&quot;October_key_id&quot;).value;var November_key=document.getElementById(&quot;November_key_id&quot;).value;var December_key=document.getElementById(&quot;December_key_id&quot;).value;var invalid_widget_url=document.getElementById(&quot;invalid_widget_url_key&quot;).value;var server_conn_drop_key=document.getElementById(&quot;server_conn_drop_key_id&quot;).value;var yes_confirm=document.getElementById(&quot;yes_key_id&quot;).value;var no_confirm=document.getElementById(&quot;no_key_id&quot;).value;
var defEmptyRepConfirm_key=document.getElementById(&quot;defemptyrepconfirm_key&quot;).value;var defEmptyRep_key=document.getElementById(&quot;defemptyrep_key&quot;).value;var activex_disabled_key=document.getElementById(&quot;activex_disabled_key_id&quot;).value;var cannotProceed_key=document.getElementById(&quot;cannot_proceed_key&quot;).value;var hijriDate_cust_key=document.getElementById(&quot;hijriDate_cust_key&quot;).value;var view_grid_key=document.getElementById(&quot;view_grid_key&quot;).value;var live_search_key=document.getElementById(&quot;live_search_key&quot;).value;
var Confirm_Reverse_Process_key=document.getElementById(&quot;Confirm_Reverse_Process_key_id&quot;).value;var notifications_key=document.getElementById(&quot;notifications_key&quot;).value;var function_not_defined_key=document.getElementById(&quot;function_not_defined_key_id&quot;).value;var your_selection_will_be_lost_key=document.getElementById(&quot;your_selection_will_be_lost_key_id&quot;).value;var cannot_open_dyn_screen_key=document.getElementById(&quot;cannot_open_dyn_screen_key&quot;).value;var contact_screen_admin_key=document.getElementById(&quot;contact_screen_admin_key_id&quot;).value;
var component_not_defined_key=document.getElementById(&quot;component_not_defined_key_id&quot;).value;var link_dyn_scr_key=document.getElementById(&quot;link_dyn_scr_key_id&quot;).value;var file_key=document.getElementById(&quot;file_key&quot;).value;var line_key=document.getElementById(&quot;line_key&quot;).value;var column_key=document.getElementById(&quot;column_key&quot;).value;var scrn_doesnt_contain_components_key=document.getElementById(&quot;scrn_doesnt_contain_components_key&quot;).value;var digital_chnl_trans_key=document.getElementById(&quot;digital_chnl_trans_key_id&quot;).value;
var report_key=document.getElementById(&quot;report_key_trans_id&quot;).value;var report_error_key=document.getElementById(&quot;report_error_key_trans_id&quot;).value;var duplicate_elem_id_key=document.getElementById(&quot;duplicate_elem_id&quot;).value;var old_value_key=document.getElementById(&quot;old_value_key_id&quot;).value;  Menu Options/html[1]/body[@class=&quot;ui-layout-container&quot;]/div[@class=&quot;displayProgBar&quot;]/div[@class=&quot;ui-widget-shadow progBarBackGound&quot;]WarningPlease, select rowPrevNextMay 2022SuMoTuWeThFrSa12345678910111213141516171819202122232425262728293031    Reset User PreferencesAdd New TabWarningPlease, select rowLoading... Code Brief Description Long Description الوصف المختصر الوصف الكامل Additional Reference1ValueValueValueValueR12Value JointValue JointValue JointValue JointR23MinorMinorMinorMinorR34SpecialSpecialSpecialSpecialR45Special JointSpecial JointSpecial JointSpecial JointR5 Page  of 65101520View1-5of28WarningPlease, select rowLoading... Code Brief Description الوصف المختصر Code1National IDالبطاقة الشخصية 3Passportجواز سفر 5Resident IDبطاقه المقيم 7Com. Reg.تسجيل الشركة 8Other IDبطاقة أخرى  Page  of 35101520View1-5of12</value>
      <webElementGuid>1bf19b25-1f63-4776-90fe-004cff79cb3e</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;ui-layout-container&quot;]</value>
      <webElementGuid>cf758ed4-a7ed-4601-ba06-a364b4c98cad</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>b99c11cc-a226-4103-8107-aeb48665d25d</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;           var globalThemeName=&quot; , &quot;'&quot; , &quot;cupertino&quot; , &quot;'&quot; , &quot;;$(document).ready(function(){$(&quot;#ui_bar_div&quot;).css(&quot;height&quot;,&quot;100%&quot;);$(&quot;#ui_bar_div&quot;).niceScroll()});                                    User    : 1477          06 March 2022       Company    : Bank Nizwa            Branch   : Head office             
.ui-autocomplete {
	max-height: 180px;
	overflow-y: auto;
	overflow-x: hidden;
}
                           var session_is_now_opened_key=document.getElementById(&quot;session_is_now_opened_key_id&quot;).value;var switch_company_key=document.getElementById(&quot;switch_company_key_id&quot;).value;var switch_branch_key=document.getElementById(&quot;switch_branch_key_id&quot;).value;var closing_session_key=document.getElementById(&quot;closing_session_key_id&quot;).value;var session_is_now_closed_key=document.getElementById(&quot;session_is_now_closed_key_id&quot;).value;var show_btn_key=document.getElementById(&quot;show_btn_key_id&quot;).value;
var assign_widget_key=document.getElementById(&quot;assign_widget_key_id&quot;).value;var usr_allowed_portlets_key=document.getElementById(&quot;usr_allowed_portlets_key_id&quot;).value;var dynclientparams_key=document.getElementById(&quot;dynclientparams_key_id&quot;).value;var dynclientparams_approve_key=document.getElementById(&quot;dynclientparams_approve_key_id&quot;).value;                      Full Screen                                  Options                                     Home                                Log Out                               Help                      Language          $.struts2_jquery.require(&quot;AppMainHeader.js&quot;,null,&quot;/imal_core_portal/common/js/customization/&quot;);   _menLayOut({currRTL:&quot;ltr&quot;});               var tech_det_log_level_key=document.getElementById(&quot;log_level_key_id&quot;).value;var saveLevelLabel=document.getElementById(&quot;saveLabel_id&quot;).value;var record_was_Updated_Successfully_tech_key=document.getElementById(&quot;record_was_Updated_Successfully_key_id&quot;).value;var info_msg_tech_title_key=document.getElementById(&quot;info_msg_title_id&quot;).value;var tech_det_system_properties_key=document.getElementById(&quot;system_properties_key&quot;).value;                          Change Password                     Switch Company                     Switch Branch                        Final Sign Off         $.struts2_jquery.require(&quot;PortalHeaderOptions.js&quot;,null,&quot;/imal_core_portal/common/js/portaloptions/&quot;);                  Menu Options      

		
	var actionName=&quot;generateMenuOnRequest&quot;;
	var targetName=&quot;content-container&quot;;
	var appName=&quot;RET&quot;;
	var id = &quot;appMenu&quot;;
	var params = &quot;actionName=&quot;+actionName+&quot;&amp;targetName=&quot;+targetName+&quot;&amp;appName=&quot;+appName+&quot;&amp;id=&quot;+id;
	params = returnEncryptedData(params);
	var url=actionName + &quot;?&quot; + params ;
	$(document).ready(function(){
	$.struts2_jquery.requireCss(&quot;/common/style/menu/AccordionMenu.css&quot;,&quot; , &quot;'&quot; , &quot;/imal_core_portal&quot; , &quot;'&quot; , &quot;);
	if(document.dir == &quot;rtl&quot;)
	{
		$.struts2_jquery.requireCss(&quot;/common/style/menu/AccordionMenu-rtl.css&quot;,&quot; , &quot;'&quot; , &quot;/imal_core_portal&quot; , &quot;'&quot; , &quot;);
	}
	ddaccordion.initRoot(id,url,false);
	
	$.subscribe(&quot; , &quot;'&quot; , &quot;setTitle&quot; , &quot;'&quot; , &quot;, function(event, data) 
		{
			var titleVal = event.originalEvent.target.text;
			$(&quot; , &quot;'&quot; , &quot;title&quot; , &quot;'&quot; , &quot;).remove();
			$(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;title>&quot; , &quot;'&quot; , &quot;+titleVal+&quot; , &quot;'&quot; , &quot;&lt;/title>&quot; , &quot;'&quot; , &quot;)
		}
	);
	function setHeaderTitle(ele,event)
	{	
		var hrefTag=$(ele).find(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;).get(0)
		var title = hrefTag.id
		$(&quot; , &quot;'&quot; , &quot;title&quot; , &quot;'&quot; , &quot;).remove();
		$(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;title>&quot; , &quot;'&quot; , &quot;+title+&quot; , &quot;'&quot; , &quot;&lt;/title>&quot; , &quot;'&quot; , &quot;)
	}
	});
	
	function accordionReloadMenuItem(menuVar)
	{
		var theMenuVar = menuVar.replace(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;); 
		theMenuVar = menuVar.replace(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot;);
		var $menuElt;
		if(menuVar == &quot;ROOT&quot;)
		{
			ddaccordion.initRoot(id ,url,false);
		}
		else
		{
			if( $(&quot;.h3_&quot;+theMenuVar).html() != null) //means header level 
			{
				$menuElt = $(&quot;.h3_&quot;+theMenuVar)
			}
			else if($(&quot;.a_&quot;+theMenuVar).html() != null) //sublevel
			{
			 	$menuElt = $(&quot;.a_&quot;+theMenuVar);
			}
			if($menuElt != null &amp;&amp; $menuElt.html() != null)
			{
				$menuElt.data(&quot; , &quot;'&quot; , &quot;ajaxinfo&quot; , &quot;'&quot; , &quot;).status = &quot;none&quot;
				$menuElt.trigger(&quot;evt_accordion&quot;,[false,true]);
			}
		} 
	}
	

360 Degrees view		AccountsFixed Maturity AccountsFixed Maturity AccountsGeneral AccountsOpening	Maintenance	Close	Close With Transfer	Reject	Approve Reject	Suspend	Reactivate	Accounts ListAccounts ListPrint	Account Query	Front Office ManagementMaintenance	Suspend	Query	Print CIF Confirmation	Print Account Confirmation	Chequebook RequestChequebook RequestMemo DetailMemo DetailLost &amp; Found ManagementLost &amp; Found ManagementCards ManagementCards ManagementTransactionsTransactionsReportsReportsFile ManagementFile ManagementQueriesQueriesCertificatesCertificates
  


var app_main_title = &quot;CSM CUSTOMER&quot;;


var currency_denomination_key=&quot;Currency Denomination&quot;;


var View_Chart_key=&quot;View Chart&quot;;
var Safebox_key=&quot;Safebox&quot;;
var Cannot_be_less_than_zero_key=&quot;Cannot be less than zero&quot;;
var Should_be_between_0_and_100_key=&quot;Should be between 0.0  and 100&quot;;
var Percentage_between_0_and_100_key=&quot;Percentage Should be between 0  and 100&quot;;
var ToDate_Cannot_be_less_then_FromDate_key=&quot;To Date Cannot be Less Than From Date&quot;;
var Retirement_Age_less_zero_key=&quot;Retirement Age Can Not be Less than 0&quot;;
var Other_Income_less_zero_key=&quot;Other Income Can Not be Less than 0&quot;;
var Annual_Salary_less_zero_key=&quot;Annual Salary Can Not be Less than 0&quot;;
var Monthly_Salary_less_zero_key=&quot;Monthly Basic Salary Can Not be Less than 0&quot;;
var Transportation_Fees_less_zero_key=&quot;Transportation Fees Can Not be Less than 0&quot;;
var Location_Fees_less_zero_key=&quot;Location Fees Can Not be Less than 0&quot;;
var Other_Fees_less_zero_key=&quot;Other Fees Can Not be Less than 0&quot;;

var cannot_be_less_than_or_equal_zero_key = &quot;Value cannot be less than or equal to zero&quot;;
var  keyNum_not_equal_oldValue  = &quot;Key number cannot be equal to the old value&quot;;


var CIF_Products_key = &quot;CIF Products&quot;;



var account_No = &quot;Account No&quot;;



var Please_Select_One_Record_key =&quot;Please Select One and Only One Record&quot;;
var file_has_been_automatically_saved_key = &quot;file has been automatically saved&quot;;
var Please_Select_Records_key =&quot;Please Select Records to Delete&quot;;


var The_CIF_is_blacklisted_key = &quot;The CIF is blacklisted&quot;;
var tax_regulation_status_key = &quot;Tax Regulation Status&quot;;
var inserted_data_will_be_cleared_do_you_wish_to_proceed_key = &quot;Inserted data will be cleared, do you wish to proceed&quot;;
var Passport_Info_key = &quot;Passport Info&quot;;
var residency_information_key = &quot;Residency Information&quot;;
var tax_payer_information_key = &quot;Tax Payer Information&quot;;
var telephone_info_key 		  = &quot;Telephone Information&quot;;
var compfinancialfigures_key = &quot;Company Financial Figures&quot;;

var Links_Management_key= &quot;Links Management&quot;;

var Account_Details_key=&quot;Account Details&quot;;
var chqbrkey=&quot;Chq Br&quot;;
var chqbknumkey = &quot;Chqbk Num&quot;;
var fillfromtochqnumfilterkey = &quot;Please fill From Cheque Number and To Cheque Number to proceed with filtering &quot;;
var invalidmissingchqnumkey = &quot;Invalid/Missing cheque number&quot;;
var tochqnumgreaterfromchqnumkey = &quot;To Chq Num should be greater than From Chq Num&quot;;
var pleaseselcheqtocancel = &quot;Please select the chequebook(s) to cancel&quot;;
var pleasefillcancelreasonallrows = &quot;Please fill cancel reason for all rows&quot;;

     var RTL_DIRECTION=&quot;ltr&quot;;$(document).ready(function(){intializeMainTabs(&quot;mainTabs&quot;,{url:jQuery.contextPath+&quot;/path/loadScreen?&quot;,reloadAlert:&quot;Do you need to reload the tab contents of&quot;,closeAlert:&quot;Close&quot;})});     Front Office Management / MaintenanceRemove Tab              
      Create Customer - Individual           F00I1MT       Smart           Audit                        Record               Record Log                 Entity               Entity log           Record        Search       New                 Loading...CIF   Branch Code CIF No Status Short Name Long Name Full Name Additional Reference Tel Eco Sector Relation Code Country Id No CIF Type One Obligor Email Id Gender Card Name Nick Name الإسم المختصر بالعربية الإسم الكامل بالعربية الاسم المفصّل ToggleClearPage  of 05101520No records to view


        
.hyperlink {font-style: italic !important; font-weight:bold !important; text-decoration:underline !important; cursor: pointer !important;}
                                                                                  
                                     CIF No    
   Branch     

   
   Additional Reference    
   Status    
    Status                 Reason     

   
   Score Value    
   
  Score       More Scores  



  FICA List   

           Reserved CIF  



    Segment Code     

   
    cif_segment_details_key  



        Reason    
                
Name &amp; AddressAdditional DetailsReferencesMarket DetailsAdditional FieldsCIF BIC CodeFund Transfer
 
 
 
 
 
 
 
        

   CIF Details 
    CIF Type *   

   
   The CIF is    CorporateIndividual

    Subscription To  



                             ID Type*   

     
   
 Id No     ANDORAND/OR

 join_type_key         Joint ID    

   
   
    IDs  



    Joint IDs  



     ID Delivery Date    
   Place Of Issue     

   
     ID Type 2    

   
   
    Reset  



     ID Delivered By       			             Date of Birth*
   			             ID Expiry Date*
   

 

 

   Biometric Approve       Country Of Issuance    

   
       Country Of Issuance *   

   
       Marital Status*   SingleDivorcedWidowedMarriedUnknown

     Gender*   FemaleMaleN/A

      Language   ArabicBothEnglish

             Intraday-Excess Limit    
                 

 

 

 Watch List          

 

 

 cif_minor_key      cif_minor_type_key    

   
       Number of Partners    
    
   CIF Brief and Long Name 
    Salutation    

   
        Nationality   

   
                  Short Name    
    Country      

   
     Long Name   
    Place of Birth     

   
      
   الإسم المختصر بالعربية   Race    

   
     Full Name    
    Region    

   
     Religion    

   
      
   الإسم الكامل بالعربية        
   الاسم المفصّل       Nick Name    
      Card Name   
    Joint    
    
   Name Details 
  
   Passport Info 
  
   CIF Address 
                                   

 

 

   Print Statement                 Address Description    
     Contact Name    
     Salutation    
     House No / Flat No   
     Building Name / Number   
     Way No   
     Area / Willayat   
     Street Details    
     City   
     Country    
                 إسم الإتصلات    
     الحالة    
        
         
        
        
     تفاصيل عن الشارع    
        
     البلد    
                   Phone No Res.     
     Office Phone No   
     Mobile No (1)       
     Mobile No (2)   
        
     Email           verify_email_key  



   

 

 

 email_verified_key      Telex                       

   
     Country   

   
        

   
     Sector   

   
     PO BOX Area    

   
      
      Postal Code     

    
     Address Description   

   
     Proof of Address    

   
                       Expiry Date   
   From Date    
   To Date    
   
	  
	  

       
 Permanent Address 
   

 

   
	  
	  

       
 Default Address 
   

 

   
	  
	  

       
 Wallet 
   

 

              More Addresses 



      
   Statement 
    Category of Statement Delivery    PrintedElectronic

    Multi Statements  



     Mode of Stmt Delivery   Collect at BranchDirect MailEmailFaxNot RequiredP.O. BoxSMSTelexStatement for StaffSwift

   Statement   HoldYesNo

     
	  
	  

       
 Consolidate Same Reference 
   

 

     Description      Periodicity    QuarterlySemi-AnnuallyDailyFortnightlyWeeklyYearlyMonthly

     Status Reason            
  Tax Exemption 
   

 

 

 Exempted from Filer  Exempted from Filer Reason   

 
  Exemption Expiry Date  
    
      Status Reason          
          

 Additional Details 
    CIF Category    PotentialClientVIPNeither

    Bank Ref    
   

 

 

   Send SMS on Account Opening    

 

 

   Chat Bank    

 

 

   Exempt Full Charges Waiver      Vat Zone    

   
   

 

 

   Exempted From VAT         

 

 

   exmpted_frm_pyments_pntly_yn                   
	  
	  

       
 Exempted From zakat 
   

 

 Reason    

 
                        Country of Birth   

   
   Register No    
       Economic_Agent_key    

   
   Legal Status*   

   
     Economic Sector*    

   
   GIIN    
    Req Doc  



    Education  



     Sub Eco Sector    

   
   profession_category_key    

   
     profession_segment_key    

   
   Profession    

   
     Ranking *    

   
      Educational Level    

   
   Profession     EmployedSelf-EmployedUnemployed

      No of Dependents    
       One Obligor    

   
    Additional One-Obligors  



      CIF Profile    

   
   

 

 

 Allow to Exceed One Obligor/CIF Capital Limit     CIF Branches  



       Exclude TOB Charges    YESNO

     Filer/ Non-Filer:    FilerN/ANon Filer

     CIF Limit Cap    
   
     Web Site    
   Proof of Occupation    

   
       Source of Funds    

   
   Source of Funds Description  
      Occupation    

   
   Position    

   
    Occupation  



     Employer CIF/Name     

   
   Monthly Income    
           Employment Status    

   
            Division *   

   
   Resident       ResidentNon Resident

      Department   

   
   Residence Type    

   
      Legal Action    
   Populate CIF    NoYes

   KYC     Not RequiredRecalcitrantCompleted By CallCompletedOutstanding

       KYC Details hidden for customization  



      BVN    
       CIF is Complete    YesNo

      KYC Expiry Date    
   kycreviewdate_key    
        agent    

   
                   
   Authorized Signatories/Power Of Attorney 
    Name    
      Ownership Details  



    More Signatory  



     ID Type    

   
    Id No    
     Country Of Issuance *   

   
        
 Introducer Details 
  
     CIF Rating    

   
    ... 



   Relation    

   
     Forbid Transaction    NoneCreditDebitBoth

    Forbid Reason         tax_regime_key    

   
        Swift Compliant Address    
    Capital Amount   
     
   BKE for BICs    NoYes

     
   Central Bank Secret No.    
     
   License Expiry Date    
        Declared Income    
         Nationality Risk    YESNO

   Politically Exposed    YESNO

     Business Risk    YESNO

   Group ID    
    

  Mobile wallet 
   
	  
	  

       
 Register 
   

 

 registration_number_key  
 Nickname  
    wallet Account  
 Mobile No.  
   
 
     
        

   References Details 
  
   Relationship Officer Details 
     Officer ID    

   Officer Name    
    More Officers  



       Role Code    

   
        Division Code    

   
        Dept Code    

   
         Account Manager    

   Manager Name    
        Group    

   
        Team    

   
        
   
              
              Additional Fields  



    

   Save  

             
      


  

                     @-webkit-keyframes marqueeAnimation-199387  { 100%  {margin-left:-0px}} $.struts2_jquery.require(&quot;jquery.marquee.js&quot;,null,jQuery.contextPath+&quot;/common/jquery/js/marquee/&quot;);$.struts2_jquery.require(&quot;jquery.pause.js&quot;,null,jQuery.contextPath+&quot;/common/jquery/js/marquee/&quot;);if($(&quot;#scrollingMarquee&quot;))$(function(){$(&quot;#scrollingMarquee&quot;).marquee({duration:5E3})});      Copyright ©2022 Azentio ®All Rights Reserved    iMAL*CSM 14.6.0.0 Build 418   Powered By                                                                                                                                                                                                                                                                                                                 var view_Memo_key=document.getElementById(&quot;View_Memo_key_id&quot;).value;var Break_of_Deposit_key=document.getElementById(&quot;Break_of_Deposit_key_id&quot;).value;var cannot_be_less_than_or_equal_zero_key=document.getElementById(&quot;cannot_be_less_than_or_equal_zero_key_id&quot;).value;var teg_is_greater_than_tie_key=document.getElementById(&quot;teg_is_greater_than_tie_key_id&quot;).value;var tie_rate_exeeded_key=document.getElementById(&quot;tie_rate_exeeded_key_id&quot;).value;var current_yield_is_key=document.getElementById(&quot;current_yield_is_key_id&quot;).value;
var and_related_teg_is_key=document.getElementById(&quot;and_related_teg_is_key_id&quot;).value;var the_optimum_teg_rate_should_be_key=document.getElementById(&quot;the_optimum_teg_rate_should_be_key_id&quot;).value;var and_the_max_yield_should_be_key=document.getElementById(&quot;and_the_max_yield_should_be_key_id&quot;).value;var teg_is_greater_than_the_tie_after_adjusting_yield_charges=document.getElementById(&quot;teg_is_greater_than_the_tie_after_adjusting_yield_charges_id&quot;).value;var the_optimum_teg_rate_changed_to_key=document.getElementById(&quot;the_optimum_teg_rate_changed_to_key_id&quot;).value;
var and_the_max_yield_is_key=document.getElementById(&quot;and_the_max_yield_is_key_id&quot;).value;var Market_Value_key=document.getElementById(&quot;Market_Value_key_id&quot;).value;var Book_Value_key=document.getElementById(&quot;Book_Value_key_id&quot;).value;var app_title_not_defined=document.getElementById(&quot;app_title_not_defined_id&quot;).value;var grid_addLabel=document.getElementById(&quot;grid_addLabel_id&quot;).value;var grid_editLabel=document.getElementById(&quot;grid_editLabel_id&quot;).value;var grid_delLabel=document.getElementById(&quot;grid_delLabel_id&quot;).value;
var grid_approveLabel=document.getElementById(&quot;grid_approveLabel_id&quot;).value;var grid_rejectLabel=document.getElementById(&quot;grid_rejectLabel_id&quot;).value;var grid_createFromLabel=document.getElementById(&quot;grid_createFromLabel_id&quot;).value;var saveLabel=document.getElementById(&quot;saveLabel_id&quot;).value;var msg_progDefaultLabel=document.getElementById(&quot;msg_progDefaultLabel_id&quot;).value;var Change_running_date_key=document.getElementById(&quot;Change_running_date_key_id&quot;).value;
var SMART_System_Maintenance_Additional_Reference_Table_key=document.getElementById(&quot;SMART_System_Maintenance_Additional_Reference_Table_key_id&quot;).value;var Smart_Details_not_defined_key=document.getElementById(&quot;Smart_Details_not_defined_key_id&quot;).value;var SMART_must_be_related_to_a_record_key=document.getElementById(&quot;SMART_must_be_related_to_a_record_key_id&quot;).value;var Error_retrieving_file_from_database_key=document.getElementById(&quot;Error_retrieving_file_from_database_key_id&quot;).value;
var Preview_image_key=document.getElementById(&quot;Preview_image_key_id&quot;).value;var Mandatory_Field_key=document.getElementById(&quot;Mandatory_Field_key_id&quot;).value;var msg_noRecordSelectedLabel=document.getElementById(&quot;msg_noRecordSelectedLabel_id&quot;).value;var Total_Amount_key=document.getElementById(&quot;Total_Amount_key_id&quot;).value;var Warning_key=document.getElementById(&quot;Warning_key_id&quot;).value;var total_All=document.getElementById(&quot;total_All_id&quot;).value;var Confirm_Approve_Process_key=document.getElementById(&quot;Confirm_Approve_Process_key_id&quot;).value;
var Confirm_Reject_Process_key=document.getElementById(&quot;Confirm_Reject_Process_key_id&quot;).value;var Confirm_Reinstate_Process_key=document.getElementById(&quot;Confirm_Reinstate_Process_key_id&quot;).value;var Confirm_Reactivate_Process_key=document.getElementById(&quot;Confirm_Reactivate_Process_key_id&quot;).value;var Confirm_Suspend_Process_key=document.getElementById(&quot;Confirm_Suspend_Process_key_id&quot;).value;var Confirm_Proceed_key=document.getElementById(&quot;Confirm_Proceed_key_id&quot;).value;
var Confirm_Delete_Process_key=document.getElementById(&quot;Confirm_Delete_Process_key_id&quot;).value;var confirm_Export_Process_key=document.getElementById(&quot;Confirm_Export_Process_key_id&quot;).value;var Confirm_Save_Process_key=document.getElementById(&quot;Confirm_Save_Process_key&quot;).value;var date_of_birth_label=document.getElementById(&quot;date_of_birth_label_id&quot;).value;var birthDate_label=document.getElementById(&quot;birthDate_label_id&quot;).value;var estbDate_label=document.getElementById(&quot;estbDate_label_id&quot;).value;
var error_msg_title=document.getElementById(&quot;error_msg_title_id&quot;).value;var success_msg_title=document.getElementById(&quot;success_msg_title_id&quot;).value;var confirm_msg_title=document.getElementById(&quot;confirm_msg_title_id&quot;).value;var warning_msg_title=document.getElementById(&quot;warning_msg_title_id&quot;).value;var info_msg_title=document.getElementById(&quot;info_msg_title_id&quot;).value;var proceed_msg_title=document.getElementById(&quot;proceed_msg_title_id&quot;).value;var changes_made_confirm_msg=document.getElementById(&quot;changes_made_confirm_msg_id&quot;).value;
var changes_not_available_key=document.getElementById(&quot;changes_not_available_key_id&quot;).value;var missing_selection_type_key=document.getElementById(&quot;missing_selection_type_key_id&quot;).value;var cache_cleared_success_key=document.getElementById(&quot;cache_cleared_success_key_id&quot;).value;var close_all_opened_screen_key=document.getElementById(&quot;close_all_opened_screen_key_id&quot;).value;var period_label=document.getElementById(&quot;period_label_id&quot;).value;var creditRating_label=document.getElementById(&quot;creditRating_label_id&quot;).value;
var trx_type_label=document.getElementById(&quot;trx_type_label_id&quot;).value;var reason_code_label=document.getElementById(&quot;reason_code_label_id&quot;).value;var charges_schema_details_label=document.getElementById(&quot;charges_schema_details_label_id&quot;).value;var cash_account_label=document.getElementById(&quot;cash_account_label_id&quot;).value;var missing_elt_msg_key=document.getElementById(&quot;missing_elt_msg_key_id&quot;).value;var account_Position_key=document.getElementById(&quot;account_Position_key_id&quot;).value;
var relogin_btn_key=document.getElementById(&quot;relogin_btn_key_id&quot;).value;var fill_old_pwd_msg_key=document.getElementById(&quot;fill_old_pwd_msg_key_id&quot;).value;var fill_both_msg_key=document.getElementById(&quot;fill_both_msg_key_id&quot;).value;var charges_Details_key=document.getElementById(&quot;charges_Details_key_id&quot;).value;var choose_a_schema_first_msg=document.getElementById(&quot;choose_a_schema_first_msg_id&quot;).value;var calculation_done_msg=document.getElementById(&quot;calculation_done_msg_id&quot;).value;
var charges_Schema_key=document.getElementById(&quot;charges_Schema_key_id&quot;).value;var account_Type_key=document.getElementById(&quot;account_Type_key_id&quot;).value;var acc_nb_key=document.getElementById(&quot;acc_nb_key_id&quot;).value;var record_was_Approved_Successfully_key=document.getElementById(&quot;record_was_Approved_Successfully_key_id&quot;).value;var record_was_Suspended_Successfully_key=document.getElementById(&quot;record_was_Suspended_Successfully_key_id&quot;).value;var record_was_Deleted_Successfully_key=document.getElementById(&quot;record_was_Deleted_Successfully_key_id&quot;).value;
var record_was_Updated_Successfully_key=document.getElementById(&quot;record_was_Updated_Successfully_key_id&quot;).value;var record_was_Reinstated_Successfully_key=document.getElementById(&quot;record_was_Reinstated_Successfully_key_id&quot;).value;var record_was_Validated_Successfully_key=document.getElementById(&quot;record_was_Validated_Successfully_key_id&quot;).value;var record_saved_Successfully_key=document.getElementById(&quot;record_saved_Successfully_key_id&quot;).value;var Authorized_Suspended_Successfully_key=document.getElementById(&quot;Authorized_Suspended_Successfully_key_id&quot;).value;
var record_reactivated_successfully_key=document.getElementById(&quot;record_reactivated_successfully_key_id&quot;).value;var authorized_reactivated_successfully_key=document.getElementById(&quot;authorized_reactivated_successfully_key_id&quot;).value;var record_rejected_Successfully_key=document.getElementById(&quot;record_rejected_Successfully_key_id&quot;).value;var authorized_rejected_successfully_key=document.getElementById(&quot;authorized_rejected_successfully_key_id&quot;).value;var record_closed_successfully_key=document.getElementById(&quot;record_closed_successfully_key_id&quot;).value;
var authorized_closed_successfully_key=document.getElementById(&quot;authorized_closed_successfully_key_id&quot;).value;var record_was_Authorized_Successfully_key=document.getElementById(&quot;record_was_Authorized_Successfully_key_id&quot;).value;var themes_transl=document.getElementById(&quot;themes_transl_id&quot;).value;var ok_label_trans=document.getElementById(&quot;ok_label_trans_id&quot;).value;var reset_label_trans=document.getElementById(&quot;reset_label_trans_id&quot;).value;var forward_label_trans=document.getElementById(&quot;forward_label_trans_id&quot;).value;
var print_label_trans=document.getElementById(&quot;print_label_trans_id&quot;).value;var dismiss_label_trans=document.getElementById(&quot;dismiss_label_trans_id&quot;).value;var openItem_label_trans=document.getElementById(&quot;openItem_label_trans_id&quot;).value;var cancel_label_trans=document.getElementById(&quot;cancel_label_trans_id&quot;).value;var max_val_trans=document.getElementById(&quot;max_val_trans_id&quot;).value;var min_val_trans=document.getElementById(&quot;min_val_trans_id&quot;).value;var max_length_trans=document.getElementById(&quot;max_length_trans_id&quot;).value;
var min_length_trans=document.getElementById(&quot;min_length_trans_id&quot;).value;var limit_key=document.getElementById(&quot;limit_key_id&quot;).value;var transaction_key=document.getElementById(&quot;transaction_key_id&quot;).value;var trx_Type_Charges_trans=document.getElementById(&quot;trx_Type_Charges_trans_id&quot;).value;var Trx_No_key=document.getElementById(&quot;Trx_No_key_id&quot;).value;var void_Reason_Charges_trans=document.getElementById(&quot;void_Reason_Charges_trans_id&quot;).value;var msg_duplicate_entry_Of_record_key=document.getElementById(&quot;msg_duplicate_entry_Of_record_key_id&quot;).value;
var msg_please_fill_Trx_Type_Code_for_already_added_record=document.getElementById(&quot;msg_please_fill_Trx_Type_Code_for_already_added_record_id&quot;).value;var msg_please_fill_Charge_Code_for_already_added_record=document.getElementById(&quot;msg_please_fill_Charge_Code_for_already_added_record_id&quot;).value;var msg_please_fill_Reason_Code_for_already_added_record=document.getElementById(&quot;msg_please_fill_Reason_Code_for_already_added_record_id&quot;).value;var msg_please_fill_Currency_for_already_added_record=document.getElementById(&quot;msg_please_fill_Currency_for_already_added_record_id&quot;).value;
var msg_please_fill_Limit_for_already_added_record=document.getElementById(&quot;msg_please_fill_Limit_for_already_added_record_id&quot;).value;var msg_please_fill_Account_Type_for_already_added_record=document.getElementById(&quot;msg_please_fill_Account_Type_for_already_added_record_id&quot;).value;var msg_please_fill_Code_for_added_record=document.getElementById(&quot;msg_please_fill_Code_for_added_record_id&quot;).value;var msg_please_fill_Line_No_for_already_added_record=document.getElementById(&quot;msg_please_fill_Line_No_for_already_added_record_id&quot;).value;
var msg_please_fill_Credit_Rating_for_already_added_record=document.getElementById(&quot;msg_please_fill_Credit_Rating_for_already_added_record_id&quot;).value;var msg_please_fill_From_Date_for_already_added_record=document.getElementById(&quot;msg_please_fill_From_Date_for_already_added_record_id&quot;).value;var msg_please_fill_To_Date_for_already_added_record=document.getElementById(&quot;msg_please_fill_To_Date_for_already_added_record_id&quot;).value;var msg_please_add_Account_Type_for_already_added_record=document.getElementById(&quot;msg_please_add_Account_Type_for_already_added_record_id&quot;).value;
var msg_enter_CIF_before_Proceeding=document.getElementById(&quot;msg_enter_CIF_before_Proceeding_id&quot;).value;var dr_suffix_key_trans=document.getElementById(&quot;dr_suffix_key_trans_id&quot;).value;var cr_suffix_key_trans=document.getElementById(&quot;cr_suffix_key_trans_id&quot;).value;var send_alert_title=document.getElementById(&quot;send_alert_title_id&quot;).value;var approve_item_key=document.getElementById(&quot;approve_item_key_id&quot;).value;var usr_inf_key=document.getElementById(&quot;usr_inf_key_id&quot;).value;var modify_button=document.getElementById(&quot;modify_button_id&quot;).value;
var open_multiple_item_confirmation=document.getElementById(&quot;open_multiple_item_confirmation_id&quot;).value;var ack_ok_warning=document.getElementById(&quot;ack_ok_warning_id&quot;).value;var trs_timeout_openitem_warning=document.getElementById(&quot;trs_timeout_openitem_warning_id&quot;).value;var void_button=document.getElementById(&quot;void_button_id&quot;).value;var scan_button=document.getElementById(&quot;scan_button_id&quot;).value;var deduct_Charges_From_Account_key=document.getElementById(&quot;deduct_Charges_From_Account_key_id&quot;).value;
var drawdown_key=document.getElementById(&quot;drawdown_key_id&quot;).value;var investment_Account_key=document.getElementById(&quot;investment_Account_key_id&quot;).value;var More_Addresses_key=document.getElementById(&quot;More_Addresses_key_id&quot;).value;var Current_Account_key=document.getElementById(&quot;Current_Account_key_id&quot;).value;var Additional_Fields_key=document.getElementById(&quot;Additional_Fields_key_id&quot;).value;var error_IRIS_Enrol_Key=document.getElementById(&quot;error_IRIS_Enrol_Key_id&quot;).value;var Choose_customer_key=document.getElementById(&quot;Choose_customer_key_id&quot;).value;
var Clear_customer_key=document.getElementById(&quot;Clear_customer_key_id&quot;).value;var imgNotAvailable=document.getElementById(&quot;imgNotAvailable_id&quot;).value;var Print_Swift_key=document.getElementById(&quot;Print_Swift_key_id&quot;).value;var Print_Swift_must_be_related_to_a_record_key=document.getElementById(&quot;Print_Swift_must_be_related_to_a_record_key_id&quot;).value;var signature_popup_title=document.getElementById(&quot;signature_popup_title_id&quot;).value;var signature_close_btn=document.getElementById(&quot;signature_close_btn_id&quot;).value;
var signature_action_title=document.getElementById(&quot;signature_action_title_id&quot;).value;var no_appr_sign_msg=document.getElementById(&quot;no_appr_sign_msg_id&quot;).value;var sign_signature_msg=document.getElementById(&quot;sign_signature_msg_id&quot;).value;var gen_ann_saved_msg=document.getElementById(&quot;gen_ann_saved_msg_id&quot;).value;var desc_mandatory_msg=document.getElementById(&quot;desc_mandatory_msg_id&quot;).value;var choose_doc_msg=document.getElementById(&quot;choose_doc_msg_id&quot;).value;var sign_copied=document.getElementById(&quot;sign_copied_id&quot;).value;
var signature_key=document.getElementById(&quot;signature_key_id&quot;).value;var jvs_list_key=document.getElementById(&quot;jvs_list_key_id&quot;).value;var Accept_key=document.getElementById(&quot;Accept_key_id&quot;).value;var Reject_Deleted_key=document.getElementById(&quot;Reject_Deleted_key_id&quot;).value;var Reject_Active_Deleted_key=document.getElementById(&quot;Reject_Active_Deleted_key_id&quot;).value;var english_key=document.getElementById(&quot;english_key_id&quot;).value;var arabic_key=document.getElementById(&quot;arabic_key_id&quot;).value;
var french_key=document.getElementById(&quot;french_key_id&quot;).value;var choose_language_key=document.getElementById(&quot;choose_language_key_id&quot;).value;var status_list_key=document.getElementById(&quot;status_list_key_id&quot;).value;var stat_of_account_key=document.getElementById(&quot;stat_of_account_key_id&quot;).value;var advice_key=document.getElementById(&quot;advice_key_id&quot;).value;var opening_acc_key=document.getElementById(&quot;opening_acc_key_id&quot;).value;var specify_report_key=document.getElementById(&quot;specify_report_key_id&quot;).value;
var whiteListed_key=document.getElementById(&quot;whiteListed_key_id&quot;).value;var company_key=document.getElementById(&quot;company_key_id&quot;).value;var branch_key=document.getElementById(&quot;branch_key_id&quot;).value;var cif_key=document.getElementById(&quot;cif_key_id&quot;).value;var acc_key=document.getElementById(&quot;acc_key_id&quot;).value;var savedMsg=document.getElementById(&quot;savedMsg_id&quot;).value;var show_picture_title=document.getElementById(&quot;show_picture_title_id&quot;).value;var No_Data_Found_key=document.getElementById(&quot;No_Data_Found_key_id&quot;).value;
var details_key=document.getElementById(&quot;details_key_id&quot;).value;var Repayment_Plan_details_not_available_key=document.getElementById(&quot;Repayment_Plan_details_not_available_key_id&quot;).value;var alerts_key=document.getElementById(&quot;alerts_key_id&quot;).value;var suspicious_list_key=document.getElementById(&quot;suspicious_list_key_id&quot;).value;var passbook_pages_exhausted_key=document.getElementById(&quot;passbook_pages_exhausted_key_id&quot;).value;var passbook_flip_pages_key=document.getElementById(&quot;passbook_flip_pages_key_id&quot;).value;
var brought_forward_amount_key=document.getElementById(&quot;brought_forward_amount_key_id&quot;).value;var Missing_File_Code_key=document.getElementById(&quot;Missing_File_Code_key_id&quot;).value;var File_Names_Do_Not_Match_key=document.getElementById(&quot;File_Names_Do_Not_Match_key_id&quot;).value;var Process_Executed_Successfully_key=document.getElementById(&quot;Process_Executed_Successfully_key_id&quot;).value;var File_Input_Parameters_Mandatory_key=document.getElementById(&quot;File_Input_Parameters_Mandatory_key_id&quot;).value;
var Input_Tag_values_Mandatory_key=document.getElementById(&quot;Input_Tag_values_Mandatory_key_id&quot;).value;var Missing_File_Location_key=document.getElementById(&quot;Missing_File_Location_key_id&quot;).value;var dyn_import_File_Loc_key=document.getElementById(&quot;Missing_File_Location_key_id&quot;).value;var fav_maint_title_key=document.getElementById(&quot;fav_maint_title_key_id&quot;).value;var select_fav_screen_key=document.getElementById(&quot;select_fav_screen_key_id&quot;).value;var selection_leaf_node_key=document.getElementById(&quot;selection_leaf_node_key_id&quot;).value;
var app_infos_title_key=document.getElementById(&quot;app_infos_title_key_id&quot;).value;var confirm_del_widget_key=document.getElementById(&quot;confirm_del_widget_key_id&quot;).value;var widget_not_avail_key=document.getElementById(&quot;widget_not_avail_key_id&quot;).value;var drag_widget_key=document.getElementById(&quot;drag_widget_key_id&quot;).value;var loading_key=document.getElementById(&quot;loading_key_id&quot;).value;var app_not_found_key=document.getElementById(&quot;app_not_found_key_id&quot;).value;var no_alert_found_key=document.getElementById(&quot;no_alert_found_key_id&quot;).value;
var resize_widget_key=document.getElementById(&quot;resize_widget_key_id&quot;).value;var custom_wkspce_key=document.getElementById(&quot;custom_wkspce_key_id&quot;).value;var add_custom_widget_key=document.getElementById(&quot;add_custom_widget_key_id&quot;).value;var trans_key=document.getElementById(&quot;trans_key_id&quot;).value;var branch_is_now_opened_key=document.getElementById(&quot;branch_is_now_opened_key_id&quot;).value;var branch_is_now_closed_key=document.getElementById(&quot;branch_is_now_closed_key&quot;).value;var no_widgets_key=document.getElementById(&quot;no_widgets_key_id&quot;).value;
var change_pass=document.getElementById(&quot;change_pass_id&quot;).value;var description_label_key=document.getElementById(&quot;description_label_key_id&quot;).value;var select_screen_help_key=document.getElementById(&quot;select_screen_help_key_id&quot;).value;var record_lvl_att_key=document.getElementById(&quot;record_lvl_att_key_id&quot;).value;var record_lvl_att_wlog_key=document.getElementById(&quot;record_lvl_att_wlog_key_id&quot;).value;var entity_lvl_att_key=document.getElementById(&quot;entity_lvl_att_key_id&quot;).value;
var entity_lvl_att_wlog_key=document.getElementById(&quot;entity_lvl_att_wlog_key_id&quot;).value;var record_Logs_Key=document.getElementById(&quot;Record_Logs_Key_id&quot;).value;var for_key=document.getElementById(&quot;for_key_id&quot;).value;var toolTipTrans_key=document.getElementById(&quot;toolTipTrans_key_id&quot;).value;var grid_user_pref_key=document.getElementById(&quot;grid_user_pref_key_id&quot;).value;var tentative_Schedule_Created_msg_key=document.getElementById(&quot;tentative_Schedule_Created_msg_key_id&quot;).value;
var finalSignOffChecking_key=document.getElementById(&quot;finalSignOffChecking_key_id&quot;).value;var report_Return_Data_key=document.getElementById(&quot;report_Return_Data_key_id&quot;).value;var server_request_completes_key=document.getElementById(&quot;server_request_completes_key_id&quot;).value;var download_key=document.getElementById(&quot;download_key_id&quot;).value;var elemIdAndNameChecking_key=document.getElementById(&quot;elemIdAndNameChecking_key_id&quot;).value;var Selected_key=document.getElementById(&quot;Selected_key_id&quot;).value;
var oldValKey=document.getElementById(&quot;oldValKey_id&quot;).value;var grid_user_def_key=document.getElementById(&quot;grid_user_def_key_id&quot;).value;var status_cust_key=document.getElementById(&quot;status_cust_key_id&quot;).value;var token_key=document.getElementById(&quot;token_key_id&quot;).value;var closeDynScreen_key=document.getElementById(&quot;closeDynScreen_key_id&quot;).value;var smartLabel_title_key=document.getElementById(&quot;smartLabel_title_key_id&quot;).value;var January_key=document.getElementById(&quot;January_key_id&quot;).value;
var February_key=document.getElementById(&quot;February_key_id&quot;).value;var March_key=document.getElementById(&quot;March_key_id&quot;).value;var April_key=document.getElementById(&quot;April_key_id&quot;).value;var May_key=document.getElementById(&quot;May_key_id&quot;).value;var June_key=document.getElementById(&quot;June_key_id&quot;).value;var July_key=document.getElementById(&quot;July_key_id&quot;).value;var August_key=document.getElementById(&quot;August_key_id&quot;).value;var September_key=document.getElementById(&quot;September_key_id&quot;).value;
var October_key=document.getElementById(&quot;October_key_id&quot;).value;var November_key=document.getElementById(&quot;November_key_id&quot;).value;var December_key=document.getElementById(&quot;December_key_id&quot;).value;var invalid_widget_url=document.getElementById(&quot;invalid_widget_url_key&quot;).value;var server_conn_drop_key=document.getElementById(&quot;server_conn_drop_key_id&quot;).value;var yes_confirm=document.getElementById(&quot;yes_key_id&quot;).value;var no_confirm=document.getElementById(&quot;no_key_id&quot;).value;
var defEmptyRepConfirm_key=document.getElementById(&quot;defemptyrepconfirm_key&quot;).value;var defEmptyRep_key=document.getElementById(&quot;defemptyrep_key&quot;).value;var activex_disabled_key=document.getElementById(&quot;activex_disabled_key_id&quot;).value;var cannotProceed_key=document.getElementById(&quot;cannot_proceed_key&quot;).value;var hijriDate_cust_key=document.getElementById(&quot;hijriDate_cust_key&quot;).value;var view_grid_key=document.getElementById(&quot;view_grid_key&quot;).value;var live_search_key=document.getElementById(&quot;live_search_key&quot;).value;
var Confirm_Reverse_Process_key=document.getElementById(&quot;Confirm_Reverse_Process_key_id&quot;).value;var notifications_key=document.getElementById(&quot;notifications_key&quot;).value;var function_not_defined_key=document.getElementById(&quot;function_not_defined_key_id&quot;).value;var your_selection_will_be_lost_key=document.getElementById(&quot;your_selection_will_be_lost_key_id&quot;).value;var cannot_open_dyn_screen_key=document.getElementById(&quot;cannot_open_dyn_screen_key&quot;).value;var contact_screen_admin_key=document.getElementById(&quot;contact_screen_admin_key_id&quot;).value;
var component_not_defined_key=document.getElementById(&quot;component_not_defined_key_id&quot;).value;var link_dyn_scr_key=document.getElementById(&quot;link_dyn_scr_key_id&quot;).value;var file_key=document.getElementById(&quot;file_key&quot;).value;var line_key=document.getElementById(&quot;line_key&quot;).value;var column_key=document.getElementById(&quot;column_key&quot;).value;var scrn_doesnt_contain_components_key=document.getElementById(&quot;scrn_doesnt_contain_components_key&quot;).value;var digital_chnl_trans_key=document.getElementById(&quot;digital_chnl_trans_key_id&quot;).value;
var report_key=document.getElementById(&quot;report_key_trans_id&quot;).value;var report_error_key=document.getElementById(&quot;report_error_key_trans_id&quot;).value;var duplicate_elem_id_key=document.getElementById(&quot;duplicate_elem_id&quot;).value;var old_value_key=document.getElementById(&quot;old_value_key_id&quot;).value;  Menu Options/html[1]/body[@class=&quot;ui-layout-container&quot;]/div[@class=&quot;displayProgBar&quot;]/div[@class=&quot;ui-widget-shadow progBarBackGound&quot;]WarningPlease, select rowPrevNextMay 2022SuMoTuWeThFrSa12345678910111213141516171819202122232425262728293031    Reset User PreferencesAdd New TabWarningPlease, select rowLoading... Code Brief Description Long Description الوصف المختصر الوصف الكامل Additional Reference1ValueValueValueValueR12Value JointValue JointValue JointValue JointR23MinorMinorMinorMinorR34SpecialSpecialSpecialSpecialR45Special JointSpecial JointSpecial JointSpecial JointR5 Page  of 65101520View1-5of28WarningPlease, select rowLoading... Code Brief Description الوصف المختصر Code1National IDالبطاقة الشخصية 3Passportجواز سفر 5Resident IDبطاقه المقيم 7Com. Reg.تسجيل الشركة 8Other IDبطاقة أخرى  Page  of 35101520View1-5of12&quot;) or . = concat(&quot;           var globalThemeName=&quot; , &quot;'&quot; , &quot;cupertino&quot; , &quot;'&quot; , &quot;;$(document).ready(function(){$(&quot;#ui_bar_div&quot;).css(&quot;height&quot;,&quot;100%&quot;);$(&quot;#ui_bar_div&quot;).niceScroll()});                                    User    : 1477          06 March 2022       Company    : Bank Nizwa            Branch   : Head office             
.ui-autocomplete {
	max-height: 180px;
	overflow-y: auto;
	overflow-x: hidden;
}
                           var session_is_now_opened_key=document.getElementById(&quot;session_is_now_opened_key_id&quot;).value;var switch_company_key=document.getElementById(&quot;switch_company_key_id&quot;).value;var switch_branch_key=document.getElementById(&quot;switch_branch_key_id&quot;).value;var closing_session_key=document.getElementById(&quot;closing_session_key_id&quot;).value;var session_is_now_closed_key=document.getElementById(&quot;session_is_now_closed_key_id&quot;).value;var show_btn_key=document.getElementById(&quot;show_btn_key_id&quot;).value;
var assign_widget_key=document.getElementById(&quot;assign_widget_key_id&quot;).value;var usr_allowed_portlets_key=document.getElementById(&quot;usr_allowed_portlets_key_id&quot;).value;var dynclientparams_key=document.getElementById(&quot;dynclientparams_key_id&quot;).value;var dynclientparams_approve_key=document.getElementById(&quot;dynclientparams_approve_key_id&quot;).value;                      Full Screen                                  Options                                     Home                                Log Out                               Help                      Language          $.struts2_jquery.require(&quot;AppMainHeader.js&quot;,null,&quot;/imal_core_portal/common/js/customization/&quot;);   _menLayOut({currRTL:&quot;ltr&quot;});               var tech_det_log_level_key=document.getElementById(&quot;log_level_key_id&quot;).value;var saveLevelLabel=document.getElementById(&quot;saveLabel_id&quot;).value;var record_was_Updated_Successfully_tech_key=document.getElementById(&quot;record_was_Updated_Successfully_key_id&quot;).value;var info_msg_tech_title_key=document.getElementById(&quot;info_msg_title_id&quot;).value;var tech_det_system_properties_key=document.getElementById(&quot;system_properties_key&quot;).value;                          Change Password                     Switch Company                     Switch Branch                        Final Sign Off         $.struts2_jquery.require(&quot;PortalHeaderOptions.js&quot;,null,&quot;/imal_core_portal/common/js/portaloptions/&quot;);                  Menu Options      

		
	var actionName=&quot;generateMenuOnRequest&quot;;
	var targetName=&quot;content-container&quot;;
	var appName=&quot;RET&quot;;
	var id = &quot;appMenu&quot;;
	var params = &quot;actionName=&quot;+actionName+&quot;&amp;targetName=&quot;+targetName+&quot;&amp;appName=&quot;+appName+&quot;&amp;id=&quot;+id;
	params = returnEncryptedData(params);
	var url=actionName + &quot;?&quot; + params ;
	$(document).ready(function(){
	$.struts2_jquery.requireCss(&quot;/common/style/menu/AccordionMenu.css&quot;,&quot; , &quot;'&quot; , &quot;/imal_core_portal&quot; , &quot;'&quot; , &quot;);
	if(document.dir == &quot;rtl&quot;)
	{
		$.struts2_jquery.requireCss(&quot;/common/style/menu/AccordionMenu-rtl.css&quot;,&quot; , &quot;'&quot; , &quot;/imal_core_portal&quot; , &quot;'&quot; , &quot;);
	}
	ddaccordion.initRoot(id,url,false);
	
	$.subscribe(&quot; , &quot;'&quot; , &quot;setTitle&quot; , &quot;'&quot; , &quot;, function(event, data) 
		{
			var titleVal = event.originalEvent.target.text;
			$(&quot; , &quot;'&quot; , &quot;title&quot; , &quot;'&quot; , &quot;).remove();
			$(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;title>&quot; , &quot;'&quot; , &quot;+titleVal+&quot; , &quot;'&quot; , &quot;&lt;/title>&quot; , &quot;'&quot; , &quot;)
		}
	);
	function setHeaderTitle(ele,event)
	{	
		var hrefTag=$(ele).find(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;).get(0)
		var title = hrefTag.id
		$(&quot; , &quot;'&quot; , &quot;title&quot; , &quot;'&quot; , &quot;).remove();
		$(&quot; , &quot;'&quot; , &quot;head&quot; , &quot;'&quot; , &quot;).append(&quot; , &quot;'&quot; , &quot;&lt;title>&quot; , &quot;'&quot; , &quot;+title+&quot; , &quot;'&quot; , &quot;&lt;/title>&quot; , &quot;'&quot; , &quot;)
	}
	});
	
	function accordionReloadMenuItem(menuVar)
	{
		var theMenuVar = menuVar.replace(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;); 
		theMenuVar = menuVar.replace(&quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot;);
		var $menuElt;
		if(menuVar == &quot;ROOT&quot;)
		{
			ddaccordion.initRoot(id ,url,false);
		}
		else
		{
			if( $(&quot;.h3_&quot;+theMenuVar).html() != null) //means header level 
			{
				$menuElt = $(&quot;.h3_&quot;+theMenuVar)
			}
			else if($(&quot;.a_&quot;+theMenuVar).html() != null) //sublevel
			{
			 	$menuElt = $(&quot;.a_&quot;+theMenuVar);
			}
			if($menuElt != null &amp;&amp; $menuElt.html() != null)
			{
				$menuElt.data(&quot; , &quot;'&quot; , &quot;ajaxinfo&quot; , &quot;'&quot; , &quot;).status = &quot;none&quot;
				$menuElt.trigger(&quot;evt_accordion&quot;,[false,true]);
			}
		} 
	}
	

360 Degrees view		AccountsFixed Maturity AccountsFixed Maturity AccountsGeneral AccountsOpening	Maintenance	Close	Close With Transfer	Reject	Approve Reject	Suspend	Reactivate	Accounts ListAccounts ListPrint	Account Query	Front Office ManagementMaintenance	Suspend	Query	Print CIF Confirmation	Print Account Confirmation	Chequebook RequestChequebook RequestMemo DetailMemo DetailLost &amp; Found ManagementLost &amp; Found ManagementCards ManagementCards ManagementTransactionsTransactionsReportsReportsFile ManagementFile ManagementQueriesQueriesCertificatesCertificates
  


var app_main_title = &quot;CSM CUSTOMER&quot;;


var currency_denomination_key=&quot;Currency Denomination&quot;;


var View_Chart_key=&quot;View Chart&quot;;
var Safebox_key=&quot;Safebox&quot;;
var Cannot_be_less_than_zero_key=&quot;Cannot be less than zero&quot;;
var Should_be_between_0_and_100_key=&quot;Should be between 0.0  and 100&quot;;
var Percentage_between_0_and_100_key=&quot;Percentage Should be between 0  and 100&quot;;
var ToDate_Cannot_be_less_then_FromDate_key=&quot;To Date Cannot be Less Than From Date&quot;;
var Retirement_Age_less_zero_key=&quot;Retirement Age Can Not be Less than 0&quot;;
var Other_Income_less_zero_key=&quot;Other Income Can Not be Less than 0&quot;;
var Annual_Salary_less_zero_key=&quot;Annual Salary Can Not be Less than 0&quot;;
var Monthly_Salary_less_zero_key=&quot;Monthly Basic Salary Can Not be Less than 0&quot;;
var Transportation_Fees_less_zero_key=&quot;Transportation Fees Can Not be Less than 0&quot;;
var Location_Fees_less_zero_key=&quot;Location Fees Can Not be Less than 0&quot;;
var Other_Fees_less_zero_key=&quot;Other Fees Can Not be Less than 0&quot;;

var cannot_be_less_than_or_equal_zero_key = &quot;Value cannot be less than or equal to zero&quot;;
var  keyNum_not_equal_oldValue  = &quot;Key number cannot be equal to the old value&quot;;


var CIF_Products_key = &quot;CIF Products&quot;;



var account_No = &quot;Account No&quot;;



var Please_Select_One_Record_key =&quot;Please Select One and Only One Record&quot;;
var file_has_been_automatically_saved_key = &quot;file has been automatically saved&quot;;
var Please_Select_Records_key =&quot;Please Select Records to Delete&quot;;


var The_CIF_is_blacklisted_key = &quot;The CIF is blacklisted&quot;;
var tax_regulation_status_key = &quot;Tax Regulation Status&quot;;
var inserted_data_will_be_cleared_do_you_wish_to_proceed_key = &quot;Inserted data will be cleared, do you wish to proceed&quot;;
var Passport_Info_key = &quot;Passport Info&quot;;
var residency_information_key = &quot;Residency Information&quot;;
var tax_payer_information_key = &quot;Tax Payer Information&quot;;
var telephone_info_key 		  = &quot;Telephone Information&quot;;
var compfinancialfigures_key = &quot;Company Financial Figures&quot;;

var Links_Management_key= &quot;Links Management&quot;;

var Account_Details_key=&quot;Account Details&quot;;
var chqbrkey=&quot;Chq Br&quot;;
var chqbknumkey = &quot;Chqbk Num&quot;;
var fillfromtochqnumfilterkey = &quot;Please fill From Cheque Number and To Cheque Number to proceed with filtering &quot;;
var invalidmissingchqnumkey = &quot;Invalid/Missing cheque number&quot;;
var tochqnumgreaterfromchqnumkey = &quot;To Chq Num should be greater than From Chq Num&quot;;
var pleaseselcheqtocancel = &quot;Please select the chequebook(s) to cancel&quot;;
var pleasefillcancelreasonallrows = &quot;Please fill cancel reason for all rows&quot;;

     var RTL_DIRECTION=&quot;ltr&quot;;$(document).ready(function(){intializeMainTabs(&quot;mainTabs&quot;,{url:jQuery.contextPath+&quot;/path/loadScreen?&quot;,reloadAlert:&quot;Do you need to reload the tab contents of&quot;,closeAlert:&quot;Close&quot;})});     Front Office Management / MaintenanceRemove Tab              
      Create Customer - Individual           F00I1MT       Smart           Audit                        Record               Record Log                 Entity               Entity log           Record        Search       New                 Loading...CIF   Branch Code CIF No Status Short Name Long Name Full Name Additional Reference Tel Eco Sector Relation Code Country Id No CIF Type One Obligor Email Id Gender Card Name Nick Name الإسم المختصر بالعربية الإسم الكامل بالعربية الاسم المفصّل ToggleClearPage  of 05101520No records to view


        
.hyperlink {font-style: italic !important; font-weight:bold !important; text-decoration:underline !important; cursor: pointer !important;}
                                                                                  
                                     CIF No    
   Branch     

   
   Additional Reference    
   Status    
    Status                 Reason     

   
   Score Value    
   
  Score       More Scores  



  FICA List   

           Reserved CIF  



    Segment Code     

   
    cif_segment_details_key  



        Reason    
                
Name &amp; AddressAdditional DetailsReferencesMarket DetailsAdditional FieldsCIF BIC CodeFund Transfer
 
 
 
 
 
 
 
        

   CIF Details 
    CIF Type *   

   
   The CIF is    CorporateIndividual

    Subscription To  



                             ID Type*   

     
   
 Id No     ANDORAND/OR

 join_type_key         Joint ID    

   
   
    IDs  



    Joint IDs  



     ID Delivery Date    
   Place Of Issue     

   
     ID Type 2    

   
   
    Reset  



     ID Delivered By       			             Date of Birth*
   			             ID Expiry Date*
   

 

 

   Biometric Approve       Country Of Issuance    

   
       Country Of Issuance *   

   
       Marital Status*   SingleDivorcedWidowedMarriedUnknown

     Gender*   FemaleMaleN/A

      Language   ArabicBothEnglish

             Intraday-Excess Limit    
                 

 

 

 Watch List          

 

 

 cif_minor_key      cif_minor_type_key    

   
       Number of Partners    
    
   CIF Brief and Long Name 
    Salutation    

   
        Nationality   

   
                  Short Name    
    Country      

   
     Long Name   
    Place of Birth     

   
      
   الإسم المختصر بالعربية   Race    

   
     Full Name    
    Region    

   
     Religion    

   
      
   الإسم الكامل بالعربية        
   الاسم المفصّل       Nick Name    
      Card Name   
    Joint    
    
   Name Details 
  
   Passport Info 
  
   CIF Address 
                                   

 

 

   Print Statement                 Address Description    
     Contact Name    
     Salutation    
     House No / Flat No   
     Building Name / Number   
     Way No   
     Area / Willayat   
     Street Details    
     City   
     Country    
                 إسم الإتصلات    
     الحالة    
        
         
        
        
     تفاصيل عن الشارع    
        
     البلد    
                   Phone No Res.     
     Office Phone No   
     Mobile No (1)       
     Mobile No (2)   
        
     Email           verify_email_key  



   

 

 

 email_verified_key      Telex                       

   
     Country   

   
        

   
     Sector   

   
     PO BOX Area    

   
      
      Postal Code     

    
     Address Description   

   
     Proof of Address    

   
                       Expiry Date   
   From Date    
   To Date    
   
	  
	  

       
 Permanent Address 
   

 

   
	  
	  

       
 Default Address 
   

 

   
	  
	  

       
 Wallet 
   

 

              More Addresses 



      
   Statement 
    Category of Statement Delivery    PrintedElectronic

    Multi Statements  



     Mode of Stmt Delivery   Collect at BranchDirect MailEmailFaxNot RequiredP.O. BoxSMSTelexStatement for StaffSwift

   Statement   HoldYesNo

     
	  
	  

       
 Consolidate Same Reference 
   

 

     Description      Periodicity    QuarterlySemi-AnnuallyDailyFortnightlyWeeklyYearlyMonthly

     Status Reason            
  Tax Exemption 
   

 

 

 Exempted from Filer  Exempted from Filer Reason   

 
  Exemption Expiry Date  
    
      Status Reason          
          

 Additional Details 
    CIF Category    PotentialClientVIPNeither

    Bank Ref    
   

 

 

   Send SMS on Account Opening    

 

 

   Chat Bank    

 

 

   Exempt Full Charges Waiver      Vat Zone    

   
   

 

 

   Exempted From VAT         

 

 

   exmpted_frm_pyments_pntly_yn                   
	  
	  

       
 Exempted From zakat 
   

 

 Reason    

 
                        Country of Birth   

   
   Register No    
       Economic_Agent_key    

   
   Legal Status*   

   
     Economic Sector*    

   
   GIIN    
    Req Doc  



    Education  



     Sub Eco Sector    

   
   profession_category_key    

   
     profession_segment_key    

   
   Profession    

   
     Ranking *    

   
      Educational Level    

   
   Profession     EmployedSelf-EmployedUnemployed

      No of Dependents    
       One Obligor    

   
    Additional One-Obligors  



      CIF Profile    

   
   

 

 

 Allow to Exceed One Obligor/CIF Capital Limit     CIF Branches  



       Exclude TOB Charges    YESNO

     Filer/ Non-Filer:    FilerN/ANon Filer

     CIF Limit Cap    
   
     Web Site    
   Proof of Occupation    

   
       Source of Funds    

   
   Source of Funds Description  
      Occupation    

   
   Position    

   
    Occupation  



     Employer CIF/Name     

   
   Monthly Income    
           Employment Status    

   
            Division *   

   
   Resident       ResidentNon Resident

      Department   

   
   Residence Type    

   
      Legal Action    
   Populate CIF    NoYes

   KYC     Not RequiredRecalcitrantCompleted By CallCompletedOutstanding

       KYC Details hidden for customization  



      BVN    
       CIF is Complete    YesNo

      KYC Expiry Date    
   kycreviewdate_key    
        agent    

   
                   
   Authorized Signatories/Power Of Attorney 
    Name    
      Ownership Details  



    More Signatory  



     ID Type    

   
    Id No    
     Country Of Issuance *   

   
        
 Introducer Details 
  
     CIF Rating    

   
    ... 



   Relation    

   
     Forbid Transaction    NoneCreditDebitBoth

    Forbid Reason         tax_regime_key    

   
        Swift Compliant Address    
    Capital Amount   
     
   BKE for BICs    NoYes

     
   Central Bank Secret No.    
     
   License Expiry Date    
        Declared Income    
         Nationality Risk    YESNO

   Politically Exposed    YESNO

     Business Risk    YESNO

   Group ID    
    

  Mobile wallet 
   
	  
	  

       
 Register 
   

 

 registration_number_key  
 Nickname  
    wallet Account  
 Mobile No.  
   
 
     
        

   References Details 
  
   Relationship Officer Details 
     Officer ID    

   Officer Name    
    More Officers  



       Role Code    

   
        Division Code    

   
        Dept Code    

   
         Account Manager    

   Manager Name    
        Group    

   
        Team    

   
        
   
              
              Additional Fields  



    

   Save  

             
      


  

                     @-webkit-keyframes marqueeAnimation-199387  { 100%  {margin-left:-0px}} $.struts2_jquery.require(&quot;jquery.marquee.js&quot;,null,jQuery.contextPath+&quot;/common/jquery/js/marquee/&quot;);$.struts2_jquery.require(&quot;jquery.pause.js&quot;,null,jQuery.contextPath+&quot;/common/jquery/js/marquee/&quot;);if($(&quot;#scrollingMarquee&quot;))$(function(){$(&quot;#scrollingMarquee&quot;).marquee({duration:5E3})});      Copyright ©2022 Azentio ®All Rights Reserved    iMAL*CSM 14.6.0.0 Build 418   Powered By                                                                                                                                                                                                                                                                                                                 var view_Memo_key=document.getElementById(&quot;View_Memo_key_id&quot;).value;var Break_of_Deposit_key=document.getElementById(&quot;Break_of_Deposit_key_id&quot;).value;var cannot_be_less_than_or_equal_zero_key=document.getElementById(&quot;cannot_be_less_than_or_equal_zero_key_id&quot;).value;var teg_is_greater_than_tie_key=document.getElementById(&quot;teg_is_greater_than_tie_key_id&quot;).value;var tie_rate_exeeded_key=document.getElementById(&quot;tie_rate_exeeded_key_id&quot;).value;var current_yield_is_key=document.getElementById(&quot;current_yield_is_key_id&quot;).value;
var and_related_teg_is_key=document.getElementById(&quot;and_related_teg_is_key_id&quot;).value;var the_optimum_teg_rate_should_be_key=document.getElementById(&quot;the_optimum_teg_rate_should_be_key_id&quot;).value;var and_the_max_yield_should_be_key=document.getElementById(&quot;and_the_max_yield_should_be_key_id&quot;).value;var teg_is_greater_than_the_tie_after_adjusting_yield_charges=document.getElementById(&quot;teg_is_greater_than_the_tie_after_adjusting_yield_charges_id&quot;).value;var the_optimum_teg_rate_changed_to_key=document.getElementById(&quot;the_optimum_teg_rate_changed_to_key_id&quot;).value;
var and_the_max_yield_is_key=document.getElementById(&quot;and_the_max_yield_is_key_id&quot;).value;var Market_Value_key=document.getElementById(&quot;Market_Value_key_id&quot;).value;var Book_Value_key=document.getElementById(&quot;Book_Value_key_id&quot;).value;var app_title_not_defined=document.getElementById(&quot;app_title_not_defined_id&quot;).value;var grid_addLabel=document.getElementById(&quot;grid_addLabel_id&quot;).value;var grid_editLabel=document.getElementById(&quot;grid_editLabel_id&quot;).value;var grid_delLabel=document.getElementById(&quot;grid_delLabel_id&quot;).value;
var grid_approveLabel=document.getElementById(&quot;grid_approveLabel_id&quot;).value;var grid_rejectLabel=document.getElementById(&quot;grid_rejectLabel_id&quot;).value;var grid_createFromLabel=document.getElementById(&quot;grid_createFromLabel_id&quot;).value;var saveLabel=document.getElementById(&quot;saveLabel_id&quot;).value;var msg_progDefaultLabel=document.getElementById(&quot;msg_progDefaultLabel_id&quot;).value;var Change_running_date_key=document.getElementById(&quot;Change_running_date_key_id&quot;).value;
var SMART_System_Maintenance_Additional_Reference_Table_key=document.getElementById(&quot;SMART_System_Maintenance_Additional_Reference_Table_key_id&quot;).value;var Smart_Details_not_defined_key=document.getElementById(&quot;Smart_Details_not_defined_key_id&quot;).value;var SMART_must_be_related_to_a_record_key=document.getElementById(&quot;SMART_must_be_related_to_a_record_key_id&quot;).value;var Error_retrieving_file_from_database_key=document.getElementById(&quot;Error_retrieving_file_from_database_key_id&quot;).value;
var Preview_image_key=document.getElementById(&quot;Preview_image_key_id&quot;).value;var Mandatory_Field_key=document.getElementById(&quot;Mandatory_Field_key_id&quot;).value;var msg_noRecordSelectedLabel=document.getElementById(&quot;msg_noRecordSelectedLabel_id&quot;).value;var Total_Amount_key=document.getElementById(&quot;Total_Amount_key_id&quot;).value;var Warning_key=document.getElementById(&quot;Warning_key_id&quot;).value;var total_All=document.getElementById(&quot;total_All_id&quot;).value;var Confirm_Approve_Process_key=document.getElementById(&quot;Confirm_Approve_Process_key_id&quot;).value;
var Confirm_Reject_Process_key=document.getElementById(&quot;Confirm_Reject_Process_key_id&quot;).value;var Confirm_Reinstate_Process_key=document.getElementById(&quot;Confirm_Reinstate_Process_key_id&quot;).value;var Confirm_Reactivate_Process_key=document.getElementById(&quot;Confirm_Reactivate_Process_key_id&quot;).value;var Confirm_Suspend_Process_key=document.getElementById(&quot;Confirm_Suspend_Process_key_id&quot;).value;var Confirm_Proceed_key=document.getElementById(&quot;Confirm_Proceed_key_id&quot;).value;
var Confirm_Delete_Process_key=document.getElementById(&quot;Confirm_Delete_Process_key_id&quot;).value;var confirm_Export_Process_key=document.getElementById(&quot;Confirm_Export_Process_key_id&quot;).value;var Confirm_Save_Process_key=document.getElementById(&quot;Confirm_Save_Process_key&quot;).value;var date_of_birth_label=document.getElementById(&quot;date_of_birth_label_id&quot;).value;var birthDate_label=document.getElementById(&quot;birthDate_label_id&quot;).value;var estbDate_label=document.getElementById(&quot;estbDate_label_id&quot;).value;
var error_msg_title=document.getElementById(&quot;error_msg_title_id&quot;).value;var success_msg_title=document.getElementById(&quot;success_msg_title_id&quot;).value;var confirm_msg_title=document.getElementById(&quot;confirm_msg_title_id&quot;).value;var warning_msg_title=document.getElementById(&quot;warning_msg_title_id&quot;).value;var info_msg_title=document.getElementById(&quot;info_msg_title_id&quot;).value;var proceed_msg_title=document.getElementById(&quot;proceed_msg_title_id&quot;).value;var changes_made_confirm_msg=document.getElementById(&quot;changes_made_confirm_msg_id&quot;).value;
var changes_not_available_key=document.getElementById(&quot;changes_not_available_key_id&quot;).value;var missing_selection_type_key=document.getElementById(&quot;missing_selection_type_key_id&quot;).value;var cache_cleared_success_key=document.getElementById(&quot;cache_cleared_success_key_id&quot;).value;var close_all_opened_screen_key=document.getElementById(&quot;close_all_opened_screen_key_id&quot;).value;var period_label=document.getElementById(&quot;period_label_id&quot;).value;var creditRating_label=document.getElementById(&quot;creditRating_label_id&quot;).value;
var trx_type_label=document.getElementById(&quot;trx_type_label_id&quot;).value;var reason_code_label=document.getElementById(&quot;reason_code_label_id&quot;).value;var charges_schema_details_label=document.getElementById(&quot;charges_schema_details_label_id&quot;).value;var cash_account_label=document.getElementById(&quot;cash_account_label_id&quot;).value;var missing_elt_msg_key=document.getElementById(&quot;missing_elt_msg_key_id&quot;).value;var account_Position_key=document.getElementById(&quot;account_Position_key_id&quot;).value;
var relogin_btn_key=document.getElementById(&quot;relogin_btn_key_id&quot;).value;var fill_old_pwd_msg_key=document.getElementById(&quot;fill_old_pwd_msg_key_id&quot;).value;var fill_both_msg_key=document.getElementById(&quot;fill_both_msg_key_id&quot;).value;var charges_Details_key=document.getElementById(&quot;charges_Details_key_id&quot;).value;var choose_a_schema_first_msg=document.getElementById(&quot;choose_a_schema_first_msg_id&quot;).value;var calculation_done_msg=document.getElementById(&quot;calculation_done_msg_id&quot;).value;
var charges_Schema_key=document.getElementById(&quot;charges_Schema_key_id&quot;).value;var account_Type_key=document.getElementById(&quot;account_Type_key_id&quot;).value;var acc_nb_key=document.getElementById(&quot;acc_nb_key_id&quot;).value;var record_was_Approved_Successfully_key=document.getElementById(&quot;record_was_Approved_Successfully_key_id&quot;).value;var record_was_Suspended_Successfully_key=document.getElementById(&quot;record_was_Suspended_Successfully_key_id&quot;).value;var record_was_Deleted_Successfully_key=document.getElementById(&quot;record_was_Deleted_Successfully_key_id&quot;).value;
var record_was_Updated_Successfully_key=document.getElementById(&quot;record_was_Updated_Successfully_key_id&quot;).value;var record_was_Reinstated_Successfully_key=document.getElementById(&quot;record_was_Reinstated_Successfully_key_id&quot;).value;var record_was_Validated_Successfully_key=document.getElementById(&quot;record_was_Validated_Successfully_key_id&quot;).value;var record_saved_Successfully_key=document.getElementById(&quot;record_saved_Successfully_key_id&quot;).value;var Authorized_Suspended_Successfully_key=document.getElementById(&quot;Authorized_Suspended_Successfully_key_id&quot;).value;
var record_reactivated_successfully_key=document.getElementById(&quot;record_reactivated_successfully_key_id&quot;).value;var authorized_reactivated_successfully_key=document.getElementById(&quot;authorized_reactivated_successfully_key_id&quot;).value;var record_rejected_Successfully_key=document.getElementById(&quot;record_rejected_Successfully_key_id&quot;).value;var authorized_rejected_successfully_key=document.getElementById(&quot;authorized_rejected_successfully_key_id&quot;).value;var record_closed_successfully_key=document.getElementById(&quot;record_closed_successfully_key_id&quot;).value;
var authorized_closed_successfully_key=document.getElementById(&quot;authorized_closed_successfully_key_id&quot;).value;var record_was_Authorized_Successfully_key=document.getElementById(&quot;record_was_Authorized_Successfully_key_id&quot;).value;var themes_transl=document.getElementById(&quot;themes_transl_id&quot;).value;var ok_label_trans=document.getElementById(&quot;ok_label_trans_id&quot;).value;var reset_label_trans=document.getElementById(&quot;reset_label_trans_id&quot;).value;var forward_label_trans=document.getElementById(&quot;forward_label_trans_id&quot;).value;
var print_label_trans=document.getElementById(&quot;print_label_trans_id&quot;).value;var dismiss_label_trans=document.getElementById(&quot;dismiss_label_trans_id&quot;).value;var openItem_label_trans=document.getElementById(&quot;openItem_label_trans_id&quot;).value;var cancel_label_trans=document.getElementById(&quot;cancel_label_trans_id&quot;).value;var max_val_trans=document.getElementById(&quot;max_val_trans_id&quot;).value;var min_val_trans=document.getElementById(&quot;min_val_trans_id&quot;).value;var max_length_trans=document.getElementById(&quot;max_length_trans_id&quot;).value;
var min_length_trans=document.getElementById(&quot;min_length_trans_id&quot;).value;var limit_key=document.getElementById(&quot;limit_key_id&quot;).value;var transaction_key=document.getElementById(&quot;transaction_key_id&quot;).value;var trx_Type_Charges_trans=document.getElementById(&quot;trx_Type_Charges_trans_id&quot;).value;var Trx_No_key=document.getElementById(&quot;Trx_No_key_id&quot;).value;var void_Reason_Charges_trans=document.getElementById(&quot;void_Reason_Charges_trans_id&quot;).value;var msg_duplicate_entry_Of_record_key=document.getElementById(&quot;msg_duplicate_entry_Of_record_key_id&quot;).value;
var msg_please_fill_Trx_Type_Code_for_already_added_record=document.getElementById(&quot;msg_please_fill_Trx_Type_Code_for_already_added_record_id&quot;).value;var msg_please_fill_Charge_Code_for_already_added_record=document.getElementById(&quot;msg_please_fill_Charge_Code_for_already_added_record_id&quot;).value;var msg_please_fill_Reason_Code_for_already_added_record=document.getElementById(&quot;msg_please_fill_Reason_Code_for_already_added_record_id&quot;).value;var msg_please_fill_Currency_for_already_added_record=document.getElementById(&quot;msg_please_fill_Currency_for_already_added_record_id&quot;).value;
var msg_please_fill_Limit_for_already_added_record=document.getElementById(&quot;msg_please_fill_Limit_for_already_added_record_id&quot;).value;var msg_please_fill_Account_Type_for_already_added_record=document.getElementById(&quot;msg_please_fill_Account_Type_for_already_added_record_id&quot;).value;var msg_please_fill_Code_for_added_record=document.getElementById(&quot;msg_please_fill_Code_for_added_record_id&quot;).value;var msg_please_fill_Line_No_for_already_added_record=document.getElementById(&quot;msg_please_fill_Line_No_for_already_added_record_id&quot;).value;
var msg_please_fill_Credit_Rating_for_already_added_record=document.getElementById(&quot;msg_please_fill_Credit_Rating_for_already_added_record_id&quot;).value;var msg_please_fill_From_Date_for_already_added_record=document.getElementById(&quot;msg_please_fill_From_Date_for_already_added_record_id&quot;).value;var msg_please_fill_To_Date_for_already_added_record=document.getElementById(&quot;msg_please_fill_To_Date_for_already_added_record_id&quot;).value;var msg_please_add_Account_Type_for_already_added_record=document.getElementById(&quot;msg_please_add_Account_Type_for_already_added_record_id&quot;).value;
var msg_enter_CIF_before_Proceeding=document.getElementById(&quot;msg_enter_CIF_before_Proceeding_id&quot;).value;var dr_suffix_key_trans=document.getElementById(&quot;dr_suffix_key_trans_id&quot;).value;var cr_suffix_key_trans=document.getElementById(&quot;cr_suffix_key_trans_id&quot;).value;var send_alert_title=document.getElementById(&quot;send_alert_title_id&quot;).value;var approve_item_key=document.getElementById(&quot;approve_item_key_id&quot;).value;var usr_inf_key=document.getElementById(&quot;usr_inf_key_id&quot;).value;var modify_button=document.getElementById(&quot;modify_button_id&quot;).value;
var open_multiple_item_confirmation=document.getElementById(&quot;open_multiple_item_confirmation_id&quot;).value;var ack_ok_warning=document.getElementById(&quot;ack_ok_warning_id&quot;).value;var trs_timeout_openitem_warning=document.getElementById(&quot;trs_timeout_openitem_warning_id&quot;).value;var void_button=document.getElementById(&quot;void_button_id&quot;).value;var scan_button=document.getElementById(&quot;scan_button_id&quot;).value;var deduct_Charges_From_Account_key=document.getElementById(&quot;deduct_Charges_From_Account_key_id&quot;).value;
var drawdown_key=document.getElementById(&quot;drawdown_key_id&quot;).value;var investment_Account_key=document.getElementById(&quot;investment_Account_key_id&quot;).value;var More_Addresses_key=document.getElementById(&quot;More_Addresses_key_id&quot;).value;var Current_Account_key=document.getElementById(&quot;Current_Account_key_id&quot;).value;var Additional_Fields_key=document.getElementById(&quot;Additional_Fields_key_id&quot;).value;var error_IRIS_Enrol_Key=document.getElementById(&quot;error_IRIS_Enrol_Key_id&quot;).value;var Choose_customer_key=document.getElementById(&quot;Choose_customer_key_id&quot;).value;
var Clear_customer_key=document.getElementById(&quot;Clear_customer_key_id&quot;).value;var imgNotAvailable=document.getElementById(&quot;imgNotAvailable_id&quot;).value;var Print_Swift_key=document.getElementById(&quot;Print_Swift_key_id&quot;).value;var Print_Swift_must_be_related_to_a_record_key=document.getElementById(&quot;Print_Swift_must_be_related_to_a_record_key_id&quot;).value;var signature_popup_title=document.getElementById(&quot;signature_popup_title_id&quot;).value;var signature_close_btn=document.getElementById(&quot;signature_close_btn_id&quot;).value;
var signature_action_title=document.getElementById(&quot;signature_action_title_id&quot;).value;var no_appr_sign_msg=document.getElementById(&quot;no_appr_sign_msg_id&quot;).value;var sign_signature_msg=document.getElementById(&quot;sign_signature_msg_id&quot;).value;var gen_ann_saved_msg=document.getElementById(&quot;gen_ann_saved_msg_id&quot;).value;var desc_mandatory_msg=document.getElementById(&quot;desc_mandatory_msg_id&quot;).value;var choose_doc_msg=document.getElementById(&quot;choose_doc_msg_id&quot;).value;var sign_copied=document.getElementById(&quot;sign_copied_id&quot;).value;
var signature_key=document.getElementById(&quot;signature_key_id&quot;).value;var jvs_list_key=document.getElementById(&quot;jvs_list_key_id&quot;).value;var Accept_key=document.getElementById(&quot;Accept_key_id&quot;).value;var Reject_Deleted_key=document.getElementById(&quot;Reject_Deleted_key_id&quot;).value;var Reject_Active_Deleted_key=document.getElementById(&quot;Reject_Active_Deleted_key_id&quot;).value;var english_key=document.getElementById(&quot;english_key_id&quot;).value;var arabic_key=document.getElementById(&quot;arabic_key_id&quot;).value;
var french_key=document.getElementById(&quot;french_key_id&quot;).value;var choose_language_key=document.getElementById(&quot;choose_language_key_id&quot;).value;var status_list_key=document.getElementById(&quot;status_list_key_id&quot;).value;var stat_of_account_key=document.getElementById(&quot;stat_of_account_key_id&quot;).value;var advice_key=document.getElementById(&quot;advice_key_id&quot;).value;var opening_acc_key=document.getElementById(&quot;opening_acc_key_id&quot;).value;var specify_report_key=document.getElementById(&quot;specify_report_key_id&quot;).value;
var whiteListed_key=document.getElementById(&quot;whiteListed_key_id&quot;).value;var company_key=document.getElementById(&quot;company_key_id&quot;).value;var branch_key=document.getElementById(&quot;branch_key_id&quot;).value;var cif_key=document.getElementById(&quot;cif_key_id&quot;).value;var acc_key=document.getElementById(&quot;acc_key_id&quot;).value;var savedMsg=document.getElementById(&quot;savedMsg_id&quot;).value;var show_picture_title=document.getElementById(&quot;show_picture_title_id&quot;).value;var No_Data_Found_key=document.getElementById(&quot;No_Data_Found_key_id&quot;).value;
var details_key=document.getElementById(&quot;details_key_id&quot;).value;var Repayment_Plan_details_not_available_key=document.getElementById(&quot;Repayment_Plan_details_not_available_key_id&quot;).value;var alerts_key=document.getElementById(&quot;alerts_key_id&quot;).value;var suspicious_list_key=document.getElementById(&quot;suspicious_list_key_id&quot;).value;var passbook_pages_exhausted_key=document.getElementById(&quot;passbook_pages_exhausted_key_id&quot;).value;var passbook_flip_pages_key=document.getElementById(&quot;passbook_flip_pages_key_id&quot;).value;
var brought_forward_amount_key=document.getElementById(&quot;brought_forward_amount_key_id&quot;).value;var Missing_File_Code_key=document.getElementById(&quot;Missing_File_Code_key_id&quot;).value;var File_Names_Do_Not_Match_key=document.getElementById(&quot;File_Names_Do_Not_Match_key_id&quot;).value;var Process_Executed_Successfully_key=document.getElementById(&quot;Process_Executed_Successfully_key_id&quot;).value;var File_Input_Parameters_Mandatory_key=document.getElementById(&quot;File_Input_Parameters_Mandatory_key_id&quot;).value;
var Input_Tag_values_Mandatory_key=document.getElementById(&quot;Input_Tag_values_Mandatory_key_id&quot;).value;var Missing_File_Location_key=document.getElementById(&quot;Missing_File_Location_key_id&quot;).value;var dyn_import_File_Loc_key=document.getElementById(&quot;Missing_File_Location_key_id&quot;).value;var fav_maint_title_key=document.getElementById(&quot;fav_maint_title_key_id&quot;).value;var select_fav_screen_key=document.getElementById(&quot;select_fav_screen_key_id&quot;).value;var selection_leaf_node_key=document.getElementById(&quot;selection_leaf_node_key_id&quot;).value;
var app_infos_title_key=document.getElementById(&quot;app_infos_title_key_id&quot;).value;var confirm_del_widget_key=document.getElementById(&quot;confirm_del_widget_key_id&quot;).value;var widget_not_avail_key=document.getElementById(&quot;widget_not_avail_key_id&quot;).value;var drag_widget_key=document.getElementById(&quot;drag_widget_key_id&quot;).value;var loading_key=document.getElementById(&quot;loading_key_id&quot;).value;var app_not_found_key=document.getElementById(&quot;app_not_found_key_id&quot;).value;var no_alert_found_key=document.getElementById(&quot;no_alert_found_key_id&quot;).value;
var resize_widget_key=document.getElementById(&quot;resize_widget_key_id&quot;).value;var custom_wkspce_key=document.getElementById(&quot;custom_wkspce_key_id&quot;).value;var add_custom_widget_key=document.getElementById(&quot;add_custom_widget_key_id&quot;).value;var trans_key=document.getElementById(&quot;trans_key_id&quot;).value;var branch_is_now_opened_key=document.getElementById(&quot;branch_is_now_opened_key_id&quot;).value;var branch_is_now_closed_key=document.getElementById(&quot;branch_is_now_closed_key&quot;).value;var no_widgets_key=document.getElementById(&quot;no_widgets_key_id&quot;).value;
var change_pass=document.getElementById(&quot;change_pass_id&quot;).value;var description_label_key=document.getElementById(&quot;description_label_key_id&quot;).value;var select_screen_help_key=document.getElementById(&quot;select_screen_help_key_id&quot;).value;var record_lvl_att_key=document.getElementById(&quot;record_lvl_att_key_id&quot;).value;var record_lvl_att_wlog_key=document.getElementById(&quot;record_lvl_att_wlog_key_id&quot;).value;var entity_lvl_att_key=document.getElementById(&quot;entity_lvl_att_key_id&quot;).value;
var entity_lvl_att_wlog_key=document.getElementById(&quot;entity_lvl_att_wlog_key_id&quot;).value;var record_Logs_Key=document.getElementById(&quot;Record_Logs_Key_id&quot;).value;var for_key=document.getElementById(&quot;for_key_id&quot;).value;var toolTipTrans_key=document.getElementById(&quot;toolTipTrans_key_id&quot;).value;var grid_user_pref_key=document.getElementById(&quot;grid_user_pref_key_id&quot;).value;var tentative_Schedule_Created_msg_key=document.getElementById(&quot;tentative_Schedule_Created_msg_key_id&quot;).value;
var finalSignOffChecking_key=document.getElementById(&quot;finalSignOffChecking_key_id&quot;).value;var report_Return_Data_key=document.getElementById(&quot;report_Return_Data_key_id&quot;).value;var server_request_completes_key=document.getElementById(&quot;server_request_completes_key_id&quot;).value;var download_key=document.getElementById(&quot;download_key_id&quot;).value;var elemIdAndNameChecking_key=document.getElementById(&quot;elemIdAndNameChecking_key_id&quot;).value;var Selected_key=document.getElementById(&quot;Selected_key_id&quot;).value;
var oldValKey=document.getElementById(&quot;oldValKey_id&quot;).value;var grid_user_def_key=document.getElementById(&quot;grid_user_def_key_id&quot;).value;var status_cust_key=document.getElementById(&quot;status_cust_key_id&quot;).value;var token_key=document.getElementById(&quot;token_key_id&quot;).value;var closeDynScreen_key=document.getElementById(&quot;closeDynScreen_key_id&quot;).value;var smartLabel_title_key=document.getElementById(&quot;smartLabel_title_key_id&quot;).value;var January_key=document.getElementById(&quot;January_key_id&quot;).value;
var February_key=document.getElementById(&quot;February_key_id&quot;).value;var March_key=document.getElementById(&quot;March_key_id&quot;).value;var April_key=document.getElementById(&quot;April_key_id&quot;).value;var May_key=document.getElementById(&quot;May_key_id&quot;).value;var June_key=document.getElementById(&quot;June_key_id&quot;).value;var July_key=document.getElementById(&quot;July_key_id&quot;).value;var August_key=document.getElementById(&quot;August_key_id&quot;).value;var September_key=document.getElementById(&quot;September_key_id&quot;).value;
var October_key=document.getElementById(&quot;October_key_id&quot;).value;var November_key=document.getElementById(&quot;November_key_id&quot;).value;var December_key=document.getElementById(&quot;December_key_id&quot;).value;var invalid_widget_url=document.getElementById(&quot;invalid_widget_url_key&quot;).value;var server_conn_drop_key=document.getElementById(&quot;server_conn_drop_key_id&quot;).value;var yes_confirm=document.getElementById(&quot;yes_key_id&quot;).value;var no_confirm=document.getElementById(&quot;no_key_id&quot;).value;
var defEmptyRepConfirm_key=document.getElementById(&quot;defemptyrepconfirm_key&quot;).value;var defEmptyRep_key=document.getElementById(&quot;defemptyrep_key&quot;).value;var activex_disabled_key=document.getElementById(&quot;activex_disabled_key_id&quot;).value;var cannotProceed_key=document.getElementById(&quot;cannot_proceed_key&quot;).value;var hijriDate_cust_key=document.getElementById(&quot;hijriDate_cust_key&quot;).value;var view_grid_key=document.getElementById(&quot;view_grid_key&quot;).value;var live_search_key=document.getElementById(&quot;live_search_key&quot;).value;
var Confirm_Reverse_Process_key=document.getElementById(&quot;Confirm_Reverse_Process_key_id&quot;).value;var notifications_key=document.getElementById(&quot;notifications_key&quot;).value;var function_not_defined_key=document.getElementById(&quot;function_not_defined_key_id&quot;).value;var your_selection_will_be_lost_key=document.getElementById(&quot;your_selection_will_be_lost_key_id&quot;).value;var cannot_open_dyn_screen_key=document.getElementById(&quot;cannot_open_dyn_screen_key&quot;).value;var contact_screen_admin_key=document.getElementById(&quot;contact_screen_admin_key_id&quot;).value;
var component_not_defined_key=document.getElementById(&quot;component_not_defined_key_id&quot;).value;var link_dyn_scr_key=document.getElementById(&quot;link_dyn_scr_key_id&quot;).value;var file_key=document.getElementById(&quot;file_key&quot;).value;var line_key=document.getElementById(&quot;line_key&quot;).value;var column_key=document.getElementById(&quot;column_key&quot;).value;var scrn_doesnt_contain_components_key=document.getElementById(&quot;scrn_doesnt_contain_components_key&quot;).value;var digital_chnl_trans_key=document.getElementById(&quot;digital_chnl_trans_key_id&quot;).value;
var report_key=document.getElementById(&quot;report_key_trans_id&quot;).value;var report_error_key=document.getElementById(&quot;report_error_key_trans_id&quot;).value;var duplicate_elem_id_key=document.getElementById(&quot;duplicate_elem_id&quot;).value;var old_value_key=document.getElementById(&quot;old_value_key_id&quot;).value;  Menu Options/html[1]/body[@class=&quot;ui-layout-container&quot;]/div[@class=&quot;displayProgBar&quot;]/div[@class=&quot;ui-widget-shadow progBarBackGound&quot;]WarningPlease, select rowPrevNextMay 2022SuMoTuWeThFrSa12345678910111213141516171819202122232425262728293031    Reset User PreferencesAdd New TabWarningPlease, select rowLoading... Code Brief Description Long Description الوصف المختصر الوصف الكامل Additional Reference1ValueValueValueValueR12Value JointValue JointValue JointValue JointR23MinorMinorMinorMinorR34SpecialSpecialSpecialSpecialR45Special JointSpecial JointSpecial JointSpecial JointR5 Page  of 65101520View1-5of28WarningPlease, select rowLoading... Code Brief Description الوصف المختصر Code1National IDالبطاقة الشخصية 3Passportجواز سفر 5Resident IDبطاقه المقيم 7Com. Reg.تسجيل الشركة 8Other IDبطاقة أخرى  Page  of 35101520View1-5of12&quot;))]</value>
      <webElementGuid>39fe7dce-19ff-4901-ba38-5a2815ebeb95</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
