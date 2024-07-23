#[cfg(test)]
mod problem_368 {
    use leetcode::problem_368::largest_divisible_subset::{
        largest_divisible_subset,
        largest_divisible_subset_optimized,
        lds
    };

    mod unoptimized_solution {
        use super::*;

        #[test]
        fn with_continuous_numbers() {
            let numbers = vec![1,2,3];
            assert_eq!(largest_divisible_subset(numbers), [1,2]);
        }
        #[test]
        fn with_powers_of_two() {
            let numbers = vec![1,2,4,8];
            assert_eq!(largest_divisible_subset(numbers), [1,2,4,8]);
        }
        #[test]
        fn with_one_number() {
            let numbers = vec![265];
            assert_eq!(largest_divisible_subset(numbers), [265]);
        }
        #[test]
        fn with_even_numbers() {
            let numbers = vec![1,2,4,6,8,10,12,14,16,18,20,22,24,26,28,30,32,34,36,38,40,
                              42,44,46,48,50,52,54,56,58,60,62,64,66,68,70,72,74,76,78,80,82,84,86,
                              88,90,92,94,96,98,100,102,104,106,108,110,112,114];
            assert_eq!(largest_divisible_subset(numbers), [1,2,4,8,16,32,64]);
        }
        #[test]
        fn with_odd_and_even_interleaved_numbers() {
            let numbers = vec![1,2,4,3,9,27,108,81,144,540];
            assert_eq!(largest_divisible_subset(numbers), [1,3,9,27,108,540]);
        }
        #[test]
        fn with_misleading_first_number() {
            let numbers = vec![3,4,16,8];
            assert_eq!(largest_divisible_subset(numbers), [4,8,16]);
        }
    }

    mod partially_optimized_solution {
        use super::*;

        #[test]
        fn with_continuous_numbers() {
            let numbers = vec![1,2,3];
            assert_eq!(largest_divisible_subset_optimized(numbers), [1,2]);
        }

        #[test]
        fn with_powers_of_two() {
            let numbers = vec![1,2,4,8];
            assert_eq!(largest_divisible_subset_optimized(numbers), [1,2,4,8]);
        }

        #[test]
        fn with_one_number() {
            let numbers = vec![265];
            assert_eq!(largest_divisible_subset_optimized(numbers), [265]);
        }

        #[test]
        fn with_even_numbers() {
            let numbers = vec![1,2,4,6,8,10,12,14,16,18,20,22,24,26,28,30,32,34,36,38,40,
                               42,44,46,48,50,52,54,56,58,60,62,64,66,68,70,72,74,76,78,80,82,84,86,
                               88,90,92,94,96,98,100,102,104,106,108,110,112,114];
            assert_eq!(largest_divisible_subset_optimized(numbers), [1,2,4,8,16,32,64]);
        }

        #[test]
        fn with_odd_and_even_interleaved_numbers() {
            let numbers = vec![1,2,4,3,9,27,108,81,144,540];
            assert_eq!(largest_divisible_subset_optimized(numbers), [1,3,9,27,108,540]);
        }

        #[test]
        fn with_only_one_number_not_in_the_subset() {
            let numbers = vec![4,8,10,240];
            assert_eq!(largest_divisible_subset_optimized(numbers), [4,8,240]);
        }

        #[test]
        fn with_misleading_first_number() {
            let numbers = vec![3,4,16,8];
            assert_eq!(largest_divisible_subset_optimized(numbers), [4,8,16]);
        }
    }

    mod fully_optimized_solution {
        use super::*;

