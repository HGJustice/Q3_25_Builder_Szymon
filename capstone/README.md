# Capstone - Decentralised Mealprep

A Solana-based meal prep platform that directly connects busy customers with local home cooks to provide access to genuinely traditional meals that corporate kitchens can't replicate, ensuring money flows directly to local communites rather than corporate stakerholders.

# How it works / Diagram

**Market initialisation**

Firstly the admin initializes the marketplace, by selecting the fee-rate. The main account for this account that is also being created is the treasury account which will hold the lamports fees collected from the complete orders. The marketplace also keeps track of the number of all active listings and orders.

**User initialisation**

When potential customers come to the dApp, we categorise them into two actors. Cook and Customer. When initializing a User account they specify their username, location and user type.

**Order initialisation**

Cooks then creates listing accounts, containing information like mealName, description, pricePerMeal, mealsAvailable, deliveryAvailable, deliveryFee.

**Listing initialisation**

Then the customers browse these listings, and find an appropriate one with the amount they wish to buy, they will initialize the order which takes listingID, and the amount for the portion. This will then create the Order account, send the appropriate lamports to the vault account where the funds will be held, and the cook is notified of the newly created order.

**Change Order Status**

This is the section where the cook alongside the customer will update their order statuses according to what's going on with the order. Once the customer sets his order status to Collected and the cook has his set on the Complete, then the cook will be able to withdraw funds from the vault.

**Cook withdraws from vault && marketplace fees get sent to treasury**

Once the order status for both customer and cook is on their confirmed states. Then the cook will be able to withdraw their payment for the meal prep. In the function before sending the cook their lamports, we first take away the marketplace fee %, from the vault's total lamport balance. Then sending the fees to the marketplace treasury, while the rest gets sent to the cook.

**Admin withdraws fees**

Once the admin of the marketplace sees that enough orders have been completed. They can withdraw the lamport fees collected to their wallet.

![Architecture Design](./architecture_design.jpg)

# Tests

Overall there are 8 total tests that test 4 total accounts and their functionality, that being marketplace, user, listing and order.

**User**

In the user testing section, we are checking if the instantiation of the user accounts works correctly with the correct data passed like username, location and if the proper userType is assigned to them correctly like cook or customer.

**Marketplace && Listing**

In the marketplace and listing section we will be instantiating the marketplace account, and checking if the proper fees were placed. We also create a listing account using the cook account, to check if proper data was placed such as meal name, amount of meals, price per meal etc.

In the marketplace account, we also test the withdraw fees function that can only be called by the admin, to check if admins
balance increases after function is called.

**Order**

In the order tests, we first check if the instantiation of the order account was correct with the proper data, and if it took the correct amount of lamports, and placed them in the vault.

Then we check the changeOrderStatus function on both customer and cook, to test if the  
function works and if its updating the status correctly.

Then we test the closeOrder function, it first checks to make sure that both the customer
and the cook confirmed that they gave out and received the order. Once both parties have confirmed then the cook can close the order and part of the fee will be sent to the marketplace treasury and the rest of the lamports to the cook.

# Links

**Pitch Deck:** https://www.canva.com/design/DAGwKqh3lOA/Lxd0H-hc3_YvxlVIB7PT_w/edit?utm_content=DAGwKqh3lOA&utm_campaign=designshare&utm_medium=link2&utm_source=sharebutton

**Program Devnet Address:** https://solscan.io/account/7oCizVtCCp4VuFjWf4wbNzxpPckLZTSABP9YyBuD8oqa?cluster=devnet