        #[test]
        fn with_continuous_numbers() {
            let numbers = vec![1,2,3];
            assert_eq!(largest_divisible_subset_optimized(numbers), [1,2]);
        }
        #[test]
        fn with_powers_of_two() {
            let numbers = vec![1,2,4,8];
            assert_eq!(largest_divisible_subset_optimized(numbers), [1,2,4,8]);
        }
        #[test]
        fn with_one_number() {
            let numbers = vec![265];
            assert_eq!(largest_divisible_subset_optimized(numbers), [265]);
        }
        #[test]
        fn with_even_numbers() {
            let numbers = vec![1,2,4,6,8,10,12,14,16,18,20,22,24,26,28,30,32,34,36,38,40,
                               42,44,46,48,50,52,54,56,58,60,62,64,66,68,70,72,74,76,78,80,82,84,86,
                               88,90,92,94,96,98,100,102,104,106,108,110,112,114];
            assert_eq!(largest_divisible_subset_optimized(numbers), [1,2,4,8,16,32,64]);
        }
        #[test]
        fn with_odd_and_even_interleaved_numbers() {
            let numbers = vec![1,2,4,3,9,27,108,81,144,540];
            assert_eq!(largest_divisible_subset_optimized(numbers), [1,3,9,27,108,540]);
        }
        #[test]
        fn with_only_one_number_not_in_the_subset() {
            let numbers = vec![4,8,10,240];
            assert_eq!(largest_divisible_subset_optimized(numbers), [4,8,240]);
        }
        #[test]
        fn with_misleading_first_number() {
            let numbers = vec![3,4,16,8];
            assert_eq!(largest_divisible_subset_optimized(numbers), [4,8,16]);
        }

        #[test]
        fn with_a_lot_of_numbers_and_equal_sized_subsets() {
            let numbers = vec![1,7,49,343,2401,16807,117649,823543,5764801,40353607,5,35,
                               245,1715,12005,84035,588245,4117715,28824005,201768035,25,175,1225,
                               8575,60025,420175,2941225,20588575,144120025,1008840175,125,875,6125,
                               42875,300125,2100875,14706125,102942875,720600125,625,4375,30625,
                               214375,1500625,10504375,73530625,514714375,3125,21875,153125,1071875,
                               7503125,52521875,367653125,15625,109375,765625,5359375,37515625,
                               262609375,1838265625,78125,546875,3828125,26796875,187578125,
                               1313046875,390625,2734375,19140625,133984375,937890625,1953125,
                               13671875,95703125,669921875,9765625,68359375,478515625,48828125,
                               341796875,3,21,147,1029,7203,50421,352947,2470629,17294403,121060821,
                               15,105,735,5145,36015,252105,1764735,12353145,86472015,605304105,75,
                               525,3675,25725,180075,1260525,8823675,61765725,432360075,375,2625,
                               18375,128625,900375,6302625,44118375,308828625,1875,13125,91875,
                               643125,4501875,31513125,220591875,1544143125,9375,65625,459375,
                               3215625,22509375,157565625,1102959375,46875,328125,2296875,16078125,
                               112546875,787828125,234375,1640625,11484375,80390625,562734375,
                               1171875,8203125,57421875,401953125,5859375,41015625,287109375,
                               29296875,205078125,1435546875,146484375,1025390625,9,63,441,3087,
                               21609,151263,1058841,7411887,51883209,363182463,45,315,2205,15435,
                               108045,756315,5294205,37059435,259416045,1815912315,225,1575,11025,
                               77175,540225,3781575,26471025,185297175,1297080225,1125,7875,55125,
                               385875,2701125,18907875,132355125,926485875,5625,39375,275625,1929375,
                               13505625,94539375,661775625,28125,196875,1378125,9646875,67528125,
                               472696875,140625,984375,6890625,48234375,337640625,703125,4921875,
                               34453125,241171875,1688203125,3515625,24609375,172265625,1205859375,
                               17578125,123046875,861328125,87890625,615234375,439453125,27,189,1323,
                               9261,64827,453789,3176523,22235661,155649627,1089547389,135,945,6615,
                               46305,324135,2268945,15882615,111178305,778248135,675,4725,33075,
                               231525,1620675,11344725,79413075,555891525,3375,23625,165375,1157625,
                               8103375,56723625,397065375,16875,118125,826875,5788125,40516875,
                               283618125,1985326875,84375,590625,4134375,28940625,202584375,
                               1418090625,421875,2953125,20671875,144703125,1012921875,2109375,
                               14765625,103359375,723515625,10546875,73828125,516796875,52734375,
                               369140625,263671875,1845703125,1318359375,81,567,3969,27783,194481,
                               1361367,9529569,66706983,466948881,405,2835,19845,138915,972405,
                               6806835,47647845,333534915,2025,14175,99225,694575,4862025,34034175,
                               238239225,1667674575,10125,70875,496125,3472875,24310125,170170875,
                               1191196125,50625,354375,2480625,17364375,121550625,850854375,253125,
                               1771875,12403125,86821875,607753125,1265625,8859375,62015625,
                               434109375,6328125,44296875,310078125,31640625,221484375,1550390625,
                               158203125,1107421875,791015625,243,1701,11907,83349,583443,4084101,
                               28588707,200120949,1400846643,1215,8505,59535,416745,2917215,20420505,
                               142943535,1000604745,6075,42525,297675,2083725,14586075,102102525,
                               714717675,30375,212625,1488375,10418625,72930375,510512625,151875,
                               1063125,7441875,52093125,364651875,759375,5315625,37209375,260465625,
                               1823259375,3796875,26578125,186046875,1302328125,18984375,132890625,
                               930234375,94921875,664453125,474609375,729,5103,35721,250047,1750329,
                               12252303,85766121,600362847,3645,25515,178605,1250235,8751645,
                               61261515,428830605,18225,127575,893025,6251175,43758225,306307575,
                               91125,637875,4465125,31255875,218791125,1531537875,455625,3189375,
                               22325625,156279375,1093955625,2278125,15946875,111628125,781396875,
                               11390625,79734375,558140625,56953125,398671875,284765625,1993359375,
                               1423828125,2187,15309,107163,750141,5250987,36756909,257298363,
                               1801088541,10935,76545,535815,3750705,26254935,183784545,1286491815,
                               54675,382725,2679075,18753525,131274675,918922725,273375,1913625,
                               13395375,93767625,656373375,1366875,9568125,66976875,468838125,
                               6834375,47840625,334884375,34171875,239203125,1674421875,170859375,
                               1196015625,854296875,6561,45927,321489,2250423,15752961,110270727,
                               771895089,32805,229635,1607445,11252115,78764805,551353635,164025,
                               1148175,8037225,56260575,393824025,820125,5740875,40186125,281302875,
                               1969120125,4100625,28704375,200930625,1406514375,20503125,143521875,
                               1004653125,102515625,717609375,512578125,19683,137781,964467,6751269,
                               47258883,330812181,98415,688905,4822335,33756345,236294415,1654060905,
                               492075,3444525,24111675,168781725,1181472075,2460375,17222625,
                               120558375,843908625,12301875,86113125,602791875,61509375,430565625,
                               307546875,1537734375,59049,413343,2893401,20253807,141776649,
                               992436543,295245,2066715,14467005,101269035,708883245,1476225,
                               10333575,72335025,506345175,7381125,51667875,361675125,36905625,
                               258339375,1808375625,184528125,1291696875,922640625,177147,1240029,
                               8680203,60761421,425329947,885735,6200145,43401015,303807105,4428675,
                               31000725,217005075,1519035525,22143375,155003625,1085025375,
                               110716875,775018125,553584375,531441,3720087,26040609,182284263,
                               1275989841,2657205,18600435,130203045,911421315,13286025,93002175,
                               651015225,66430125,465010875,332150625,1660753125,1594323,11160261,
                               78121827,546852789,7971615,55801305,390609135,39858075,279006525,
                               1953045675,199290375,1395032625,996451875,4782969,33480783,234365481,
                               1640558367,23914845,167403915,1171827405,119574225,837019575,
                               597871125,2,14,98,686,4802,33614,235298,1647086,11529602,80707214,10,
                               70,490,3430,24010,168070,1176490,8235430,57648010,403536070,50,350,
                               2450,17150,120050,840350,5882450,41177150,288240050,250,1750,12250,
                               85750,600250,4201750,29412250,205885750,1441200250,1250,8750,61250,
                               428750,3001250,21008750,147061250,1029428750,6250,43750,306250,
                               2143750,15006250,105043750,735306250,31250,218750,1531250,10718750,
                               75031250,525218750,156250,1093750,7656250,53593750,375156250,781250,
                               5468750,38281250,267968750,1875781250,3906250,27343750,191406250,
                               1339843750,19531250,136718750,957031250,97656250,683593750,6,42,294,
                               2058,14406,100842,705894,4941258,34588806,242121642,30,210,1470,10290,
                               72030,504210,3529470,24706290,172944030,1210608210,150,1050,7350,
                               51450,360150,2521050,17647350,123531450,864720150,750,5250,36750,
                               257250,1800750,12605250,88236750,617657250,3750,26250,183750,1286250,
                               9003750,63026250,441183750,18750,131250,918750,6431250,45018750,
                               315131250,93750,656250,4593750,32156250,225093750,1575656250,468750,
                               3281250,22968750,160781250,1125468750,2343750,16406250,114843750,
                               803906250,11718750,82031250,574218750,58593750,410156250,292968750,18,
                               126,882,6174,43218,302526,2117682,14823774,103766418,726364926,90,630,
                               4410,30870,216090,1512630,10588410,74118870,518832090,450,3150,22050,
                               154350,1080450,7563150,52942050,370594350,2250,15750,110250,771750,
                               5402250,37815750,264710250,1852971750,11250,78750,551250,3858750,
                               27011250,189078750,1323551250,56250,393750,2756250,19293750,135056250,
                               945393750,281250,1968750,13781250,96468750,675281250,1406250,9843750,
                               68906250,482343750,7031250,49218750,344531250,35156250,246093750,
                               1722656250,175781250,1230468750,878906250,54,378,2646,18522,129654,
                               907578,6353046,44471322,311299254,270,1890,13230,92610,648270,4537890,
                               31765230,222356610,1556496270,1350,9450,66150,463050,3241350,22689450,
                               158826150,1111783050,6750,47250,330750,2315250,16206750,113447250,
                               794130750,33750,236250,1653750,11576250,81033750,567236250,168750,
                               1181250,8268750,57881250,405168750,843750,5906250,41343750,289406250,
                               4218750,29531250,206718750,1447031250,21093750,147656250,1033593750,
                               105468750,738281250,527343750,162,1134,7938,55566,388962,2722734,
                               19059138,133413966,933897762,810,5670,39690,277830,1944810,13613670,
                               95295690,667069830,4050,28350,198450,1389150,9724050,68068350,
                               476478450,20250,141750,992250,6945750,48620250,340341750,101250,
                               708750,4961250,34728750,243101250,1701708750,506250,3543750,24806250,
                               173643750,1215506250,2531250,17718750,124031250,868218750,12656250,
                               88593750,620156250,63281250,442968750,316406250,1582031250,486,3402,
                               23814,166698,1166886,8168202,57177414,400241898,2430,17010,119070,
                               833490,5834430,40841010,285887070,12150,85050,595350,4167450,29172150,
                               204205050,1429435350,60750,425250,2976750,20837250,145860750,
                               1021025250,303750,2126250,14883750,104186250,729303750,1518750,
                               10631250,74418750,520931250,7593750,53156250,372093750,37968750,
                               265781250,1860468750,189843750,1328906250,949218750,1458,10206,71442,
                               500094,3500658,24504606,171532242];
            let subset_one = vec![1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049, 177147, 531441, 1594323, 4782969, 23914845, 119574225, 597871125];
            let subset_two = vec![1, 7, 49, 343, 1029, 3087, 9261, 27783, 83349, 250047, 750141, 2250423, 6751269, 20253807, 60761421, 182284263, 546852789, 1640558367];

            let result = lds(numbers);

            assert!(result == subset_one || result == subset_two);
        }
    }
}