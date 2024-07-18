module Quickfix
	class BeginSeqNo < Quickfix::IntField
		def BeginSeqNo.field
			return 7
		end
		def initialize(data = nil)
			if( data == nil )
				super(7)
			else
				super(7, data)
			end
		end
	end

	class BeginString < Quickfix::StringField
		def BeginString.field
			return 8
		end
		def initialize(data = nil)
			if( data == nil )
				super(8)
			else
				super(8, data)
			end
		end
	end

	class BodyLength < Quickfix::IntField
		def BodyLength.field
			return 9
		end
		def initialize(data = nil)
			if( data == nil )
				super(9)
			else
				super(9, data)
			end
		end
	end

	class CheckSum < Quickfix::CheckSumField
		def CheckSum.field
			return 10
		end
		def initialize(data = nil)
			if( data == nil )
				super(10)
			else
				super(10, data)
			end
		end
	end

	class EndSeqNo < Quickfix::IntField
		def EndSeqNo.field
			return 16
		end
		def initialize(data = nil)
			if( data == nil )
				super(16)
			else
				super(16, data)
			end
		end
	end

	class MsgSeqNum < Quickfix::IntField
		def MsgSeqNum.field
			return 34
		end
		def initialize(data = nil)
			if( data == nil )
				super(34)
			else
				super(34, data)
			end
		end
	end

	class MsgType < Quickfix::StringField
		def MsgType.field
			return 35
		end
		def initialize(data = nil)
			if( data == nil )
				super(35)
			else
				super(35, data)
			end
		end
	end

	class NewSeqNo < Quickfix::IntField
		def NewSeqNo.field
			return 36
		end
		def initialize(data = nil)
			if( data == nil )
				super(36)
			else
				super(36, data)
			end
		end
	end

	class PossDupFlag < Quickfix::BoolField
		def PossDupFlag.field
			return 43
		end
		def initialize(data = nil)
			if( data == nil )
				super(43)
			else
				super(43, data)
			end
		end
	end

	class RefSeqNum < Quickfix::IntField
		def RefSeqNum.field
			return 45
		end
		def initialize(data = nil)
			if( data == nil )
				super(45)
			else
				super(45, data)
			end
		end
	end

	class SenderCompID < Quickfix::StringField
		def SenderCompID.field
			return 49
		end
		def initialize(data = nil)
			if( data == nil )
				super(49)
			else
				super(49, data)
			end
		end
	end

	class SenderSubID < Quickfix::StringField
		def SenderSubID.field
			return 50
		end
		def initialize(data = nil)
			if( data == nil )
				super(50)
			else
				super(50, data)
			end
		end
	end

	class SendingTime < Quickfix::UtcTimeStampField
		def SendingTime.field
			return 52
		end
		def initialize(data = nil)
			if( data == nil )
				super(52)
			else
				super(52, data)
			end
		end
	end

	class TargetCompID < Quickfix::StringField
		def TargetCompID.field
			return 56
		end
		def initialize(data = nil)
			if( data == nil )
				super(56)
			else
				super(56, data)
			end
		end
	end

	class TargetSubID < Quickfix::StringField
		def TargetSubID.field
			return 57
		end
		def initialize(data = nil)
			if( data == nil )
				super(57)
			else
				super(57, data)
			end
		end
	end

	class Text < Quickfix::StringField
		def Text.field
			return 58
		end
		def initialize(data = nil)
			if( data == nil )
				super(58)
			else
				super(58, data)
			end
		end
	end

	class Signature < Quickfix::StringField
		def Signature.field
			return 89
		end
		def initialize(data = nil)
			if( data == nil )
				super(89)
			else
				super(89, data)
			end
		end
	end

	class SecureDataLen < Quickfix::IntField
		def SecureDataLen.field
			return 90
		end
		def initialize(data = nil)
			if( data == nil )
				super(90)
			else
				super(90, data)
			end
		end
	end

	class SecureData < Quickfix::StringField
		def SecureData.field
			return 91
		end
		def initialize(data = nil)
			if( data == nil )
				super(91)
			else
				super(91, data)
			end
		end
	end

	class SignatureLength < Quickfix::IntField
		def SignatureLength.field
			return 93
		end
		def initialize(data = nil)
			if( data == nil )
				super(93)
			else
				super(93, data)
			end
		end
	end

	class RawDataLength < Quickfix::IntField
		def RawDataLength.field
			return 95
		end
		def initialize(data = nil)
			if( data == nil )
				super(95)
			else
				super(95, data)
			end
		end
	end

	class RawData < Quickfix::StringField
		def RawData.field
			return 96
		end
		def initialize(data = nil)
			if( data == nil )
				super(96)
			else
				super(96, data)
			end
		end
	end

	class PossResend < Quickfix::BoolField
		def PossResend.field
			return 97
		end
		def initialize(data = nil)
			if( data == nil )
				super(97)
			else
				super(97, data)
			end
		end
	end

	class EncryptMethod < Quickfix::IntField
		def EncryptMethod.field
			return 98
		end
		def initialize(data = nil)
			if( data == nil )
				super(98)
			else
				super(98, data)
			end
		end
	end

	class HeartBtInt < Quickfix::IntField
		def HeartBtInt.field
			return 108
		end
		def initialize(data = nil)
			if( data == nil )
				super(108)
			else
				super(108, data)
			end
		end
	end

	class TestReqID < Quickfix::StringField
		def TestReqID.field
			return 112
		end
		def initialize(data = nil)
			if( data == nil )
				super(112)
			else
				super(112, data)
			end
		end
	end

	class OnBehalfOfCompID < Quickfix::StringField
		def OnBehalfOfCompID.field
			return 115
		end
		def initialize(data = nil)
			if( data == nil )
				super(115)
			else
				super(115, data)
			end
		end
	end

	class OnBehalfOfSubID < Quickfix::StringField
		def OnBehalfOfSubID.field
			return 116
		end
		def initialize(data = nil)
			if( data == nil )
				super(116)
			else
				super(116, data)
			end
		end
	end

	class OrigSendingTime < Quickfix::UtcTimeStampField
		def OrigSendingTime.field
			return 122
		end
		def initialize(data = nil)
			if( data == nil )
				super(122)
			else
				super(122, data)
			end
		end
	end

	class GapFillFlag < Quickfix::BoolField
		def GapFillFlag.field
			return 123
		end
		def initialize(data = nil)
			if( data == nil )
				super(123)
			else
				super(123, data)
			end
		end
	end

	class DeliverToCompID < Quickfix::StringField
		def DeliverToCompID.field
			return 128
		end
		def initialize(data = nil)
			if( data == nil )
				super(128)
			else
				super(128, data)
			end
		end
	end

	class DeliverToSubID < Quickfix::StringField
		def DeliverToSubID.field
			return 129
		end
		def initialize(data = nil)
			if( data == nil )
				super(129)
			else
				super(129, data)
			end
		end
	end

	class ResetSeqNumFlag < Quickfix::BoolField
		def ResetSeqNumFlag.field
			return 141
		end
		def initialize(data = nil)
			if( data == nil )
				super(141)
			else
				super(141, data)
			end
		end
	end

	class SenderLocationID < Quickfix::StringField
		def SenderLocationID.field
			return 142
		end
		def initialize(data = nil)
			if( data == nil )
				super(142)
			else
				super(142, data)
			end
		end
	end

	class TargetLocationID < Quickfix::StringField
		def TargetLocationID.field
			return 143
		end
		def initialize(data = nil)
			if( data == nil )
				super(143)
			else
				super(143, data)
			end
		end
	end

	class OnBehalfOfLocationID < Quickfix::StringField
		def OnBehalfOfLocationID.field
			return 144
		end
		def initialize(data = nil)
			if( data == nil )
				super(144)
			else
				super(144, data)
			end
		end
	end

	class DeliverToLocationID < Quickfix::StringField
		def DeliverToLocationID.field
			return 145
		end
		def initialize(data = nil)
			if( data == nil )
				super(145)
			else
				super(145, data)
			end
		end
	end

	class XmlDataLen < Quickfix::IntField
		def XmlDataLen.field
			return 212
		end
		def initialize(data = nil)
			if( data == nil )
				super(212)
			else
				super(212, data)
			end
		end
	end

	class XmlData < Quickfix::StringField
		def XmlData.field
			return 213
		end
		def initialize(data = nil)
			if( data == nil )
				super(213)
			else
				super(213, data)
			end
		end
	end

	class MessageEncoding < Quickfix::StringField
		def MessageEncoding.field
			return 347
		end
		def initialize(data = nil)
			if( data == nil )
				super(347)
			else
				super(347, data)
			end
		end
	end

	class EncodedTextLen < Quickfix::IntField
		def EncodedTextLen.field
			return 354
		end
		def initialize(data = nil)
			if( data == nil )
				super(354)
			else
				super(354, data)
			end
		end
	end

	class EncodedText < Quickfix::StringField
		def EncodedText.field
			return 355
		end
		def initialize(data = nil)
			if( data == nil )
				super(355)
			else
				super(355, data)
			end
		end
	end

	class LastMsgSeqNumProcessed < Quickfix::IntField
		def LastMsgSeqNumProcessed.field
			return 369
		end
		def initialize(data = nil)
			if( data == nil )
				super(369)
			else
				super(369, data)
			end
		end
	end

	class RefTagID < Quickfix::IntField
		def RefTagID.field
			return 371
		end
		def initialize(data = nil)
			if( data == nil )
				super(371)
			else
				super(371, data)
			end
		end
	end

	class RefMsgType < Quickfix::StringField
		def RefMsgType.field
			return 372
		end
		def initialize(data = nil)
			if( data == nil )
				super(372)
			else
				super(372, data)
			end
		end
	end

	class SessionRejectReason < Quickfix::IntField
		def SessionRejectReason.field
			return 373
		end
		def initialize(data = nil)
			if( data == nil )
				super(373)
			else
				super(373, data)
			end
		end
	end

	class MaxMessageSize < Quickfix::IntField
		def MaxMessageSize.field
			return 383
		end
		def initialize(data = nil)
			if( data == nil )
				super(383)
			else
				super(383, data)
			end
		end
	end

	class TestMessageIndicator < Quickfix::BoolField
		def TestMessageIndicator.field
			return 464
		end
		def initialize(data = nil)
			if( data == nil )
				super(464)
			else
				super(464, data)
			end
		end
	end

	class Username < Quickfix::StringField
		def Username.field
			return 553
		end
		def initialize(data = nil)
			if( data == nil )
				super(553)
			else
				super(553, data)
			end
		end
	end

	class Password < Quickfix::StringField
		def Password.field
			return 554
		end
		def initialize(data = nil)
			if( data == nil )
				super(554)
			else
				super(554, data)
			end
		end
	end

	class NoHops < Quickfix::IntField
		def NoHops.field
			return 627
		end
		def initialize(data = nil)
			if( data == nil )
				super(627)
			else
				super(627, data)
			end
		end
	end

	class HopCompID < Quickfix::StringField
		def HopCompID.field
			return 628
		end
		def initialize(data = nil)
			if( data == nil )
				super(628)
			else
				super(628, data)
			end
		end
	end

	class HopSendingTime < Quickfix::UtcTimeStampField
		def HopSendingTime.field
			return 629
		end
		def initialize(data = nil)
			if( data == nil )
				super(629)
			else
				super(629, data)
			end
		end
	end

	class HopRefID < Quickfix::IntField
		def HopRefID.field
			return 630
		end
		def initialize(data = nil)
			if( data == nil )
				super(630)
			else
				super(630, data)
			end
		end
	end

	class NextExpectedMsgSeqNum < Quickfix::IntField
		def NextExpectedMsgSeqNum.field
			return 789
		end
		def initialize(data = nil)
			if( data == nil )
				super(789)
			else
				super(789, data)
			end
		end
	end

	class NewPassword < Quickfix::StringField
		def NewPassword.field
			return 925
		end
		def initialize(data = nil)
			if( data == nil )
				super(925)
			else
				super(925, data)
			end
		end
	end

	class ApplVerID < Quickfix::StringField
		def ApplVerID.field
			return 1128
		end
		def initialize(data = nil)
			if( data == nil )
				super(1128)
			else
				super(1128, data)
			end
		end
	end

	class CstmApplVerID < Quickfix::StringField
		def CstmApplVerID.field
			return 1129
		end
		def initialize(data = nil)
			if( data == nil )
				super(1129)
			else
				super(1129, data)
			end
		end
	end

	class RefApplVerID < Quickfix::StringField
		def RefApplVerID.field
			return 1130
		end
		def initialize(data = nil)
			if( data == nil )
				super(1130)
			else
				super(1130, data)
			end
		end
	end

	class RefCstmApplVerID < Quickfix::StringField
		def RefCstmApplVerID.field
			return 1131
		end
		def initialize(data = nil)
			if( data == nil )
				super(1131)
			else
				super(1131, data)
			end
		end
	end

	class DefaultApplVerID < Quickfix::StringField
		def DefaultApplVerID.field
			return 1137
		end
		def initialize(data = nil)
			if( data == nil )
				super(1137)
			else
				super(1137, data)
			end
		end
	end

	class ApplExtID < Quickfix::IntField
		def ApplExtID.field
			return 1156
		end
		def initialize(data = nil)
			if( data == nil )
				super(1156)
			else
				super(1156, data)
			end
		end
	end

	class EncryptedPasswordMethod < Quickfix::IntField
		def EncryptedPasswordMethod.field
			return 1400
		end
		def initialize(data = nil)
			if( data == nil )
				super(1400)
			else
				super(1400, data)
			end
		end
	end

	class EncryptedPasswordLen < Quickfix::IntField
		def EncryptedPasswordLen.field
			return 1401
		end
		def initialize(data = nil)
			if( data == nil )
				super(1401)
			else
				super(1401, data)
			end
		end
	end

	class EncryptedPassword < Quickfix::StringField
		def EncryptedPassword.field
			return 1402
		end
		def initialize(data = nil)
			if( data == nil )
				super(1402)
			else
				super(1402, data)
			end
		end
	end

	class EncryptedNewPasswordLen < Quickfix::IntField
		def EncryptedNewPasswordLen.field
			return 1403
		end
		def initialize(data = nil)
			if( data == nil )
				super(1403)
			else
				super(1403, data)
			end
		end
	end

	class EncryptedNewPassword < Quickfix::StringField
		def EncryptedNewPassword.field
			return 1404
		end
		def initialize(data = nil)
			if( data == nil )
				super(1404)
			else
				super(1404, data)
			end
		end
	end

	class RefApplExtID < Quickfix::IntField
		def RefApplExtID.field
			return 1406
		end
		def initialize(data = nil)
			if( data == nil )
				super(1406)
			else
				super(1406, data)
			end
		end
	end

	class DefaultApplExtID < Quickfix::IntField
		def DefaultApplExtID.field
			return 1407
		end
		def initialize(data = nil)
			if( data == nil )
				super(1407)
			else
				super(1407, data)
			end
		end
	end

	class DefaultCstmApplVerID < Quickfix::StringField
		def DefaultCstmApplVerID.field
			return 1408
		end
		def initialize(data = nil)
			if( data == nil )
				super(1408)
			else
				super(1408, data)
			end
		end
	end

	class SessionStatus < Quickfix::IntField
		def SessionStatus.field
			return 1409
		end
		def initialize(data = nil)
			if( data == nil )
				super(1409)
			else
				super(1409, data)
			end
		end
	end

	class Account < Quickfix::StringField
		def Account.field
			return 1
		end
		def initialize(data = nil)
			if( data == nil )
				super(1)
			else
				super(1, data)
			end
		end
	end

	class AdvId < Quickfix::StringField
		def AdvId.field
			return 2
		end
		def initialize(data = nil)
			if( data == nil )
				super(2)
			else
				super(2, data)
			end
		end
	end

	class AdvRefID < Quickfix::StringField
		def AdvRefID.field
			return 3
		end
		def initialize(data = nil)
			if( data == nil )
				super(3)
			else
				super(3, data)
			end
		end
	end

	class AdvSide < Quickfix::CharField
		def AdvSide.field
			return 4
		end
		def initialize(data = nil)
			if( data == nil )
				super(4)
			else
				super(4, data)
			end
		end
	end

	class AdvTransType < Quickfix::StringField
		def AdvTransType.field
			return 5
		end
		def initialize(data = nil)
			if( data == nil )
				super(5)
			else
				super(5, data)
			end
		end
	end

	class AvgPx < Quickfix::DoubleField
		def AvgPx.field
			return 6
		end
		def initialize(data = nil)
			if( data == nil )
				super(6)
			else
				super(6, data)
			end
		end
	end

	class ClOrdID < Quickfix::StringField
		def ClOrdID.field
			return 11
		end
		def initialize(data = nil)
			if( data == nil )
				super(11)
			else
				super(11, data)
			end
		end
	end

	class Commission < Quickfix::DoubleField
		def Commission.field
			return 12
		end
		def initialize(data = nil)
			if( data == nil )
				super(12)
			else
				super(12, data)
			end
		end
	end

	class CommType < Quickfix::CharField
		def CommType.field
			return 13
		end
		def initialize(data = nil)
			if( data == nil )
				super(13)
			else
				super(13, data)
			end
		end
	end

	class CumQty < Quickfix::DoubleField
		def CumQty.field
			return 14
		end
		def initialize(data = nil)
			if( data == nil )
				super(14)
			else
				super(14, data)
			end
		end
	end

	class Currency < Quickfix::StringField
		def Currency.field
			return 15
		end
		def initialize(data = nil)
			if( data == nil )
				super(15)
			else
				super(15, data)
			end
		end
	end

	class ExecID < Quickfix::StringField
		def ExecID.field
			return 17
		end
		def initialize(data = nil)
			if( data == nil )
				super(17)
			else
				super(17, data)
			end
		end
	end

	class ExecInst < Quickfix::StringField
		def ExecInst.field
			return 18
		end
		def initialize(data = nil)
			if( data == nil )
				super(18)
			else
				super(18, data)
			end
		end
	end

	class ExecRefID < Quickfix::StringField
		def ExecRefID.field
			return 19
		end
		def initialize(data = nil)
			if( data == nil )
				super(19)
			else
				super(19, data)
			end
		end
	end

	class ExecTransType < Quickfix::CharField
		def ExecTransType.field
			return 20
		end
		def initialize(data = nil)
			if( data == nil )
				super(20)
			else
				super(20, data)
			end
		end
	end

	class HandlInst < Quickfix::CharField
		def HandlInst.field
			return 21
		end
		def initialize(data = nil)
			if( data == nil )
				super(21)
			else
				super(21, data)
			end
		end
	end

	class IDSource < Quickfix::StringField
		def IDSource.field
			return 22
		end
		def initialize(data = nil)
			if( data == nil )
				super(22)
			else
				super(22, data)
			end
		end
	end

	class IOIid < Quickfix::StringField
		def IOIid.field
			return 23
		end
		def initialize(data = nil)
			if( data == nil )
				super(23)
			else
				super(23, data)
			end
		end
	end

	class IOIOthSvc < Quickfix::CharField
		def IOIOthSvc.field
			return 24
		end
		def initialize(data = nil)
			if( data == nil )
				super(24)
			else
				super(24, data)
			end
		end
	end

	class IOIQltyInd < Quickfix::CharField
		def IOIQltyInd.field
			return 25
		end
		def initialize(data = nil)
			if( data == nil )
				super(25)
			else
				super(25, data)
			end
		end
	end

	class IOIRefID < Quickfix::StringField
		def IOIRefID.field
			return 26
		end
		def initialize(data = nil)
			if( data == nil )
				super(26)
			else
				super(26, data)
			end
		end
	end

	class IOIShares < Quickfix::StringField
		def IOIShares.field
			return 27
		end
		def initialize(data = nil)
			if( data == nil )
				super(27)
			else
				super(27, data)
			end
		end
	end

	class IOITransType < Quickfix::CharField
		def IOITransType.field
			return 28
		end
		def initialize(data = nil)
			if( data == nil )
				super(28)
			else
				super(28, data)
			end
		end
	end

	class LastCapacity < Quickfix::CharField
		def LastCapacity.field
			return 29
		end
		def initialize(data = nil)
			if( data == nil )
				super(29)
			else
				super(29, data)
			end
		end
	end

	class LastMkt < Quickfix::StringField
		def LastMkt.field
			return 30
		end
		def initialize(data = nil)
			if( data == nil )
				super(30)
			else
				super(30, data)
			end
		end
	end

	class LastPx < Quickfix::DoubleField
		def LastPx.field
			return 31
		end
		def initialize(data = nil)
			if( data == nil )
				super(31)
			else
				super(31, data)
			end
		end
	end

	class LastShares < Quickfix::DoubleField
		def LastShares.field
			return 32
		end
		def initialize(data = nil)
			if( data == nil )
				super(32)
			else
				super(32, data)
			end
		end
	end

	class LinesOfText < Quickfix::IntField
		def LinesOfText.field
			return 33
		end
		def initialize(data = nil)
			if( data == nil )
				super(33)
			else
				super(33, data)
			end
		end
	end

	class OrderID < Quickfix::StringField
		def OrderID.field
			return 37
		end
		def initialize(data = nil)
			if( data == nil )
				super(37)
			else
				super(37, data)
			end
		end
	end

	class OrderQty < Quickfix::DoubleField
		def OrderQty.field
			return 38
		end
		def initialize(data = nil)
			if( data == nil )
				super(38)
			else
				super(38, data)
			end
		end
	end

	class OrdStatus < Quickfix::CharField
		def OrdStatus.field
			return 39
		end
		def initialize(data = nil)
			if( data == nil )
				super(39)
			else
				super(39, data)
			end
		end
	end

	class OrdType < Quickfix::CharField
		def OrdType.field
			return 40
		end
		def initialize(data = nil)
			if( data == nil )
				super(40)
			else
				super(40, data)
			end
		end
	end

	class OrigClOrdID < Quickfix::StringField
		def OrigClOrdID.field
			return 41
		end
		def initialize(data = nil)
			if( data == nil )
				super(41)
			else
				super(41, data)
			end
		end
	end

	class OrigTime < Quickfix::UtcTimeStampField
		def OrigTime.field
			return 42
		end
		def initialize(data = nil)
			if( data == nil )
				super(42)
			else
				super(42, data)
			end
		end
	end

	class Price < Quickfix::DoubleField
		def Price.field
			return 44
		end
		def initialize(data = nil)
			if( data == nil )
				super(44)
			else
				super(44, data)
			end
		end
	end

	class RelatdSym < Quickfix::StringField
		def RelatdSym.field
			return 46
		end
		def initialize(data = nil)
			if( data == nil )
				super(46)
			else
				super(46, data)
			end
		end
	end

	class Rule80A < Quickfix::CharField
		def Rule80A.field
			return 47
		end
		def initialize(data = nil)
			if( data == nil )
				super(47)
			else
				super(47, data)
			end
		end
	end

	class SecurityID < Quickfix::StringField
		def SecurityID.field
			return 48
		end
		def initialize(data = nil)
			if( data == nil )
				super(48)
			else
				super(48, data)
			end
		end
	end

	class Shares < Quickfix::DoubleField
		def Shares.field
			return 53
		end
		def initialize(data = nil)
			if( data == nil )
				super(53)
			else
				super(53, data)
			end
		end
	end

	class Side < Quickfix::CharField
		def Side.field
			return 54
		end
		def initialize(data = nil)
			if( data == nil )
				super(54)
			else
				super(54, data)
			end
		end
	end

	class Symbol < Quickfix::StringField
		def Symbol.field
			return 55
		end
		def initialize(data = nil)
			if( data == nil )
				super(55)
			else
				super(55, data)
			end
		end
	end

	class TimeInForce < Quickfix::CharField
		def TimeInForce.field
			return 59
		end
		def initialize(data = nil)
			if( data == nil )
				super(59)
			else
				super(59, data)
			end
		end
	end

	class TransactTime < Quickfix::UtcTimeStampField
		def TransactTime.field
			return 60
		end
		def initialize(data = nil)
			if( data == nil )
				super(60)
			else
				super(60, data)
			end
		end
	end

	class Urgency < Quickfix::CharField
		def Urgency.field
			return 61
		end
		def initialize(data = nil)
			if( data == nil )
				super(61)
			else
				super(61, data)
			end
		end
	end

	class ValidUntilTime < Quickfix::UtcTimeStampField
		def ValidUntilTime.field
			return 62
		end
		def initialize(data = nil)
			if( data == nil )
				super(62)
			else
				super(62, data)
			end
		end
	end

	class SettlmntTyp < Quickfix::CharField
		def SettlmntTyp.field
			return 63
		end
		def initialize(data = nil)
			if( data == nil )
				super(63)
			else
				super(63, data)
			end
		end
	end

	class FutSettDate < Quickfix::StringField
		def FutSettDate.field
			return 64
		end
		def initialize(data = nil)
			if( data == nil )
				super(64)
			else
				super(64, data)
			end
		end
	end

	class SymbolSfx < Quickfix::StringField
		def SymbolSfx.field
			return 65
		end
		def initialize(data = nil)
			if( data == nil )
				super(65)
			else
				super(65, data)
			end
		end
	end

	class ListID < Quickfix::StringField
		def ListID.field
			return 66
		end
		def initialize(data = nil)
			if( data == nil )
				super(66)
			else
				super(66, data)
			end
		end
	end

	class ListSeqNo < Quickfix::IntField
		def ListSeqNo.field
			return 67
		end
		def initialize(data = nil)
			if( data == nil )
				super(67)
			else
				super(67, data)
			end
		end
	end

	class ListNoOrds < Quickfix::IntField
		def ListNoOrds.field
			return 68
		end
		def initialize(data = nil)
			if( data == nil )
				super(68)
			else
				super(68, data)
			end
		end
	end

	class ListExecInst < Quickfix::StringField
		def ListExecInst.field
			return 69
		end
		def initialize(data = nil)
			if( data == nil )
				super(69)
			else
				super(69, data)
			end
		end
	end

	class AllocID < Quickfix::StringField
		def AllocID.field
			return 70
		end
		def initialize(data = nil)
			if( data == nil )
				super(70)
			else
				super(70, data)
			end
		end
	end

	class AllocTransType < Quickfix::CharField
		def AllocTransType.field
			return 71
		end
		def initialize(data = nil)
			if( data == nil )
				super(71)
			else
				super(71, data)
			end
		end
	end

	class RefAllocID < Quickfix::StringField
		def RefAllocID.field
			return 72
		end
		def initialize(data = nil)
			if( data == nil )
				super(72)
			else
				super(72, data)
			end
		end
	end

	class NoOrders < Quickfix::IntField
		def NoOrders.field
			return 73
		end
		def initialize(data = nil)
			if( data == nil )
				super(73)
			else
				super(73, data)
			end
		end
	end

	class AvgPrxPrecision < Quickfix::IntField
		def AvgPrxPrecision.field
			return 74
		end
		def initialize(data = nil)
			if( data == nil )
				super(74)
			else
				super(74, data)
			end
		end
	end

	class TradeDate < Quickfix::StringField
		def TradeDate.field
			return 75
		end
		def initialize(data = nil)
			if( data == nil )
				super(75)
			else
				super(75, data)
			end
		end
	end

	class ExecBroker < Quickfix::StringField
		def ExecBroker.field
			return 76
		end
		def initialize(data = nil)
			if( data == nil )
				super(76)
			else
				super(76, data)
			end
		end
	end

	class OpenClose < Quickfix::CharField
		def OpenClose.field
			return 77
		end
		def initialize(data = nil)
			if( data == nil )
				super(77)
			else
				super(77, data)
			end
		end
	end

	class NoAllocs < Quickfix::IntField
		def NoAllocs.field
			return 78
		end
		def initialize(data = nil)
			if( data == nil )
				super(78)
			else
				super(78, data)
			end
		end
	end

	class AllocAccount < Quickfix::StringField
		def AllocAccount.field
			return 79
		end
		def initialize(data = nil)
			if( data == nil )
				super(79)
			else
				super(79, data)
			end
		end
	end

	class AllocShares < Quickfix::DoubleField
		def AllocShares.field
			return 80
		end
		def initialize(data = nil)
			if( data == nil )
				super(80)
			else
				super(80, data)
			end
		end
	end

	class ProcessCode < Quickfix::CharField
		def ProcessCode.field
			return 81
		end
		def initialize(data = nil)
			if( data == nil )
				super(81)
			else
				super(81, data)
			end
		end
	end

	class NoRpts < Quickfix::IntField
		def NoRpts.field
			return 82
		end
		def initialize(data = nil)
			if( data == nil )
				super(82)
			else
				super(82, data)
			end
		end
	end

	class RptSeq < Quickfix::IntField
		def RptSeq.field
			return 83
		end
		def initialize(data = nil)
			if( data == nil )
				super(83)
			else
				super(83, data)
			end
		end
	end

	class CxlQty < Quickfix::DoubleField
		def CxlQty.field
			return 84
		end
		def initialize(data = nil)
			if( data == nil )
				super(84)
			else
				super(84, data)
			end
		end
	end

	class NoDlvyInst < Quickfix::IntField
		def NoDlvyInst.field
			return 85
		end
		def initialize(data = nil)
			if( data == nil )
				super(85)
			else
				super(85, data)
			end
		end
	end

	class DlvyInst < Quickfix::StringField
		def DlvyInst.field
			return 86
		end
		def initialize(data = nil)
			if( data == nil )
				super(86)
			else
				super(86, data)
			end
		end
	end

	class AllocStatus < Quickfix::IntField
		def AllocStatus.field
			return 87
		end
		def initialize(data = nil)
			if( data == nil )
				super(87)
			else
				super(87, data)
			end
		end
	end

	class AllocRejCode < Quickfix::IntField
		def AllocRejCode.field
			return 88
		end
		def initialize(data = nil)
			if( data == nil )
				super(88)
			else
				super(88, data)
			end
		end
	end

	class BrokerOfCredit < Quickfix::StringField
		def BrokerOfCredit.field
			return 92
		end
		def initialize(data = nil)
			if( data == nil )
				super(92)
			else
				super(92, data)
			end
		end
	end

	class EmailType < Quickfix::CharField
		def EmailType.field
			return 94
		end
		def initialize(data = nil)
			if( data == nil )
				super(94)
			else
				super(94, data)
			end
		end
	end

	class StopPx < Quickfix::DoubleField
		def StopPx.field
			return 99
		end
		def initialize(data = nil)
			if( data == nil )
				super(99)
			else
				super(99, data)
			end
		end
	end

	class ExDestination < Quickfix::StringField
		def ExDestination.field
			return 100
		end
		def initialize(data = nil)
			if( data == nil )
				super(100)
			else
				super(100, data)
			end
		end
	end

	class CxlRejReason < Quickfix::IntField
		def CxlRejReason.field
			return 102
		end
		def initialize(data = nil)
			if( data == nil )
				super(102)
			else
				super(102, data)
			end
		end
	end

	class OrdRejReason < Quickfix::IntField
		def OrdRejReason.field
			return 103
		end
		def initialize(data = nil)
			if( data == nil )
				super(103)
			else
				super(103, data)
			end
		end
	end

	class IOIQualifier < Quickfix::CharField
		def IOIQualifier.field
			return 104
		end
		def initialize(data = nil)
			if( data == nil )
				super(104)
			else
				super(104, data)
			end
		end
	end

	class WaveNo < Quickfix::StringField
		def WaveNo.field
			return 105
		end
		def initialize(data = nil)
			if( data == nil )
				super(105)
			else
				super(105, data)
			end
		end
	end

	class Issuer < Quickfix::StringField
		def Issuer.field
			return 106
		end
		def initialize(data = nil)
			if( data == nil )
				super(106)
			else
				super(106, data)
			end
		end
	end

	class SecurityDesc < Quickfix::StringField
		def SecurityDesc.field
			return 107
		end
		def initialize(data = nil)
			if( data == nil )
				super(107)
			else
				super(107, data)
			end
		end
	end

	class ClientID < Quickfix::StringField
		def ClientID.field
			return 109
		end
		def initialize(data = nil)
			if( data == nil )
				super(109)
			else
				super(109, data)
			end
		end
	end

	class MinQty < Quickfix::DoubleField
		def MinQty.field
			return 110
		end
		def initialize(data = nil)
			if( data == nil )
				super(110)
			else
				super(110, data)
			end
		end
	end

	class MaxFloor < Quickfix::DoubleField
		def MaxFloor.field
			return 111
		end
		def initialize(data = nil)
			if( data == nil )
				super(111)
			else
				super(111, data)
			end
		end
	end

	class ReportToExch < Quickfix::BoolField
		def ReportToExch.field
			return 113
		end
		def initialize(data = nil)
			if( data == nil )
				super(113)
			else
				super(113, data)
			end
		end
	end

	class LocateReqd < Quickfix::BoolField
		def LocateReqd.field
			return 114
		end
		def initialize(data = nil)
			if( data == nil )
				super(114)
			else
				super(114, data)
			end
		end
	end

	class QuoteID < Quickfix::StringField
		def QuoteID.field
			return 117
		end
		def initialize(data = nil)
			if( data == nil )
				super(117)
			else
				super(117, data)
			end
		end
	end

	class NetMoney < Quickfix::DoubleField
		def NetMoney.field
			return 118
		end
		def initialize(data = nil)
			if( data == nil )
				super(118)
			else
				super(118, data)
			end
		end
	end

	class SettlCurrAmt < Quickfix::DoubleField
		def SettlCurrAmt.field
			return 119
		end
		def initialize(data = nil)
			if( data == nil )
				super(119)
			else
				super(119, data)
			end
		end
	end

	class SettlCurrency < Quickfix::StringField
		def SettlCurrency.field
			return 120
		end
		def initialize(data = nil)
			if( data == nil )
				super(120)
			else
				super(120, data)
			end
		end
	end

	class ForexReq < Quickfix::BoolField
		def ForexReq.field
			return 121
		end
		def initialize(data = nil)
			if( data == nil )
				super(121)
			else
				super(121, data)
			end
		end
	end

	class NoExecs < Quickfix::IntField
		def NoExecs.field
			return 124
		end
		def initialize(data = nil)
			if( data == nil )
				super(124)
			else
				super(124, data)
			end
		end
	end

	class CxlType < Quickfix::CharField
		def CxlType.field
			return 125
		end
		def initialize(data = nil)
			if( data == nil )
				super(125)
			else
				super(125, data)
			end
		end
	end

	class ExpireTime < Quickfix::UtcTimeStampField
		def ExpireTime.field
			return 126
		end
		def initialize(data = nil)
			if( data == nil )
				super(126)
			else
				super(126, data)
			end
		end
	end

	class DKReason < Quickfix::CharField
		def DKReason.field
			return 127
		end
		def initialize(data = nil)
			if( data == nil )
				super(127)
			else
				super(127, data)
			end
		end
	end

	class IOINaturalFlag < Quickfix::BoolField
		def IOINaturalFlag.field
			return 130
		end
		def initialize(data = nil)
			if( data == nil )
				super(130)
			else
				super(130, data)
			end
		end
	end

	class QuoteReqID < Quickfix::StringField
		def QuoteReqID.field
			return 131
		end
		def initialize(data = nil)
			if( data == nil )
				super(131)
			else
				super(131, data)
			end
		end
	end

	class BidPx < Quickfix::DoubleField
		def BidPx.field
			return 132
		end
		def initialize(data = nil)
			if( data == nil )
				super(132)
			else
				super(132, data)
			end
		end
	end

	class OfferPx < Quickfix::DoubleField
		def OfferPx.field
			return 133
		end
		def initialize(data = nil)
			if( data == nil )
				super(133)
			else
				super(133, data)
			end
		end
	end

	class BidSize < Quickfix::DoubleField
		def BidSize.field
			return 134
		end
		def initialize(data = nil)
			if( data == nil )
				super(134)
			else
				super(134, data)
			end
		end
	end

	class OfferSize < Quickfix::DoubleField
		def OfferSize.field
			return 135
		end
		def initialize(data = nil)
			if( data == nil )
				super(135)
			else
				super(135, data)
			end
		end
	end

	class NoMiscFees < Quickfix::IntField
		def NoMiscFees.field
			return 136
		end
		def initialize(data = nil)
			if( data == nil )
				super(136)
			else
				super(136, data)
			end
		end
	end

	class MiscFeeAmt < Quickfix::DoubleField
		def MiscFeeAmt.field
			return 137
		end
		def initialize(data = nil)
			if( data == nil )
				super(137)
			else
				super(137, data)
			end
		end
	end

	class MiscFeeCurr < Quickfix::StringField
		def MiscFeeCurr.field
			return 138
		end
		def initialize(data = nil)
			if( data == nil )
				super(138)
			else
				super(138, data)
			end
		end
	end

	class MiscFeeType < Quickfix::StringField
		def MiscFeeType.field
			return 139
		end
		def initialize(data = nil)
			if( data == nil )
				super(139)
			else
				super(139, data)
			end
		end
	end

	class PrevClosePx < Quickfix::DoubleField
		def PrevClosePx.field
			return 140
		end
		def initialize(data = nil)
			if( data == nil )
				super(140)
			else
				super(140, data)
			end
		end
	end

	class NoRelatedSym < Quickfix::IntField
		def NoRelatedSym.field
			return 146
		end
		def initialize(data = nil)
			if( data == nil )
				super(146)
			else
				super(146, data)
			end
		end
	end

	class Subject < Quickfix::StringField
		def Subject.field
			return 147
		end
		def initialize(data = nil)
			if( data == nil )
				super(147)
			else
				super(147, data)
			end
		end
	end

	class Headline < Quickfix::StringField
		def Headline.field
			return 148
		end
		def initialize(data = nil)
			if( data == nil )
				super(148)
			else
				super(148, data)
			end
		end
	end

	class URLLink < Quickfix::StringField
		def URLLink.field
			return 149
		end
		def initialize(data = nil)
			if( data == nil )
				super(149)
			else
				super(149, data)
			end
		end
	end

	class ExecType < Quickfix::CharField
		def ExecType.field
			return 150
		end
		def initialize(data = nil)
			if( data == nil )
				super(150)
			else
				super(150, data)
			end
		end
	end

	class LeavesQty < Quickfix::DoubleField
		def LeavesQty.field
			return 151
		end
		def initialize(data = nil)
			if( data == nil )
				super(151)
			else
				super(151, data)
			end
		end
	end

	class CashOrderQty < Quickfix::DoubleField
		def CashOrderQty.field
			return 152
		end
		def initialize(data = nil)
			if( data == nil )
				super(152)
			else
				super(152, data)
			end
		end
	end

	class AllocAvgPx < Quickfix::DoubleField
		def AllocAvgPx.field
			return 153
		end
		def initialize(data = nil)
			if( data == nil )
				super(153)
			else
				super(153, data)
			end
		end
	end

	class AllocNetMoney < Quickfix::DoubleField
		def AllocNetMoney.field
			return 154
		end
		def initialize(data = nil)
			if( data == nil )
				super(154)
			else
				super(154, data)
			end
		end
	end

	class SettlCurrFxRate < Quickfix::DoubleField
		def SettlCurrFxRate.field
			return 155
		end
		def initialize(data = nil)
			if( data == nil )
				super(155)
			else
				super(155, data)
			end
		end
	end

	class SettlCurrFxRateCalc < Quickfix::CharField
		def SettlCurrFxRateCalc.field
			return 156
		end
		def initialize(data = nil)
			if( data == nil )
				super(156)
			else
				super(156, data)
			end
		end
	end

	class NumDaysInterest < Quickfix::IntField
		def NumDaysInterest.field
			return 157
		end
		def initialize(data = nil)
			if( data == nil )
				super(157)
			else
				super(157, data)
			end
		end
	end

	class AccruedInterestRate < Quickfix::DoubleField
		def AccruedInterestRate.field
			return 158
		end
		def initialize(data = nil)
			if( data == nil )
				super(158)
			else
				super(158, data)
			end
		end
	end

	class AccruedInterestAmt < Quickfix::DoubleField
		def AccruedInterestAmt.field
			return 159
		end
		def initialize(data = nil)
			if( data == nil )
				super(159)
			else
				super(159, data)
			end
		end
	end

	class SettlInstMode < Quickfix::CharField
		def SettlInstMode.field
			return 160
		end
		def initialize(data = nil)
			if( data == nil )
				super(160)
			else
				super(160, data)
			end
		end
	end

	class AllocText < Quickfix::StringField
		def AllocText.field
			return 161
		end
		def initialize(data = nil)
			if( data == nil )
				super(161)
			else
				super(161, data)
			end
		end
	end

	class SettlInstID < Quickfix::StringField
		def SettlInstID.field
			return 162
		end
		def initialize(data = nil)
			if( data == nil )
				super(162)
			else
				super(162, data)
			end
		end
	end

	class SettlInstTransType < Quickfix::CharField
		def SettlInstTransType.field
			return 163
		end
		def initialize(data = nil)
			if( data == nil )
				super(163)
			else
				super(163, data)
			end
		end
	end

	class EmailThreadID < Quickfix::StringField
		def EmailThreadID.field
			return 164
		end
		def initialize(data = nil)
			if( data == nil )
				super(164)
			else
				super(164, data)
			end
		end
	end

	class SettlInstSource < Quickfix::CharField
		def SettlInstSource.field
			return 165
		end
		def initialize(data = nil)
			if( data == nil )
				super(165)
			else
				super(165, data)
			end
		end
	end

	class SettlLocation < Quickfix::StringField
		def SettlLocation.field
			return 166
		end
		def initialize(data = nil)
			if( data == nil )
				super(166)
			else
				super(166, data)
			end
		end
	end

	class SecurityType < Quickfix::StringField
		def SecurityType.field
			return 167
		end
		def initialize(data = nil)
			if( data == nil )
				super(167)
			else
				super(167, data)
			end
		end
	end

	class EffectiveTime < Quickfix::UtcTimeStampField
		def EffectiveTime.field
			return 168
		end
		def initialize(data = nil)
			if( data == nil )
				super(168)
			else
				super(168, data)
			end
		end
	end

	class StandInstDbType < Quickfix::IntField
		def StandInstDbType.field
			return 169
		end
		def initialize(data = nil)
			if( data == nil )
				super(169)
			else
				super(169, data)
			end
		end
	end

	class StandInstDbName < Quickfix::StringField
		def StandInstDbName.field
			return 170
		end
		def initialize(data = nil)
			if( data == nil )
				super(170)
			else
				super(170, data)
			end
		end
	end

	class StandInstDbID < Quickfix::StringField
		def StandInstDbID.field
			return 171
		end
		def initialize(data = nil)
			if( data == nil )
				super(171)
			else
				super(171, data)
			end
		end
	end

	class SettlDeliveryType < Quickfix::IntField
		def SettlDeliveryType.field
			return 172
		end
		def initialize(data = nil)
			if( data == nil )
				super(172)
			else
				super(172, data)
			end
		end
	end

	class SettlDepositoryCode < Quickfix::StringField
		def SettlDepositoryCode.field
			return 173
		end
		def initialize(data = nil)
			if( data == nil )
				super(173)
			else
				super(173, data)
			end
		end
	end

	class SettlBrkrCode < Quickfix::StringField
		def SettlBrkrCode.field
			return 174
		end
		def initialize(data = nil)
			if( data == nil )
				super(174)
			else
				super(174, data)
			end
		end
	end

	class SettlInstCode < Quickfix::StringField
		def SettlInstCode.field
			return 175
		end
		def initialize(data = nil)
			if( data == nil )
				super(175)
			else
				super(175, data)
			end
		end
	end

	class SecuritySettlAgentName < Quickfix::StringField
		def SecuritySettlAgentName.field
			return 176
		end
		def initialize(data = nil)
			if( data == nil )
				super(176)
			else
				super(176, data)
			end
		end
	end

	class SecuritySettlAgentCode < Quickfix::StringField
		def SecuritySettlAgentCode.field
			return 177
		end
		def initialize(data = nil)
			if( data == nil )
				super(177)
			else
				super(177, data)
			end
		end
	end

	class SecuritySettlAgentAcctNum < Quickfix::StringField
		def SecuritySettlAgentAcctNum.field
			return 178
		end
		def initialize(data = nil)
			if( data == nil )
				super(178)
			else
				super(178, data)
			end
		end
	end

	class SecuritySettlAgentAcctName < Quickfix::StringField
		def SecuritySettlAgentAcctName.field
			return 179
		end
		def initialize(data = nil)
			if( data == nil )
				super(179)
			else
				super(179, data)
			end
		end
	end

	class SecuritySettlAgentContactName < Quickfix::StringField
		def SecuritySettlAgentContactName.field
			return 180
		end
		def initialize(data = nil)
			if( data == nil )
				super(180)
			else
				super(180, data)
			end
		end
	end

	class SecuritySettlAgentContactPhone < Quickfix::StringField
		def SecuritySettlAgentContactPhone.field
			return 181
		end
		def initialize(data = nil)
			if( data == nil )
				super(181)
			else
				super(181, data)
			end
		end
	end

	class CashSettlAgentName < Quickfix::StringField
		def CashSettlAgentName.field
			return 182
		end
		def initialize(data = nil)
			if( data == nil )
				super(182)
			else
				super(182, data)
			end
		end
	end

	class CashSettlAgentCode < Quickfix::StringField
		def CashSettlAgentCode.field
			return 183
		end
		def initialize(data = nil)
			if( data == nil )
				super(183)
			else
				super(183, data)
			end
		end
	end

	class CashSettlAgentAcctNum < Quickfix::StringField
		def CashSettlAgentAcctNum.field
			return 184
		end
		def initialize(data = nil)
			if( data == nil )
				super(184)
			else
				super(184, data)
			end
		end
	end

	class CashSettlAgentAcctName < Quickfix::StringField
		def CashSettlAgentAcctName.field
			return 185
		end
		def initialize(data = nil)
			if( data == nil )
				super(185)
			else
				super(185, data)
			end
		end
	end

	class CashSettlAgentContactName < Quickfix::StringField
		def CashSettlAgentContactName.field
			return 186
		end
		def initialize(data = nil)
			if( data == nil )
				super(186)
			else
				super(186, data)
			end
		end
	end

	class CashSettlAgentContactPhone < Quickfix::StringField
		def CashSettlAgentContactPhone.field
			return 187
		end
		def initialize(data = nil)
			if( data == nil )
				super(187)
			else
				super(187, data)
			end
		end
	end

	class BidSpotRate < Quickfix::DoubleField
		def BidSpotRate.field
			return 188
		end
		def initialize(data = nil)
			if( data == nil )
				super(188)
			else
				super(188, data)
			end
		end
	end

	class BidForwardPoints < Quickfix::DoubleField
		def BidForwardPoints.field
			return 189
		end
		def initialize(data = nil)
			if( data == nil )
				super(189)
			else
				super(189, data)
			end
		end
	end

	class OfferSpotRate < Quickfix::DoubleField
		def OfferSpotRate.field
			return 190
		end
		def initialize(data = nil)
			if( data == nil )
				super(190)
			else
				super(190, data)
			end
		end
	end

	class OfferForwardPoints < Quickfix::DoubleField
		def OfferForwardPoints.field
			return 191
		end
		def initialize(data = nil)
			if( data == nil )
				super(191)
			else
				super(191, data)
			end
		end
	end

	class OrderQty2 < Quickfix::DoubleField
		def OrderQty2.field
			return 192
		end
		def initialize(data = nil)
			if( data == nil )
				super(192)
			else
				super(192, data)
			end
		end
	end

	class FutSettDate2 < Quickfix::StringField
		def FutSettDate2.field
			return 193
		end
		def initialize(data = nil)
			if( data == nil )
				super(193)
			else
				super(193, data)
			end
		end
	end

	class LastSpotRate < Quickfix::DoubleField
		def LastSpotRate.field
			return 194
		end
		def initialize(data = nil)
			if( data == nil )
				super(194)
			else
				super(194, data)
			end
		end
	end

	class LastForwardPoints < Quickfix::DoubleField
		def LastForwardPoints.field
			return 195
		end
		def initialize(data = nil)
			if( data == nil )
				super(195)
			else
				super(195, data)
			end
		end
	end

	class AllocLinkID < Quickfix::StringField
		def AllocLinkID.field
			return 196
		end
		def initialize(data = nil)
			if( data == nil )
				super(196)
			else
				super(196, data)
			end
		end
	end

	class AllocLinkType < Quickfix::IntField
		def AllocLinkType.field
			return 197
		end
		def initialize(data = nil)
			if( data == nil )
				super(197)
			else
				super(197, data)
			end
		end
	end

	class SecondaryOrderID < Quickfix::StringField
		def SecondaryOrderID.field
			return 198
		end
		def initialize(data = nil)
			if( data == nil )
				super(198)
			else
				super(198, data)
			end
		end
	end

	class NoIOIQualifiers < Quickfix::IntField
		def NoIOIQualifiers.field
			return 199
		end
		def initialize(data = nil)
			if( data == nil )
				super(199)
			else
				super(199, data)
			end
		end
	end

	class MaturityMonthYear < Quickfix::StringField
		def MaturityMonthYear.field
			return 200
		end
		def initialize(data = nil)
			if( data == nil )
				super(200)
			else
				super(200, data)
			end
		end
	end

	class PutOrCall < Quickfix::IntField
		def PutOrCall.field
			return 201
		end
		def initialize(data = nil)
			if( data == nil )
				super(201)
			else
				super(201, data)
			end
		end
	end

	class StrikePrice < Quickfix::DoubleField
		def StrikePrice.field
			return 202
		end
		def initialize(data = nil)
			if( data == nil )
				super(202)
			else
				super(202, data)
			end
		end
	end

	class CoveredOrUncovered < Quickfix::IntField
		def CoveredOrUncovered.field
			return 203
		end
		def initialize(data = nil)
			if( data == nil )
				super(203)
			else
				super(203, data)
			end
		end
	end

	class CustomerOrFirm < Quickfix::IntField
		def CustomerOrFirm.field
			return 204
		end
		def initialize(data = nil)
			if( data == nil )
				super(204)
			else
				super(204, data)
			end
		end
	end

	class MaturityDay < Quickfix::StringField
		def MaturityDay.field
			return 205
		end
		def initialize(data = nil)
			if( data == nil )
				super(205)
			else
				super(205, data)
			end
		end
	end

	class OptAttribute < Quickfix::CharField
		def OptAttribute.field
			return 206
		end
		def initialize(data = nil)
			if( data == nil )
				super(206)
			else
				super(206, data)
			end
		end
	end

	class SecurityExchange < Quickfix::StringField
		def SecurityExchange.field
			return 207
		end
		def initialize(data = nil)
			if( data == nil )
				super(207)
			else
				super(207, data)
			end
		end
	end

	class NotifyBrokerOfCredit < Quickfix::BoolField
		def NotifyBrokerOfCredit.field
			return 208
		end
		def initialize(data = nil)
			if( data == nil )
				super(208)
			else
				super(208, data)
			end
		end
	end

	class AllocHandlInst < Quickfix::IntField
		def AllocHandlInst.field
			return 209
		end
		def initialize(data = nil)
			if( data == nil )
				super(209)
			else
				super(209, data)
			end
		end
	end

	class MaxShow < Quickfix::DoubleField
		def MaxShow.field
			return 210
		end
		def initialize(data = nil)
			if( data == nil )
				super(210)
			else
				super(210, data)
			end
		end
	end

	class PegDifference < Quickfix::DoubleField
		def PegDifference.field
			return 211
		end
		def initialize(data = nil)
			if( data == nil )
				super(211)
			else
				super(211, data)
			end
		end
	end

	class SendingDate < Quickfix::StringField
		def SendingDate.field
			return 51
		end
		def initialize(data = nil)
			if( data == nil )
				super(51)
			else
				super(51, data)
			end
		end
	end

	class TotNoOrders < Quickfix::IntField
		def TotNoOrders.field
			return 68
		end
		def initialize(data = nil)
			if( data == nil )
				super(68)
			else
				super(68, data)
			end
		end
	end

	class SettlInstRefID < Quickfix::StringField
		def SettlInstRefID.field
			return 214
		end
		def initialize(data = nil)
			if( data == nil )
				super(214)
			else
				super(214, data)
			end
		end
	end

	class NoRoutingIDs < Quickfix::IntField
		def NoRoutingIDs.field
			return 215
		end
		def initialize(data = nil)
			if( data == nil )
				super(215)
			else
				super(215, data)
			end
		end
	end

	class RoutingType < Quickfix::IntField
		def RoutingType.field
			return 216
		end
		def initialize(data = nil)
			if( data == nil )
				super(216)
			else
				super(216, data)
			end
		end
	end

	class RoutingID < Quickfix::StringField
		def RoutingID.field
			return 217
		end
		def initialize(data = nil)
			if( data == nil )
				super(217)
			else
				super(217, data)
			end
		end
	end

	class SpreadToBenchmark < Quickfix::DoubleField
		def SpreadToBenchmark.field
			return 218
		end
		def initialize(data = nil)
			if( data == nil )
				super(218)
			else
				super(218, data)
			end
		end
	end

	class Benchmark < Quickfix::CharField
		def Benchmark.field
			return 219
		end
		def initialize(data = nil)
			if( data == nil )
				super(219)
			else
				super(219, data)
			end
		end
	end

	class CouponRate < Quickfix::DoubleField
		def CouponRate.field
			return 223
		end
		def initialize(data = nil)
			if( data == nil )
				super(223)
			else
				super(223, data)
			end
		end
	end

	class ContractMultiplier < Quickfix::DoubleField
		def ContractMultiplier.field
			return 231
		end
		def initialize(data = nil)
			if( data == nil )
				super(231)
			else
				super(231, data)
			end
		end
	end

	class MDReqID < Quickfix::StringField
		def MDReqID.field
			return 262
		end
		def initialize(data = nil)
			if( data == nil )
				super(262)
			else
				super(262, data)
			end
		end
	end

	class SubscriptionRequestType < Quickfix::CharField
		def SubscriptionRequestType.field
			return 263
		end
		def initialize(data = nil)
			if( data == nil )
				super(263)
			else
				super(263, data)
			end
		end
	end

	class MarketDepth < Quickfix::IntField
		def MarketDepth.field
			return 264
		end
		def initialize(data = nil)
			if( data == nil )
				super(264)
			else
				super(264, data)
			end
		end
	end

	class MDUpdateType < Quickfix::IntField
		def MDUpdateType.field
			return 265
		end
		def initialize(data = nil)
			if( data == nil )
				super(265)
			else
				super(265, data)
			end
		end
	end

	class AggregatedBook < Quickfix::BoolField
		def AggregatedBook.field
			return 266
		end
		def initialize(data = nil)
			if( data == nil )
				super(266)
			else
				super(266, data)
			end
		end
	end

	class NoMDEntryTypes < Quickfix::IntField
		def NoMDEntryTypes.field
			return 267
		end
		def initialize(data = nil)
			if( data == nil )
				super(267)
			else
				super(267, data)
			end
		end
	end

	class NoMDEntries < Quickfix::IntField
		def NoMDEntries.field
			return 268
		end
		def initialize(data = nil)
			if( data == nil )
				super(268)
			else
				super(268, data)
			end
		end
	end

	class MDEntryType < Quickfix::CharField
		def MDEntryType.field
			return 269
		end
		def initialize(data = nil)
			if( data == nil )
				super(269)
			else
				super(269, data)
			end
		end
	end

	class MDEntryPx < Quickfix::DoubleField
		def MDEntryPx.field
			return 270
		end
		def initialize(data = nil)
			if( data == nil )
				super(270)
			else
				super(270, data)
			end
		end
	end

	class MDEntrySize < Quickfix::DoubleField
		def MDEntrySize.field
			return 271
		end
		def initialize(data = nil)
			if( data == nil )
				super(271)
			else
				super(271, data)
			end
		end
	end

	class MDEntryDate < Quickfix::UtcDateField
		def MDEntryDate.field
			return 272
		end
		def initialize(data = nil)
			if( data == nil )
				super(272)
			else
				super(272, data)
			end
		end
	end

	class MDEntryTime < Quickfix::UtcTimeOnlyField
		def MDEntryTime.field
			return 273
		end
		def initialize(data = nil)
			if( data == nil )
				super(273)
			else
				super(273, data)
			end
		end
	end

	class TickDirection < Quickfix::CharField
		def TickDirection.field
			return 274
		end
		def initialize(data = nil)
			if( data == nil )
				super(274)
			else
				super(274, data)
			end
		end
	end

	class MDMkt < Quickfix::StringField
		def MDMkt.field
			return 275
		end
		def initialize(data = nil)
			if( data == nil )
				super(275)
			else
				super(275, data)
			end
		end
	end

	class QuoteCondition < Quickfix::StringField
		def QuoteCondition.field
			return 276
		end
		def initialize(data = nil)
			if( data == nil )
				super(276)
			else
				super(276, data)
			end
		end
	end

	class TradeCondition < Quickfix::StringField
		def TradeCondition.field
			return 277
		end
		def initialize(data = nil)
			if( data == nil )
				super(277)
			else
				super(277, data)
			end
		end
	end

	class MDEntryID < Quickfix::StringField
		def MDEntryID.field
			return 278
		end
		def initialize(data = nil)
			if( data == nil )
				super(278)
			else
				super(278, data)
			end
		end
	end

	class MDUpdateAction < Quickfix::CharField
		def MDUpdateAction.field
			return 279
		end
		def initialize(data = nil)
			if( data == nil )
				super(279)
			else
				super(279, data)
			end
		end
	end

	class MDEntryRefID < Quickfix::StringField
		def MDEntryRefID.field
			return 280
		end
		def initialize(data = nil)
			if( data == nil )
				super(280)
			else
				super(280, data)
			end
		end
	end

	class MDReqRejReason < Quickfix::CharField
		def MDReqRejReason.field
			return 281
		end
		def initialize(data = nil)
			if( data == nil )
				super(281)
			else
				super(281, data)
			end
		end
	end

	class MDEntryOriginator < Quickfix::StringField
		def MDEntryOriginator.field
			return 282
		end
		def initialize(data = nil)
			if( data == nil )
				super(282)
			else
				super(282, data)
			end
		end
	end

	class LocationID < Quickfix::StringField
		def LocationID.field
			return 283
		end
		def initialize(data = nil)
			if( data == nil )
				super(283)
			else
				super(283, data)
			end
		end
	end

	class DeskID < Quickfix::StringField
		def DeskID.field
			return 284
		end
		def initialize(data = nil)
			if( data == nil )
				super(284)
			else
				super(284, data)
			end
		end
	end

	class DeleteReason < Quickfix::CharField
		def DeleteReason.field
			return 285
		end
		def initialize(data = nil)
			if( data == nil )
				super(285)
			else
				super(285, data)
			end
		end
	end

	class OpenCloseSettleFlag < Quickfix::StringField
		def OpenCloseSettleFlag.field
			return 286
		end
		def initialize(data = nil)
			if( data == nil )
				super(286)
			else
				super(286, data)
			end
		end
	end

	class SellerDays < Quickfix::IntField
		def SellerDays.field
			return 287
		end
		def initialize(data = nil)
			if( data == nil )
				super(287)
			else
				super(287, data)
			end
		end
	end

	class MDEntryBuyer < Quickfix::StringField
		def MDEntryBuyer.field
			return 288
		end
		def initialize(data = nil)
			if( data == nil )
				super(288)
			else
				super(288, data)
			end
		end
	end

	class MDEntrySeller < Quickfix::StringField
		def MDEntrySeller.field
			return 289
		end
		def initialize(data = nil)
			if( data == nil )
				super(289)
			else
				super(289, data)
			end
		end
	end

	class MDEntryPositionNo < Quickfix::IntField
		def MDEntryPositionNo.field
			return 290
		end
		def initialize(data = nil)
			if( data == nil )
				super(290)
			else
				super(290, data)
			end
		end
	end

	class FinancialStatus < Quickfix::StringField
		def FinancialStatus.field
			return 291
		end
		def initialize(data = nil)
			if( data == nil )
				super(291)
			else
				super(291, data)
			end
		end
	end

	class CorporateAction < Quickfix::StringField
		def CorporateAction.field
			return 292
		end
		def initialize(data = nil)
			if( data == nil )
				super(292)
			else
				super(292, data)
			end
		end
	end

	class DefBidSize < Quickfix::DoubleField
		def DefBidSize.field
			return 293
		end
		def initialize(data = nil)
			if( data == nil )
				super(293)
			else
				super(293, data)
			end
		end
	end

	class DefOfferSize < Quickfix::DoubleField
		def DefOfferSize.field
			return 294
		end
		def initialize(data = nil)
			if( data == nil )
				super(294)
			else
				super(294, data)
			end
		end
	end

	class NoQuoteEntries < Quickfix::IntField
		def NoQuoteEntries.field
			return 295
		end
		def initialize(data = nil)
			if( data == nil )
				super(295)
			else
				super(295, data)
			end
		end
	end

	class NoQuoteSets < Quickfix::IntField
		def NoQuoteSets.field
			return 296
		end
		def initialize(data = nil)
			if( data == nil )
				super(296)
			else
				super(296, data)
			end
		end
	end

	class QuoteAckStatus < Quickfix::IntField
		def QuoteAckStatus.field
			return 1865
		end
		def initialize(data = nil)
			if( data == nil )
				super(1865)
			else
				super(1865, data)
			end
		end
	end

	class QuoteCancelType < Quickfix::IntField
		def QuoteCancelType.field
			return 298
		end
		def initialize(data = nil)
			if( data == nil )
				super(298)
			else
				super(298, data)
			end
		end
	end

	class QuoteEntryID < Quickfix::StringField
		def QuoteEntryID.field
			return 299
		end
		def initialize(data = nil)
			if( data == nil )
				super(299)
			else
				super(299, data)
			end
		end
	end

	class QuoteRejectReason < Quickfix::IntField
		def QuoteRejectReason.field
			return 300
		end
		def initialize(data = nil)
			if( data == nil )
				super(300)
			else
				super(300, data)
			end
		end
	end

	class QuoteResponseLevel < Quickfix::IntField
		def QuoteResponseLevel.field
			return 301
		end
		def initialize(data = nil)
			if( data == nil )
				super(301)
			else
				super(301, data)
			end
		end
	end

	class QuoteSetID < Quickfix::StringField
		def QuoteSetID.field
			return 302
		end
		def initialize(data = nil)
			if( data == nil )
				super(302)
			else
				super(302, data)
			end
		end
	end

	class QuoteRequestType < Quickfix::IntField
		def QuoteRequestType.field
			return 303
		end
		def initialize(data = nil)
			if( data == nil )
				super(303)
			else
				super(303, data)
			end
		end
	end

	class TotQuoteEntries < Quickfix::IntField
		def TotQuoteEntries.field
			return 304
		end
		def initialize(data = nil)
			if( data == nil )
				super(304)
			else
				super(304, data)
			end
		end
	end

	class UnderlyingIDSource < Quickfix::StringField
		def UnderlyingIDSource.field
			return 305
		end
		def initialize(data = nil)
			if( data == nil )
				super(305)
			else
				super(305, data)
			end
		end
	end

	class UnderlyingIssuer < Quickfix::StringField
		def UnderlyingIssuer.field
			return 306
		end
		def initialize(data = nil)
			if( data == nil )
				super(306)
			else
				super(306, data)
			end
		end
	end

	class UnderlyingSecurityDesc < Quickfix::StringField
		def UnderlyingSecurityDesc.field
			return 307
		end
		def initialize(data = nil)
			if( data == nil )
				super(307)
			else
				super(307, data)
			end
		end
	end

	class UnderlyingSecurityExchange < Quickfix::StringField
		def UnderlyingSecurityExchange.field
			return 308
		end
		def initialize(data = nil)
			if( data == nil )
				super(308)
			else
				super(308, data)
			end
		end
	end

	class UnderlyingSecurityID < Quickfix::StringField
		def UnderlyingSecurityID.field
			return 309
		end
		def initialize(data = nil)
			if( data == nil )
				super(309)
			else
				super(309, data)
			end
		end
	end

	class UnderlyingSecurityType < Quickfix::StringField
		def UnderlyingSecurityType.field
			return 310
		end
		def initialize(data = nil)
			if( data == nil )
				super(310)
			else
				super(310, data)
			end
		end
	end

	class UnderlyingSymbol < Quickfix::StringField
		def UnderlyingSymbol.field
			return 311
		end
		def initialize(data = nil)
			if( data == nil )
				super(311)
			else
				super(311, data)
			end
		end
	end

	class UnderlyingSymbolSfx < Quickfix::StringField
		def UnderlyingSymbolSfx.field
			return 312
		end
		def initialize(data = nil)
			if( data == nil )
				super(312)
			else
				super(312, data)
			end
		end
	end

	class UnderlyingMaturityMonthYear < Quickfix::StringField
		def UnderlyingMaturityMonthYear.field
			return 313
		end
		def initialize(data = nil)
			if( data == nil )
				super(313)
			else
				super(313, data)
			end
		end
	end

	class UnderlyingMaturityDay < Quickfix::StringField
		def UnderlyingMaturityDay.field
			return 314
		end
		def initialize(data = nil)
			if( data == nil )
				super(314)
			else
				super(314, data)
			end
		end
	end

	class UnderlyingPutOrCall < Quickfix::IntField
		def UnderlyingPutOrCall.field
			return 315
		end
		def initialize(data = nil)
			if( data == nil )
				super(315)
			else
				super(315, data)
			end
		end
	end

	class UnderlyingStrikePrice < Quickfix::DoubleField
		def UnderlyingStrikePrice.field
			return 316
		end
		def initialize(data = nil)
			if( data == nil )
				super(316)
			else
				super(316, data)
			end
		end
	end

	class UnderlyingOptAttribute < Quickfix::CharField
		def UnderlyingOptAttribute.field
			return 317
		end
		def initialize(data = nil)
			if( data == nil )
				super(317)
			else
				super(317, data)
			end
		end
	end

	class UnderlyingCurrency < Quickfix::StringField
		def UnderlyingCurrency.field
			return 318
		end
		def initialize(data = nil)
			if( data == nil )
				super(318)
			else
				super(318, data)
			end
		end
	end

	class RatioQty < Quickfix::DoubleField
		def RatioQty.field
			return 319
		end
		def initialize(data = nil)
			if( data == nil )
				super(319)
			else
				super(319, data)
			end
		end
	end

	class SecurityReqID < Quickfix::StringField
		def SecurityReqID.field
			return 320
		end
		def initialize(data = nil)
			if( data == nil )
				super(320)
			else
				super(320, data)
			end
		end
	end

	class SecurityRequestType < Quickfix::IntField
		def SecurityRequestType.field
			return 321
		end
		def initialize(data = nil)
			if( data == nil )
				super(321)
			else
				super(321, data)
			end
		end
	end

	class SecurityResponseID < Quickfix::StringField
		def SecurityResponseID.field
			return 322
		end
		def initialize(data = nil)
			if( data == nil )
				super(322)
			else
				super(322, data)
			end
		end
	end

	class SecurityResponseType < Quickfix::IntField
		def SecurityResponseType.field
			return 323
		end
		def initialize(data = nil)
			if( data == nil )
				super(323)
			else
				super(323, data)
			end
		end
	end

	class SecurityStatusReqID < Quickfix::StringField
		def SecurityStatusReqID.field
			return 324
		end
		def initialize(data = nil)
			if( data == nil )
				super(324)
			else
				super(324, data)
			end
		end
	end

	class UnsolicitedIndicator < Quickfix::BoolField
		def UnsolicitedIndicator.field
			return 325
		end
		def initialize(data = nil)
			if( data == nil )
				super(325)
			else
				super(325, data)
			end
		end
	end

	class SecurityTradingStatus < Quickfix::IntField
		def SecurityTradingStatus.field
			return 326
		end
		def initialize(data = nil)
			if( data == nil )
				super(326)
			else
				super(326, data)
			end
		end
	end

	class HaltReasonChar < Quickfix::CharField
		def HaltReasonChar.field
			return 327
		end
		def initialize(data = nil)
			if( data == nil )
				super(327)
			else
				super(327, data)
			end
		end
	end

	class InViewOfCommon < Quickfix::BoolField
		def InViewOfCommon.field
			return 328
		end
		def initialize(data = nil)
			if( data == nil )
				super(328)
			else
				super(328, data)
			end
		end
	end

	class DueToRelated < Quickfix::BoolField
		def DueToRelated.field
			return 329
		end
		def initialize(data = nil)
			if( data == nil )
				super(329)
			else
				super(329, data)
			end
		end
	end

	class BuyVolume < Quickfix::DoubleField
		def BuyVolume.field
			return 330
		end
		def initialize(data = nil)
			if( data == nil )
				super(330)
			else
				super(330, data)
			end
		end
	end

	class SellVolume < Quickfix::DoubleField
		def SellVolume.field
			return 331
		end
		def initialize(data = nil)
			if( data == nil )
				super(331)
			else
				super(331, data)
			end
		end
	end

	class HighPx < Quickfix::DoubleField
		def HighPx.field
			return 332
		end
		def initialize(data = nil)
			if( data == nil )
				super(332)
			else
				super(332, data)
			end
		end
	end

	class LowPx < Quickfix::DoubleField
		def LowPx.field
			return 333
		end
		def initialize(data = nil)
			if( data == nil )
				super(333)
			else
				super(333, data)
			end
		end
	end

	class Adjustment < Quickfix::IntField
		def Adjustment.field
			return 334
		end
		def initialize(data = nil)
			if( data == nil )
				super(334)
			else
				super(334, data)
			end
		end
	end

	class TradSesReqID < Quickfix::StringField
		def TradSesReqID.field
			return 335
		end
		def initialize(data = nil)
			if( data == nil )
				super(335)
			else
				super(335, data)
			end
		end
	end

	class TradingSessionID < Quickfix::StringField
		def TradingSessionID.field
			return 336
		end
		def initialize(data = nil)
			if( data == nil )
				super(336)
			else
				super(336, data)
			end
		end
	end

	class ContraTrader < Quickfix::StringField
		def ContraTrader.field
			return 337
		end
		def initialize(data = nil)
			if( data == nil )
				super(337)
			else
				super(337, data)
			end
		end
	end

	class TradSesMethod < Quickfix::IntField
		def TradSesMethod.field
			return 338
		end
		def initialize(data = nil)
			if( data == nil )
				super(338)
			else
				super(338, data)
			end
		end
	end

	class TradSesMode < Quickfix::IntField
		def TradSesMode.field
			return 339
		end
		def initialize(data = nil)
			if( data == nil )
				super(339)
			else
				super(339, data)
			end
		end
	end

	class TradSesStatus < Quickfix::IntField
		def TradSesStatus.field
			return 340
		end
		def initialize(data = nil)
			if( data == nil )
				super(340)
			else
				super(340, data)
			end
		end
	end

	class TradSesStartTime < Quickfix::UtcTimeStampField
		def TradSesStartTime.field
			return 341
		end
		def initialize(data = nil)
			if( data == nil )
				super(341)
			else
				super(341, data)
			end
		end
	end

	class TradSesOpenTime < Quickfix::UtcTimeStampField
		def TradSesOpenTime.field
			return 342
		end
		def initialize(data = nil)
			if( data == nil )
				super(342)
			else
				super(342, data)
			end
		end
	end

	class TradSesPreCloseTime < Quickfix::UtcTimeStampField
		def TradSesPreCloseTime.field
			return 343
		end
		def initialize(data = nil)
			if( data == nil )
				super(343)
			else
				super(343, data)
			end
		end
	end

	class TradSesCloseTime < Quickfix::UtcTimeStampField
		def TradSesCloseTime.field
			return 344
		end
		def initialize(data = nil)
			if( data == nil )
				super(344)
			else
				super(344, data)
			end
		end
	end

	class TradSesEndTime < Quickfix::UtcTimeStampField
		def TradSesEndTime.field
			return 345
		end
		def initialize(data = nil)
			if( data == nil )
				super(345)
			else
				super(345, data)
			end
		end
	end

	class NumberOfOrders < Quickfix::IntField
		def NumberOfOrders.field
			return 346
		end
		def initialize(data = nil)
			if( data == nil )
				super(346)
			else
				super(346, data)
			end
		end
	end

	class EncodedIssuerLen < Quickfix::IntField
		def EncodedIssuerLen.field
			return 348
		end
		def initialize(data = nil)
			if( data == nil )
				super(348)
			else
				super(348, data)
			end
		end
	end

	class EncodedIssuer < Quickfix::StringField
		def EncodedIssuer.field
			return 349
		end
		def initialize(data = nil)
			if( data == nil )
				super(349)
			else
				super(349, data)
			end
		end
	end

	class EncodedSecurityDescLen < Quickfix::IntField
		def EncodedSecurityDescLen.field
			return 350
		end
		def initialize(data = nil)
			if( data == nil )
				super(350)
			else
				super(350, data)
			end
		end
	end

	class EncodedSecurityDesc < Quickfix::StringField
		def EncodedSecurityDesc.field
			return 351
		end
		def initialize(data = nil)
			if( data == nil )
				super(351)
			else
				super(351, data)
			end
		end
	end

	class EncodedListExecInstLen < Quickfix::IntField
		def EncodedListExecInstLen.field
			return 352
		end
		def initialize(data = nil)
			if( data == nil )
				super(352)
			else
				super(352, data)
			end
		end
	end

	class EncodedListExecInst < Quickfix::StringField
		def EncodedListExecInst.field
			return 353
		end
		def initialize(data = nil)
			if( data == nil )
				super(353)
			else
				super(353, data)
			end
		end
	end

	class EncodedSubjectLen < Quickfix::IntField
		def EncodedSubjectLen.field
			return 356
		end
		def initialize(data = nil)
			if( data == nil )
				super(356)
			else
				super(356, data)
			end
		end
	end

	class EncodedSubject < Quickfix::StringField
		def EncodedSubject.field
			return 357
		end
		def initialize(data = nil)
			if( data == nil )
				super(357)
			else
				super(357, data)
			end
		end
	end

	class EncodedHeadlineLen < Quickfix::IntField
		def EncodedHeadlineLen.field
			return 358
		end
		def initialize(data = nil)
			if( data == nil )
				super(358)
			else
				super(358, data)
			end
		end
	end

	class EncodedHeadline < Quickfix::StringField
		def EncodedHeadline.field
			return 359
		end
		def initialize(data = nil)
			if( data == nil )
				super(359)
			else
				super(359, data)
			end
		end
	end

	class EncodedAllocTextLen < Quickfix::IntField
		def EncodedAllocTextLen.field
			return 360
		end
		def initialize(data = nil)
			if( data == nil )
				super(360)
			else
				super(360, data)
			end
		end
	end

	class EncodedAllocText < Quickfix::StringField
		def EncodedAllocText.field
			return 361
		end
		def initialize(data = nil)
			if( data == nil )
				super(361)
			else
				super(361, data)
			end
		end
	end

	class EncodedUnderlyingIssuerLen < Quickfix::IntField
		def EncodedUnderlyingIssuerLen.field
			return 362
		end
		def initialize(data = nil)
			if( data == nil )
				super(362)
			else
				super(362, data)
			end
		end
	end

	class EncodedUnderlyingIssuer < Quickfix::StringField
		def EncodedUnderlyingIssuer.field
			return 363
		end
		def initialize(data = nil)
			if( data == nil )
				super(363)
			else
				super(363, data)
			end
		end
	end

	class EncodedUnderlyingSecurityDescLen < Quickfix::IntField
		def EncodedUnderlyingSecurityDescLen.field
			return 364
		end
		def initialize(data = nil)
			if( data == nil )
				super(364)
			else
				super(364, data)
			end
		end
	end

	class EncodedUnderlyingSecurityDesc < Quickfix::StringField
		def EncodedUnderlyingSecurityDesc.field
			return 365
		end
		def initialize(data = nil)
			if( data == nil )
				super(365)
			else
				super(365, data)
			end
		end
	end

	class AllocPrice < Quickfix::DoubleField
		def AllocPrice.field
			return 366
		end
		def initialize(data = nil)
			if( data == nil )
				super(366)
			else
				super(366, data)
			end
		end
	end

	class QuoteSetValidUntilTime < Quickfix::UtcTimeStampField
		def QuoteSetValidUntilTime.field
			return 367
		end
		def initialize(data = nil)
			if( data == nil )
				super(367)
			else
				super(367, data)
			end
		end
	end

	class QuoteEntryRejectReason < Quickfix::IntField
		def QuoteEntryRejectReason.field
			return 368
		end
		def initialize(data = nil)
			if( data == nil )
				super(368)
			else
				super(368, data)
			end
		end
	end

	class OnBehalfOfSendingTime < Quickfix::UtcTimeStampField
		def OnBehalfOfSendingTime.field
			return 370
		end
		def initialize(data = nil)
			if( data == nil )
				super(370)
			else
				super(370, data)
			end
		end
	end

	class BidRequestTransType < Quickfix::CharField
		def BidRequestTransType.field
			return 374
		end
		def initialize(data = nil)
			if( data == nil )
				super(374)
			else
				super(374, data)
			end
		end
	end

	class ContraBroker < Quickfix::StringField
		def ContraBroker.field
			return 375
		end
		def initialize(data = nil)
			if( data == nil )
				super(375)
			else
				super(375, data)
			end
		end
	end

	class ComplianceID < Quickfix::StringField
		def ComplianceID.field
			return 376
		end
		def initialize(data = nil)
			if( data == nil )
				super(376)
			else
				super(376, data)
			end
		end
	end

	class SolicitedFlag < Quickfix::BoolField
		def SolicitedFlag.field
			return 377
		end
		def initialize(data = nil)
			if( data == nil )
				super(377)
			else
				super(377, data)
			end
		end
	end

	class ExecRestatementReason < Quickfix::IntField
		def ExecRestatementReason.field
			return 378
		end
		def initialize(data = nil)
			if( data == nil )
				super(378)
			else
				super(378, data)
			end
		end
	end

	class BusinessRejectRefID < Quickfix::StringField
		def BusinessRejectRefID.field
			return 379
		end
		def initialize(data = nil)
			if( data == nil )
				super(379)
			else
				super(379, data)
			end
		end
	end

	class BusinessRejectReason < Quickfix::IntField
		def BusinessRejectReason.field
			return 380
		end
		def initialize(data = nil)
			if( data == nil )
				super(380)
			else
				super(380, data)
			end
		end
	end

	class GrossTradeAmt < Quickfix::DoubleField
		def GrossTradeAmt.field
			return 381
		end
		def initialize(data = nil)
			if( data == nil )
				super(381)
			else
				super(381, data)
			end
		end
	end

	class NoContraBrokers < Quickfix::IntField
		def NoContraBrokers.field
			return 382
		end
		def initialize(data = nil)
			if( data == nil )
				super(382)
			else
				super(382, data)
			end
		end
	end

	class NoMsgTypes < Quickfix::IntField
		def NoMsgTypes.field
			return 384
		end
		def initialize(data = nil)
			if( data == nil )
				super(384)
			else
				super(384, data)
			end
		end
	end

	class MsgDirection < Quickfix::CharField
		def MsgDirection.field
			return 385
		end
		def initialize(data = nil)
			if( data == nil )
				super(385)
			else
				super(385, data)
			end
		end
	end

	class NoTradingSessions < Quickfix::IntField
		def NoTradingSessions.field
			return 386
		end
		def initialize(data = nil)
			if( data == nil )
				super(386)
			else
				super(386, data)
			end
		end
	end

	class TotalVolumeTraded < Quickfix::DoubleField
		def TotalVolumeTraded.field
			return 387
		end
		def initialize(data = nil)
			if( data == nil )
				super(387)
			else
				super(387, data)
			end
		end
	end

	class DiscretionInst < Quickfix::CharField
		def DiscretionInst.field
			return 388
		end
		def initialize(data = nil)
			if( data == nil )
				super(388)
			else
				super(388, data)
			end
		end
	end

	class DiscretionOffset < Quickfix::DoubleField
		def DiscretionOffset.field
			return 389
		end
		def initialize(data = nil)
			if( data == nil )
				super(389)
			else
				super(389, data)
			end
		end
	end

	class BidID < Quickfix::StringField
		def BidID.field
			return 390
		end
		def initialize(data = nil)
			if( data == nil )
				super(390)
			else
				super(390, data)
			end
		end
	end

	class ClientBidID < Quickfix::StringField
		def ClientBidID.field
			return 391
		end
		def initialize(data = nil)
			if( data == nil )
				super(391)
			else
				super(391, data)
			end
		end
	end

	class ListName < Quickfix::StringField
		def ListName.field
			return 392
		end
		def initialize(data = nil)
			if( data == nil )
				super(392)
			else
				super(392, data)
			end
		end
	end

	class TotalNumSecurities < Quickfix::IntField
		def TotalNumSecurities.field
			return 393
		end
		def initialize(data = nil)
			if( data == nil )
				super(393)
			else
				super(393, data)
			end
		end
	end

	class BidType < Quickfix::IntField
		def BidType.field
			return 394
		end
		def initialize(data = nil)
			if( data == nil )
				super(394)
			else
				super(394, data)
			end
		end
	end

	class NumTickets < Quickfix::IntField
		def NumTickets.field
			return 395
		end
		def initialize(data = nil)
			if( data == nil )
				super(395)
			else
				super(395, data)
			end
		end
	end

	class SideValue1 < Quickfix::DoubleField
		def SideValue1.field
			return 396
		end
		def initialize(data = nil)
			if( data == nil )
				super(396)
			else
				super(396, data)
			end
		end
	end

	class SideValue2 < Quickfix::DoubleField
		def SideValue2.field
			return 397
		end
		def initialize(data = nil)
			if( data == nil )
				super(397)
			else
				super(397, data)
			end
		end
	end

	class NoBidDescriptors < Quickfix::IntField
		def NoBidDescriptors.field
			return 398
		end
		def initialize(data = nil)
			if( data == nil )
				super(398)
			else
				super(398, data)
			end
		end
	end

	class BidDescriptorType < Quickfix::IntField
		def BidDescriptorType.field
			return 399
		end
		def initialize(data = nil)
			if( data == nil )
				super(399)
			else
				super(399, data)
			end
		end
	end

	class BidDescriptor < Quickfix::StringField
		def BidDescriptor.field
			return 400
		end
		def initialize(data = nil)
			if( data == nil )
				super(400)
			else
				super(400, data)
			end
		end
	end

	class SideValueInd < Quickfix::IntField
		def SideValueInd.field
			return 401
		end
		def initialize(data = nil)
			if( data == nil )
				super(401)
			else
				super(401, data)
			end
		end
	end

	class LiquidityPctLow < Quickfix::DoubleField
		def LiquidityPctLow.field
			return 402
		end
		def initialize(data = nil)
			if( data == nil )
				super(402)
			else
				super(402, data)
			end
		end
	end

	class LiquidityPctHigh < Quickfix::DoubleField
		def LiquidityPctHigh.field
			return 403
		end
		def initialize(data = nil)
			if( data == nil )
				super(403)
			else
				super(403, data)
			end
		end
	end

	class LiquidityValue < Quickfix::DoubleField
		def LiquidityValue.field
			return 404
		end
		def initialize(data = nil)
			if( data == nil )
				super(404)
			else
				super(404, data)
			end
		end
	end

	class EFPTrackingError < Quickfix::DoubleField
		def EFPTrackingError.field
			return 405
		end
		def initialize(data = nil)
			if( data == nil )
				super(405)
			else
				super(405, data)
			end
		end
	end

	class FairValue < Quickfix::DoubleField
		def FairValue.field
			return 406
		end
		def initialize(data = nil)
			if( data == nil )
				super(406)
			else
				super(406, data)
			end
		end
	end

	class OutsideIndexPct < Quickfix::DoubleField
		def OutsideIndexPct.field
			return 407
		end
		def initialize(data = nil)
			if( data == nil )
				super(407)
			else
				super(407, data)
			end
		end
	end

	class ValueOfFutures < Quickfix::DoubleField
		def ValueOfFutures.field
			return 408
		end
		def initialize(data = nil)
			if( data == nil )
				super(408)
			else
				super(408, data)
			end
		end
	end

	class LiquidityIndType < Quickfix::IntField
		def LiquidityIndType.field
			return 409
		end
		def initialize(data = nil)
			if( data == nil )
				super(409)
			else
				super(409, data)
			end
		end
	end

	class WtAverageLiquidity < Quickfix::DoubleField
		def WtAverageLiquidity.field
			return 410
		end
		def initialize(data = nil)
			if( data == nil )
				super(410)
			else
				super(410, data)
			end
		end
	end

	class ExchangeForPhysical < Quickfix::BoolField
		def ExchangeForPhysical.field
			return 411
		end
		def initialize(data = nil)
			if( data == nil )
				super(411)
			else
				super(411, data)
			end
		end
	end

	class OutMainCntryUIndex < Quickfix::DoubleField
		def OutMainCntryUIndex.field
			return 412
		end
		def initialize(data = nil)
			if( data == nil )
				super(412)
			else
				super(412, data)
			end
		end
	end

	class CrossPercent < Quickfix::DoubleField
		def CrossPercent.field
			return 413
		end
		def initialize(data = nil)
			if( data == nil )
				super(413)
			else
				super(413, data)
			end
		end
	end

	class ProgRptReqs < Quickfix::IntField
		def ProgRptReqs.field
			return 414
		end
		def initialize(data = nil)
			if( data == nil )
				super(414)
			else
				super(414, data)
			end
		end
	end

	class ProgPeriodInterval < Quickfix::IntField
		def ProgPeriodInterval.field
			return 415
		end
		def initialize(data = nil)
			if( data == nil )
				super(415)
			else
				super(415, data)
			end
		end
	end

	class IncTaxInd < Quickfix::IntField
		def IncTaxInd.field
			return 416
		end
		def initialize(data = nil)
			if( data == nil )
				super(416)
			else
				super(416, data)
			end
		end
	end

	class NumBidders < Quickfix::IntField
		def NumBidders.field
			return 417
		end
		def initialize(data = nil)
			if( data == nil )
				super(417)
			else
				super(417, data)
			end
		end
	end

	class TradeType < Quickfix::CharField
		def TradeType.field
			return 418
		end
		def initialize(data = nil)
			if( data == nil )
				super(418)
			else
				super(418, data)
			end
		end
	end

	class BasisPxType < Quickfix::CharField
		def BasisPxType.field
			return 419
		end
		def initialize(data = nil)
			if( data == nil )
				super(419)
			else
				super(419, data)
			end
		end
	end

	class NoBidComponents < Quickfix::IntField
		def NoBidComponents.field
			return 420
		end
		def initialize(data = nil)
			if( data == nil )
				super(420)
			else
				super(420, data)
			end
		end
	end

	class Country < Quickfix::StringField
		def Country.field
			return 421
		end
		def initialize(data = nil)
			if( data == nil )
				super(421)
			else
				super(421, data)
			end
		end
	end

	class TotNoStrikes < Quickfix::IntField
		def TotNoStrikes.field
			return 422
		end
		def initialize(data = nil)
			if( data == nil )
				super(422)
			else
				super(422, data)
			end
		end
	end

	class PriceType < Quickfix::IntField
		def PriceType.field
			return 423
		end
		def initialize(data = nil)
			if( data == nil )
				super(423)
			else
				super(423, data)
			end
		end
	end

	class DayOrderQty < Quickfix::DoubleField
		def DayOrderQty.field
			return 424
		end
		def initialize(data = nil)
			if( data == nil )
				super(424)
			else
				super(424, data)
			end
		end
	end

	class DayCumQty < Quickfix::DoubleField
		def DayCumQty.field
			return 425
		end
		def initialize(data = nil)
			if( data == nil )
				super(425)
			else
				super(425, data)
			end
		end
	end

	class DayAvgPx < Quickfix::DoubleField
		def DayAvgPx.field
			return 426
		end
		def initialize(data = nil)
			if( data == nil )
				super(426)
			else
				super(426, data)
			end
		end
	end

	class GTBookingInst < Quickfix::IntField
		def GTBookingInst.field
			return 427
		end
		def initialize(data = nil)
			if( data == nil )
				super(427)
			else
				super(427, data)
			end
		end
	end

	class NoStrikes < Quickfix::IntField
		def NoStrikes.field
			return 428
		end
		def initialize(data = nil)
			if( data == nil )
				super(428)
			else
				super(428, data)
			end
		end
	end

	class ListStatusType < Quickfix::IntField
		def ListStatusType.field
			return 429
		end
		def initialize(data = nil)
			if( data == nil )
				super(429)
			else
				super(429, data)
			end
		end
	end

	class NetGrossInd < Quickfix::IntField
		def NetGrossInd.field
			return 430
		end
		def initialize(data = nil)
			if( data == nil )
				super(430)
			else
				super(430, data)
			end
		end
	end

	class ListOrderStatus < Quickfix::IntField
		def ListOrderStatus.field
			return 431
		end
		def initialize(data = nil)
			if( data == nil )
				super(431)
			else
				super(431, data)
			end
		end
	end

	class ExpireDate < Quickfix::StringField
		def ExpireDate.field
			return 432
		end
		def initialize(data = nil)
			if( data == nil )
				super(432)
			else
				super(432, data)
			end
		end
	end

	class ListExecInstType < Quickfix::CharField
		def ListExecInstType.field
			return 433
		end
		def initialize(data = nil)
			if( data == nil )
				super(433)
			else
				super(433, data)
			end
		end
	end

	class CxlRejResponseTo < Quickfix::CharField
		def CxlRejResponseTo.field
			return 434
		end
		def initialize(data = nil)
			if( data == nil )
				super(434)
			else
				super(434, data)
			end
		end
	end

	class UnderlyingCouponRate < Quickfix::DoubleField
		def UnderlyingCouponRate.field
			return 435
		end
		def initialize(data = nil)
			if( data == nil )
				super(435)
			else
				super(435, data)
			end
		end
	end

	class UnderlyingContractMultiplier < Quickfix::DoubleField
		def UnderlyingContractMultiplier.field
			return 436
		end
		def initialize(data = nil)
			if( data == nil )
				super(436)
			else
				super(436, data)
			end
		end
	end

	class ContraTradeQty < Quickfix::DoubleField
		def ContraTradeQty.field
			return 437
		end
		def initialize(data = nil)
			if( data == nil )
				super(437)
			else
				super(437, data)
			end
		end
	end

	class ContraTradeTime < Quickfix::UtcTimeStampField
		def ContraTradeTime.field
			return 438
		end
		def initialize(data = nil)
			if( data == nil )
				super(438)
			else
				super(438, data)
			end
		end
	end

	class ClearingFirm < Quickfix::StringField
		def ClearingFirm.field
			return 439
		end
		def initialize(data = nil)
			if( data == nil )
				super(439)
			else
				super(439, data)
			end
		end
	end

	class ClearingAccount < Quickfix::StringField
		def ClearingAccount.field
			return 440
		end
		def initialize(data = nil)
			if( data == nil )
				super(440)
			else
				super(440, data)
			end
		end
	end

	class LiquidityNumSecurities < Quickfix::IntField
		def LiquidityNumSecurities.field
			return 441
		end
		def initialize(data = nil)
			if( data == nil )
				super(441)
			else
				super(441, data)
			end
		end
	end

	class MultiLegReportingType < Quickfix::CharField
		def MultiLegReportingType.field
			return 442
		end
		def initialize(data = nil)
			if( data == nil )
				super(442)
			else
				super(442, data)
			end
		end
	end

	class StrikeTime < Quickfix::UtcTimeStampField
		def StrikeTime.field
			return 443
		end
		def initialize(data = nil)
			if( data == nil )
				super(443)
			else
				super(443, data)
			end
		end
	end

	class ListStatusText < Quickfix::StringField
		def ListStatusText.field
			return 444
		end
		def initialize(data = nil)
			if( data == nil )
				super(444)
			else
				super(444, data)
			end
		end
	end

	class EncodedListStatusTextLen < Quickfix::IntField
		def EncodedListStatusTextLen.field
			return 445
		end
		def initialize(data = nil)
			if( data == nil )
				super(445)
			else
				super(445, data)
			end
		end
	end

	class EncodedListStatusText < Quickfix::StringField
		def EncodedListStatusText.field
			return 446
		end
		def initialize(data = nil)
			if( data == nil )
				super(446)
			else
				super(446, data)
			end
		end
	end

	class SecurityIDSource < Quickfix::StringField
		def SecurityIDSource.field
			return 22
		end
		def initialize(data = nil)
			if( data == nil )
				super(22)
			else
				super(22, data)
			end
		end
	end

	class IOIQty < Quickfix::StringField
		def IOIQty.field
			return 27
		end
		def initialize(data = nil)
			if( data == nil )
				super(27)
			else
				super(27, data)
			end
		end
	end

	class LastQty < Quickfix::DoubleField
		def LastQty.field
			return 32
		end
		def initialize(data = nil)
			if( data == nil )
				super(32)
			else
				super(32, data)
			end
		end
	end

	class Quantity < Quickfix::DoubleField
		def Quantity.field
			return 53
		end
		def initialize(data = nil)
			if( data == nil )
				super(53)
			else
				super(53, data)
			end
		end
	end

	class PositionEffect < Quickfix::CharField
		def PositionEffect.field
			return 77
		end
		def initialize(data = nil)
			if( data == nil )
				super(77)
			else
				super(77, data)
			end
		end
	end

	class AllocQty < Quickfix::DoubleField
		def AllocQty.field
			return 80
		end
		def initialize(data = nil)
			if( data == nil )
				super(80)
			else
				super(80, data)
			end
		end
	end

	class Spread < Quickfix::DoubleField
		def Spread.field
			return 218
		end
		def initialize(data = nil)
			if( data == nil )
				super(218)
			else
				super(218, data)
			end
		end
	end

	class BenchmarkCurveCurrency < Quickfix::StringField
		def BenchmarkCurveCurrency.field
			return 220
		end
		def initialize(data = nil)
			if( data == nil )
				super(220)
			else
				super(220, data)
			end
		end
	end

	class BenchmarkCurveName < Quickfix::StringField
		def BenchmarkCurveName.field
			return 221
		end
		def initialize(data = nil)
			if( data == nil )
				super(221)
			else
				super(221, data)
			end
		end
	end

	class BenchmarkCurvePoint < Quickfix::StringField
		def BenchmarkCurvePoint.field
			return 222
		end
		def initialize(data = nil)
			if( data == nil )
				super(222)
			else
				super(222, data)
			end
		end
	end

	class CouponPaymentDate < Quickfix::StringField
		def CouponPaymentDate.field
			return 224
		end
		def initialize(data = nil)
			if( data == nil )
				super(224)
			else
				super(224, data)
			end
		end
	end

	class IssueDate < Quickfix::StringField
		def IssueDate.field
			return 225
		end
		def initialize(data = nil)
			if( data == nil )
				super(225)
			else
				super(225, data)
			end
		end
	end

	class RepurchaseTerm < Quickfix::IntField
		def RepurchaseTerm.field
			return 226
		end
		def initialize(data = nil)
			if( data == nil )
				super(226)
			else
				super(226, data)
			end
		end
	end

	class RepurchaseRate < Quickfix::DoubleField
		def RepurchaseRate.field
			return 227
		end
		def initialize(data = nil)
			if( data == nil )
				super(227)
			else
				super(227, data)
			end
		end
	end

	class Factor < Quickfix::DoubleField
		def Factor.field
			return 228
		end
		def initialize(data = nil)
			if( data == nil )
				super(228)
			else
				super(228, data)
			end
		end
	end

	class TradeOriginationDate < Quickfix::StringField
		def TradeOriginationDate.field
			return 229
		end
		def initialize(data = nil)
			if( data == nil )
				super(229)
			else
				super(229, data)
			end
		end
	end

	class ExDate < Quickfix::StringField
		def ExDate.field
			return 230
		end
		def initialize(data = nil)
			if( data == nil )
				super(230)
			else
				super(230, data)
			end
		end
	end

	class NoStipulations < Quickfix::IntField
		def NoStipulations.field
			return 232
		end
		def initialize(data = nil)
			if( data == nil )
				super(232)
			else
				super(232, data)
			end
		end
	end

	class StipulationType < Quickfix::StringField
		def StipulationType.field
			return 233
		end
		def initialize(data = nil)
			if( data == nil )
				super(233)
			else
				super(233, data)
			end
		end
	end

	class StipulationValue < Quickfix::StringField
		def StipulationValue.field
			return 234
		end
		def initialize(data = nil)
			if( data == nil )
				super(234)
			else
				super(234, data)
			end
		end
	end

	class YieldType < Quickfix::StringField
		def YieldType.field
			return 235
		end
		def initialize(data = nil)
			if( data == nil )
				super(235)
			else
				super(235, data)
			end
		end
	end

	class Yield < Quickfix::DoubleField
		def Yield.field
			return 236
		end
		def initialize(data = nil)
			if( data == nil )
				super(236)
			else
				super(236, data)
			end
		end
	end

	class TotalTakedown < Quickfix::DoubleField
		def TotalTakedown.field
			return 237
		end
		def initialize(data = nil)
			if( data == nil )
				super(237)
			else
				super(237, data)
			end
		end
	end

	class Concession < Quickfix::DoubleField
		def Concession.field
			return 238
		end
		def initialize(data = nil)
			if( data == nil )
				super(238)
			else
				super(238, data)
			end
		end
	end

	class RepoCollateralSecurityType < Quickfix::StringField
		def RepoCollateralSecurityType.field
			return 239
		end
		def initialize(data = nil)
			if( data == nil )
				super(239)
			else
				super(239, data)
			end
		end
	end

	class RedemptionDate < Quickfix::StringField
		def RedemptionDate.field
			return 240
		end
		def initialize(data = nil)
			if( data == nil )
				super(240)
			else
				super(240, data)
			end
		end
	end

	class UnderlyingCouponPaymentDate < Quickfix::StringField
		def UnderlyingCouponPaymentDate.field
			return 241
		end
		def initialize(data = nil)
			if( data == nil )
				super(241)
			else
				super(241, data)
			end
		end
	end

	class UnderlyingIssueDate < Quickfix::StringField
		def UnderlyingIssueDate.field
			return 242
		end
		def initialize(data = nil)
			if( data == nil )
				super(242)
			else
				super(242, data)
			end
		end
	end

	class UnderlyingRepoCollateralSecurityType < Quickfix::StringField
		def UnderlyingRepoCollateralSecurityType.field
			return 243
		end
		def initialize(data = nil)
			if( data == nil )
				super(243)
			else
				super(243, data)
			end
		end
	end

	class UnderlyingRepurchaseTerm < Quickfix::IntField
		def UnderlyingRepurchaseTerm.field
			return 244
		end
		def initialize(data = nil)
			if( data == nil )
				super(244)
			else
				super(244, data)
			end
		end
	end

	class UnderlyingRepurchaseRate < Quickfix::DoubleField
		def UnderlyingRepurchaseRate.field
			return 245
		end
		def initialize(data = nil)
			if( data == nil )
				super(245)
			else
				super(245, data)
			end
		end
	end

	class UnderlyingFactor < Quickfix::DoubleField
		def UnderlyingFactor.field
			return 246
		end
		def initialize(data = nil)
			if( data == nil )
				super(246)
			else
				super(246, data)
			end
		end
	end

	class UnderlyingRedemptionDate < Quickfix::StringField
		def UnderlyingRedemptionDate.field
			return 247
		end
		def initialize(data = nil)
			if( data == nil )
				super(247)
			else
				super(247, data)
			end
		end
	end

	class LegCouponPaymentDate < Quickfix::StringField
		def LegCouponPaymentDate.field
			return 248
		end
		def initialize(data = nil)
			if( data == nil )
				super(248)
			else
				super(248, data)
			end
		end
	end

	class LegIssueDate < Quickfix::StringField
		def LegIssueDate.field
			return 249
		end
		def initialize(data = nil)
			if( data == nil )
				super(249)
			else
				super(249, data)
			end
		end
	end

	class LegRepoCollateralSecurityType < Quickfix::StringField
		def LegRepoCollateralSecurityType.field
			return 250
		end
		def initialize(data = nil)
			if( data == nil )
				super(250)
			else
				super(250, data)
			end
		end
	end

	class LegRepurchaseTerm < Quickfix::IntField
		def LegRepurchaseTerm.field
			return 251
		end
		def initialize(data = nil)
			if( data == nil )
				super(251)
			else
				super(251, data)
			end
		end
	end

	class LegRepurchaseRate < Quickfix::DoubleField
		def LegRepurchaseRate.field
			return 252
		end
		def initialize(data = nil)
			if( data == nil )
				super(252)
			else
				super(252, data)
			end
		end
	end

	class LegFactor < Quickfix::DoubleField
		def LegFactor.field
			return 253
		end
		def initialize(data = nil)
			if( data == nil )
				super(253)
			else
				super(253, data)
			end
		end
	end

	class LegRedemptionDate < Quickfix::StringField
		def LegRedemptionDate.field
			return 254
		end
		def initialize(data = nil)
			if( data == nil )
				super(254)
			else
				super(254, data)
			end
		end
	end

	class CreditRating < Quickfix::StringField
		def CreditRating.field
			return 255
		end
		def initialize(data = nil)
			if( data == nil )
				super(255)
			else
				super(255, data)
			end
		end
	end

	class UnderlyingCreditRating < Quickfix::StringField
		def UnderlyingCreditRating.field
			return 256
		end
		def initialize(data = nil)
			if( data == nil )
				super(256)
			else
				super(256, data)
			end
		end
	end

	class LegCreditRating < Quickfix::StringField
		def LegCreditRating.field
			return 257
		end
		def initialize(data = nil)
			if( data == nil )
				super(257)
			else
				super(257, data)
			end
		end
	end

	class TradedFlatSwitch < Quickfix::BoolField
		def TradedFlatSwitch.field
			return 258
		end
		def initialize(data = nil)
			if( data == nil )
				super(258)
			else
				super(258, data)
			end
		end
	end

	class BasisFeatureDate < Quickfix::StringField
		def BasisFeatureDate.field
			return 259
		end
		def initialize(data = nil)
			if( data == nil )
				super(259)
			else
				super(259, data)
			end
		end
	end

	class BasisFeaturePrice < Quickfix::DoubleField
		def BasisFeaturePrice.field
			return 260
		end
		def initialize(data = nil)
			if( data == nil )
				super(260)
			else
				super(260, data)
			end
		end
	end

	class QuoteStatus < Quickfix::IntField
		def QuoteStatus.field
			return 297
		end
		def initialize(data = nil)
			if( data == nil )
				super(297)
			else
				super(297, data)
			end
		end
	end

	class UnderlyingSecurityIDSource < Quickfix::StringField
		def UnderlyingSecurityIDSource.field
			return 305
		end
		def initialize(data = nil)
			if( data == nil )
				super(305)
			else
				super(305, data)
			end
		end
	end

	class PartyIDSource < Quickfix::CharField
		def PartyIDSource.field
			return 447
		end
		def initialize(data = nil)
			if( data == nil )
				super(447)
			else
				super(447, data)
			end
		end
	end

	class PartyID < Quickfix::StringField
		def PartyID.field
			return 448
		end
		def initialize(data = nil)
			if( data == nil )
				super(448)
			else
				super(448, data)
			end
		end
	end

	class TotalVolumeTradedDate < Quickfix::UtcDateField
		def TotalVolumeTradedDate.field
			return 449
		end
		def initialize(data = nil)
			if( data == nil )
				super(449)
			else
				super(449, data)
			end
		end
	end

	class TotalVolumeTradedTime < Quickfix::UtcTimeOnlyField
		def TotalVolumeTradedTime.field
			return 450
		end
		def initialize(data = nil)
			if( data == nil )
				super(450)
			else
				super(450, data)
			end
		end
	end

	class NetChgPrevDay < Quickfix::DoubleField
		def NetChgPrevDay.field
			return 451
		end
		def initialize(data = nil)
			if( data == nil )
				super(451)
			else
				super(451, data)
			end
		end
	end

	class PartyRole < Quickfix::IntField
		def PartyRole.field
			return 452
		end
		def initialize(data = nil)
			if( data == nil )
				super(452)
			else
				super(452, data)
			end
		end
	end

	class NoPartyIDs < Quickfix::IntField
		def NoPartyIDs.field
			return 453
		end
		def initialize(data = nil)
			if( data == nil )
				super(453)
			else
				super(453, data)
			end
		end
	end

	class NoSecurityAltID < Quickfix::IntField
		def NoSecurityAltID.field
			return 454
		end
		def initialize(data = nil)
			if( data == nil )
				super(454)
			else
				super(454, data)
			end
		end
	end

	class SecurityAltID < Quickfix::StringField
		def SecurityAltID.field
			return 455
		end
		def initialize(data = nil)
			if( data == nil )
				super(455)
			else
				super(455, data)
			end
		end
	end

	class SecurityAltIDSource < Quickfix::StringField
		def SecurityAltIDSource.field
			return 456
		end
		def initialize(data = nil)
			if( data == nil )
				super(456)
			else
				super(456, data)
			end
		end
	end

	class NoUnderlyingSecurityAltID < Quickfix::IntField
		def NoUnderlyingSecurityAltID.field
			return 457
		end
		def initialize(data = nil)
			if( data == nil )
				super(457)
			else
				super(457, data)
			end
		end
	end

	class UnderlyingSecurityAltID < Quickfix::StringField
		def UnderlyingSecurityAltID.field
			return 458
		end
		def initialize(data = nil)
			if( data == nil )
				super(458)
			else
				super(458, data)
			end
		end
	end

	class UnderlyingSecurityAltIDSource < Quickfix::StringField
		def UnderlyingSecurityAltIDSource.field
			return 459
		end
		def initialize(data = nil)
			if( data == nil )
				super(459)
			else
				super(459, data)
			end
		end
	end

	class Product < Quickfix::IntField
		def Product.field
			return 460
		end
		def initialize(data = nil)
			if( data == nil )
				super(460)
			else
				super(460, data)
			end
		end
	end

	class CFICode < Quickfix::StringField
		def CFICode.field
			return 461
		end
		def initialize(data = nil)
			if( data == nil )
				super(461)
			else
				super(461, data)
			end
		end
	end

	class UnderlyingProduct < Quickfix::IntField
		def UnderlyingProduct.field
			return 462
		end
		def initialize(data = nil)
			if( data == nil )
				super(462)
			else
				super(462, data)
			end
		end
	end

	class UnderlyingCFICode < Quickfix::StringField
		def UnderlyingCFICode.field
			return 463
		end
		def initialize(data = nil)
			if( data == nil )
				super(463)
			else
				super(463, data)
			end
		end
	end

	class QuantityType < Quickfix::IntField
		def QuantityType.field
			return 465
		end
		def initialize(data = nil)
			if( data == nil )
				super(465)
			else
				super(465, data)
			end
		end
	end

	class BookingRefID < Quickfix::StringField
		def BookingRefID.field
			return 466
		end
		def initialize(data = nil)
			if( data == nil )
				super(466)
			else
				super(466, data)
			end
		end
	end

	class IndividualAllocID < Quickfix::StringField
		def IndividualAllocID.field
			return 467
		end
		def initialize(data = nil)
			if( data == nil )
				super(467)
			else
				super(467, data)
			end
		end
	end

	class RoundingDirection < Quickfix::CharField
		def RoundingDirection.field
			return 468
		end
		def initialize(data = nil)
			if( data == nil )
				super(468)
			else
				super(468, data)
			end
		end
	end

	class RoundingModulus < Quickfix::DoubleField
		def RoundingModulus.field
			return 469
		end
		def initialize(data = nil)
			if( data == nil )
				super(469)
			else
				super(469, data)
			end
		end
	end

	class CountryOfIssue < Quickfix::StringField
		def CountryOfIssue.field
			return 470
		end
		def initialize(data = nil)
			if( data == nil )
				super(470)
			else
				super(470, data)
			end
		end
	end

	class StateOrProvinceOfIssue < Quickfix::StringField
		def StateOrProvinceOfIssue.field
			return 471
		end
		def initialize(data = nil)
			if( data == nil )
				super(471)
			else
				super(471, data)
			end
		end
	end

	class LocaleOfIssue < Quickfix::StringField
		def LocaleOfIssue.field
			return 472
		end
		def initialize(data = nil)
			if( data == nil )
				super(472)
			else
				super(472, data)
			end
		end
	end

	class NoRegistDtls < Quickfix::IntField
		def NoRegistDtls.field
			return 473
		end
		def initialize(data = nil)
			if( data == nil )
				super(473)
			else
				super(473, data)
			end
		end
	end

	class MailingDtls < Quickfix::StringField
		def MailingDtls.field
			return 474
		end
		def initialize(data = nil)
			if( data == nil )
				super(474)
			else
				super(474, data)
			end
		end
	end

	class InvestorCountryOfResidence < Quickfix::StringField
		def InvestorCountryOfResidence.field
			return 475
		end
		def initialize(data = nil)
			if( data == nil )
				super(475)
			else
				super(475, data)
			end
		end
	end

	class PaymentRef < Quickfix::StringField
		def PaymentRef.field
			return 476
		end
		def initialize(data = nil)
			if( data == nil )
				super(476)
			else
				super(476, data)
			end
		end
	end

	class DistribPaymentMethod < Quickfix::IntField
		def DistribPaymentMethod.field
			return 477
		end
		def initialize(data = nil)
			if( data == nil )
				super(477)
			else
				super(477, data)
			end
		end
	end

	class CashDistribCurr < Quickfix::StringField
		def CashDistribCurr.field
			return 478
		end
		def initialize(data = nil)
			if( data == nil )
				super(478)
			else
				super(478, data)
			end
		end
	end

	class CommCurrency < Quickfix::StringField
		def CommCurrency.field
			return 479
		end
		def initialize(data = nil)
			if( data == nil )
				super(479)
			else
				super(479, data)
			end
		end
	end

	class CancellationRights < Quickfix::CharField
		def CancellationRights.field
			return 480
		end
		def initialize(data = nil)
			if( data == nil )
				super(480)
			else
				super(480, data)
			end
		end
	end

	class MoneyLaunderingStatus < Quickfix::CharField
		def MoneyLaunderingStatus.field
			return 481
		end
		def initialize(data = nil)
			if( data == nil )
				super(481)
			else
				super(481, data)
			end
		end
	end

	class MailingInst < Quickfix::StringField
		def MailingInst.field
			return 482
		end
		def initialize(data = nil)
			if( data == nil )
				super(482)
			else
				super(482, data)
			end
		end
	end

	class TransBkdTime < Quickfix::UtcTimeStampField
		def TransBkdTime.field
			return 483
		end
		def initialize(data = nil)
			if( data == nil )
				super(483)
			else
				super(483, data)
			end
		end
	end

	class ExecPriceType < Quickfix::CharField
		def ExecPriceType.field
			return 484
		end
		def initialize(data = nil)
			if( data == nil )
				super(484)
			else
				super(484, data)
			end
		end
	end

	class ExecPriceAdjustment < Quickfix::DoubleField
		def ExecPriceAdjustment.field
			return 485
		end
		def initialize(data = nil)
			if( data == nil )
				super(485)
			else
				super(485, data)
			end
		end
	end

	class DateOfBirth < Quickfix::StringField
		def DateOfBirth.field
			return 486
		end
		def initialize(data = nil)
			if( data == nil )
				super(486)
			else
				super(486, data)
			end
		end
	end

	class TradeReportTransType < Quickfix::IntField
		def TradeReportTransType.field
			return 487
		end
		def initialize(data = nil)
			if( data == nil )
				super(487)
			else
				super(487, data)
			end
		end
	end

	class CardHolderName < Quickfix::StringField
		def CardHolderName.field
			return 488
		end
		def initialize(data = nil)
			if( data == nil )
				super(488)
			else
				super(488, data)
			end
		end
	end

	class CardNumber < Quickfix::StringField
		def CardNumber.field
			return 489
		end
		def initialize(data = nil)
			if( data == nil )
				super(489)
			else
				super(489, data)
			end
		end
	end

	class CardExpDate < Quickfix::StringField
		def CardExpDate.field
			return 490
		end
		def initialize(data = nil)
			if( data == nil )
				super(490)
			else
				super(490, data)
			end
		end
	end

	class CardIssNo < Quickfix::StringField
		def CardIssNo.field
			return 491
		end
		def initialize(data = nil)
			if( data == nil )
				super(491)
			else
				super(491, data)
			end
		end
	end

	class PaymentMethod < Quickfix::IntField
		def PaymentMethod.field
			return 492
		end
		def initialize(data = nil)
			if( data == nil )
				super(492)
			else
				super(492, data)
			end
		end
	end

	class RegistAcctType < Quickfix::StringField
		def RegistAcctType.field
			return 493
		end
		def initialize(data = nil)
			if( data == nil )
				super(493)
			else
				super(493, data)
			end
		end
	end

	class Designation < Quickfix::StringField
		def Designation.field
			return 494
		end
		def initialize(data = nil)
			if( data == nil )
				super(494)
			else
				super(494, data)
			end
		end
	end

	class TaxAdvantageType < Quickfix::IntField
		def TaxAdvantageType.field
			return 495
		end
		def initialize(data = nil)
			if( data == nil )
				super(495)
			else
				super(495, data)
			end
		end
	end

	class RegistRejReasonText < Quickfix::StringField
		def RegistRejReasonText.field
			return 496
		end
		def initialize(data = nil)
			if( data == nil )
				super(496)
			else
				super(496, data)
			end
		end
	end

	class FundRenewWaiv < Quickfix::CharField
		def FundRenewWaiv.field
			return 497
		end
		def initialize(data = nil)
			if( data == nil )
				super(497)
			else
				super(497, data)
			end
		end
	end

	class CashDistribAgentName < Quickfix::StringField
		def CashDistribAgentName.field
			return 498
		end
		def initialize(data = nil)
			if( data == nil )
				super(498)
			else
				super(498, data)
			end
		end
	end

	class CashDistribAgentCode < Quickfix::StringField
		def CashDistribAgentCode.field
			return 499
		end
		def initialize(data = nil)
			if( data == nil )
				super(499)
			else
				super(499, data)
			end
		end
	end

	class CashDistribAgentAcctNumber < Quickfix::StringField
		def CashDistribAgentAcctNumber.field
			return 500
		end
		def initialize(data = nil)
			if( data == nil )
				super(500)
			else
				super(500, data)
			end
		end
	end

	class CashDistribPayRef < Quickfix::StringField
		def CashDistribPayRef.field
			return 501
		end
		def initialize(data = nil)
			if( data == nil )
				super(501)
			else
				super(501, data)
			end
		end
	end

	class CardStartDate < Quickfix::StringField
		def CardStartDate.field
			return 503
		end
		def initialize(data = nil)
			if( data == nil )
				super(503)
			else
				super(503, data)
			end
		end
	end

	class PaymentDate < Quickfix::StringField
		def PaymentDate.field
			return 504
		end
		def initialize(data = nil)
			if( data == nil )
				super(504)
			else
				super(504, data)
			end
		end
	end

	class PaymentRemitterID < Quickfix::StringField
		def PaymentRemitterID.field
			return 505
		end
		def initialize(data = nil)
			if( data == nil )
				super(505)
			else
				super(505, data)
			end
		end
	end

	class RegistStatus < Quickfix::CharField
		def RegistStatus.field
			return 506
		end
		def initialize(data = nil)
			if( data == nil )
				super(506)
			else
				super(506, data)
			end
		end
	end

	class RegistRejReasonCode < Quickfix::IntField
		def RegistRejReasonCode.field
			return 507
		end
		def initialize(data = nil)
			if( data == nil )
				super(507)
			else
				super(507, data)
			end
		end
	end

	class RegistRefID < Quickfix::StringField
		def RegistRefID.field
			return 508
		end
		def initialize(data = nil)
			if( data == nil )
				super(508)
			else
				super(508, data)
			end
		end
	end

	class RegistDetls < Quickfix::StringField
		def RegistDetls.field
			return 509
		end
		def initialize(data = nil)
			if( data == nil )
				super(509)
			else
				super(509, data)
			end
		end
	end

	class NoDistribInsts < Quickfix::IntField
		def NoDistribInsts.field
			return 510
		end
		def initialize(data = nil)
			if( data == nil )
				super(510)
			else
				super(510, data)
			end
		end
	end

	class RegistEmail < Quickfix::StringField
		def RegistEmail.field
			return 511
		end
		def initialize(data = nil)
			if( data == nil )
				super(511)
			else
				super(511, data)
			end
		end
	end

	class DistribPercentage < Quickfix::DoubleField
		def DistribPercentage.field
			return 512
		end
		def initialize(data = nil)
			if( data == nil )
				super(512)
			else
				super(512, data)
			end
		end
	end

	class RegistID < Quickfix::StringField
		def RegistID.field
			return 513
		end
		def initialize(data = nil)
			if( data == nil )
				super(513)
			else
				super(513, data)
			end
		end
	end

	class RegistTransType < Quickfix::CharField
		def RegistTransType.field
			return 514
		end
		def initialize(data = nil)
			if( data == nil )
				super(514)
			else
				super(514, data)
			end
		end
	end

	class ExecValuationPoint < Quickfix::UtcTimeStampField
		def ExecValuationPoint.field
			return 515
		end
		def initialize(data = nil)
			if( data == nil )
				super(515)
			else
				super(515, data)
			end
		end
	end

	class OrderPercent < Quickfix::DoubleField
		def OrderPercent.field
			return 516
		end
		def initialize(data = nil)
			if( data == nil )
				super(516)
			else
				super(516, data)
			end
		end
	end

	class OwnershipType < Quickfix::CharField
		def OwnershipType.field
			return 517
		end
		def initialize(data = nil)
			if( data == nil )
				super(517)
			else
				super(517, data)
			end
		end
	end

	class NoContAmts < Quickfix::IntField
		def NoContAmts.field
			return 518
		end
		def initialize(data = nil)
			if( data == nil )
				super(518)
			else
				super(518, data)
			end
		end
	end

	class ContAmtType < Quickfix::IntField
		def ContAmtType.field
			return 519
		end
		def initialize(data = nil)
			if( data == nil )
				super(519)
			else
				super(519, data)
			end
		end
	end

	class ContAmtValue < Quickfix::DoubleField
		def ContAmtValue.field
			return 520
		end
		def initialize(data = nil)
			if( data == nil )
				super(520)
			else
				super(520, data)
			end
		end
	end

	class ContAmtCurr < Quickfix::StringField
		def ContAmtCurr.field
			return 521
		end
		def initialize(data = nil)
			if( data == nil )
				super(521)
			else
				super(521, data)
			end
		end
	end

	class OwnerType < Quickfix::IntField
		def OwnerType.field
			return 522
		end
		def initialize(data = nil)
			if( data == nil )
				super(522)
			else
				super(522, data)
			end
		end
	end

	class PartySubID < Quickfix::StringField
		def PartySubID.field
			return 523
		end
		def initialize(data = nil)
			if( data == nil )
				super(523)
			else
				super(523, data)
			end
		end
	end

	class NestedPartyID < Quickfix::StringField
		def NestedPartyID.field
			return 524
		end
		def initialize(data = nil)
			if( data == nil )
				super(524)
			else
				super(524, data)
			end
		end
	end

	class NestedPartyIDSource < Quickfix::CharField
		def NestedPartyIDSource.field
			return 525
		end
		def initialize(data = nil)
			if( data == nil )
				super(525)
			else
				super(525, data)
			end
		end
	end

	class SecondaryClOrdID < Quickfix::StringField
		def SecondaryClOrdID.field
			return 526
		end
		def initialize(data = nil)
			if( data == nil )
				super(526)
			else
				super(526, data)
			end
		end
	end

	class SecondaryExecID < Quickfix::StringField
		def SecondaryExecID.field
			return 527
		end
		def initialize(data = nil)
			if( data == nil )
				super(527)
			else
				super(527, data)
			end
		end
	end

	class OrderCapacity < Quickfix::CharField
		def OrderCapacity.field
			return 528
		end
		def initialize(data = nil)
			if( data == nil )
				super(528)
			else
				super(528, data)
			end
		end
	end

	class OrderRestrictions < Quickfix::StringField
		def OrderRestrictions.field
			return 529
		end
		def initialize(data = nil)
			if( data == nil )
				super(529)
			else
				super(529, data)
			end
		end
	end

	class MassCancelRequestType < Quickfix::CharField
		def MassCancelRequestType.field
			return 530
		end
		def initialize(data = nil)
			if( data == nil )
				super(530)
			else
				super(530, data)
			end
		end
	end

	class MassCancelResponse < Quickfix::CharField
		def MassCancelResponse.field
			return 531
		end
		def initialize(data = nil)
			if( data == nil )
				super(531)
			else
				super(531, data)
			end
		end
	end

	class MassCancelRejectReason < Quickfix::IntField
		def MassCancelRejectReason.field
			return 532
		end
		def initialize(data = nil)
			if( data == nil )
				super(532)
			else
				super(532, data)
			end
		end
	end

	class TotalAffectedOrders < Quickfix::IntField
		def TotalAffectedOrders.field
			return 533
		end
		def initialize(data = nil)
			if( data == nil )
				super(533)
			else
				super(533, data)
			end
		end
	end

	class NoAffectedOrders < Quickfix::IntField
		def NoAffectedOrders.field
			return 534
		end
		def initialize(data = nil)
			if( data == nil )
				super(534)
			else
				super(534, data)
			end
		end
	end

	class AffectedOrderID < Quickfix::StringField
		def AffectedOrderID.field
			return 535
		end
		def initialize(data = nil)
			if( data == nil )
				super(535)
			else
				super(535, data)
			end
		end
	end

	class AffectedSecondaryOrderID < Quickfix::StringField
		def AffectedSecondaryOrderID.field
			return 536
		end
		def initialize(data = nil)
			if( data == nil )
				super(536)
			else
				super(536, data)
			end
		end
	end

	class QuoteType < Quickfix::IntField
		def QuoteType.field
			return 537
		end
		def initialize(data = nil)
			if( data == nil )
				super(537)
			else
				super(537, data)
			end
		end
	end

	class NestedPartyRole < Quickfix::IntField
		def NestedPartyRole.field
			return 538
		end
		def initialize(data = nil)
			if( data == nil )
				super(538)
			else
				super(538, data)
			end
		end
	end

	class NoNestedPartyIDs < Quickfix::IntField
		def NoNestedPartyIDs.field
			return 539
		end
		def initialize(data = nil)
			if( data == nil )
				super(539)
			else
				super(539, data)
			end
		end
	end

	class TotalAccruedInterestAmt < Quickfix::DoubleField
		def TotalAccruedInterestAmt.field
			return 540
		end
		def initialize(data = nil)
			if( data == nil )
				super(540)
			else
				super(540, data)
			end
		end
	end

	class MaturityDate < Quickfix::StringField
		def MaturityDate.field
			return 541
		end
		def initialize(data = nil)
			if( data == nil )
				super(541)
			else
				super(541, data)
			end
		end
	end

	class UnderlyingMaturityDate < Quickfix::StringField
		def UnderlyingMaturityDate.field
			return 542
		end
		def initialize(data = nil)
			if( data == nil )
				super(542)
			else
				super(542, data)
			end
		end
	end

	class InstrRegistry < Quickfix::StringField
		def InstrRegistry.field
			return 543
		end
		def initialize(data = nil)
			if( data == nil )
				super(543)
			else
				super(543, data)
			end
		end
	end

	class CashMargin < Quickfix::CharField
		def CashMargin.field
			return 544
		end
		def initialize(data = nil)
			if( data == nil )
				super(544)
			else
				super(544, data)
			end
		end
	end

	class NestedPartySubID < Quickfix::StringField
		def NestedPartySubID.field
			return 545
		end
		def initialize(data = nil)
			if( data == nil )
				super(545)
			else
				super(545, data)
			end
		end
	end

	class Scope < Quickfix::StringField
		def Scope.field
			return 546
		end
		def initialize(data = nil)
			if( data == nil )
				super(546)
			else
				super(546, data)
			end
		end
	end

	class MDImplicitDelete < Quickfix::BoolField
		def MDImplicitDelete.field
			return 547
		end
		def initialize(data = nil)
			if( data == nil )
				super(547)
			else
				super(547, data)
			end
		end
	end

	class CrossID < Quickfix::StringField
		def CrossID.field
			return 548
		end
		def initialize(data = nil)
			if( data == nil )
				super(548)
			else
				super(548, data)
			end
		end
	end

	class CrossType < Quickfix::IntField
		def CrossType.field
			return 549
		end
		def initialize(data = nil)
			if( data == nil )
				super(549)
			else
				super(549, data)
			end
		end
	end

	class CrossPrioritization < Quickfix::IntField
		def CrossPrioritization.field
			return 550
		end
		def initialize(data = nil)
			if( data == nil )
				super(550)
			else
				super(550, data)
			end
		end
	end

	class OrigCrossID < Quickfix::StringField
		def OrigCrossID.field
			return 551
		end
		def initialize(data = nil)
			if( data == nil )
				super(551)
			else
				super(551, data)
			end
		end
	end

	class NoSides < Quickfix::IntField
		def NoSides.field
			return 552
		end
		def initialize(data = nil)
			if( data == nil )
				super(552)
			else
				super(552, data)
			end
		end
	end

	class NoLegs < Quickfix::IntField
		def NoLegs.field
			return 555
		end
		def initialize(data = nil)
			if( data == nil )
				super(555)
			else
				super(555, data)
			end
		end
	end

	class LegCurrency < Quickfix::StringField
		def LegCurrency.field
			return 556
		end
		def initialize(data = nil)
			if( data == nil )
				super(556)
			else
				super(556, data)
			end
		end
	end

	class TotalNumSecurityTypes < Quickfix::IntField
		def TotalNumSecurityTypes.field
			return 557
		end
		def initialize(data = nil)
			if( data == nil )
				super(557)
			else
				super(557, data)
			end
		end
	end

	class NoSecurityTypes < Quickfix::IntField
		def NoSecurityTypes.field
			return 558
		end
		def initialize(data = nil)
			if( data == nil )
				super(558)
			else
				super(558, data)
			end
		end
	end

	class SecurityListRequestType < Quickfix::IntField
		def SecurityListRequestType.field
			return 559
		end
		def initialize(data = nil)
			if( data == nil )
				super(559)
			else
				super(559, data)
			end
		end
	end

	class SecurityRequestResult < Quickfix::IntField
		def SecurityRequestResult.field
			return 560
		end
		def initialize(data = nil)
			if( data == nil )
				super(560)
			else
				super(560, data)
			end
		end
	end

	class RoundLot < Quickfix::DoubleField
		def RoundLot.field
			return 561
		end
		def initialize(data = nil)
			if( data == nil )
				super(561)
			else
				super(561, data)
			end
		end
	end

	class MinTradeVol < Quickfix::DoubleField
		def MinTradeVol.field
			return 562
		end
		def initialize(data = nil)
			if( data == nil )
				super(562)
			else
				super(562, data)
			end
		end
	end

	class MultiLegRptTypeReq < Quickfix::IntField
		def MultiLegRptTypeReq.field
			return 563
		end
		def initialize(data = nil)
			if( data == nil )
				super(563)
			else
				super(563, data)
			end
		end
	end

	class LegPositionEffect < Quickfix::CharField
		def LegPositionEffect.field
			return 564
		end
		def initialize(data = nil)
			if( data == nil )
				super(564)
			else
				super(564, data)
			end
		end
	end

	class LegCoveredOrUncovered < Quickfix::IntField
		def LegCoveredOrUncovered.field
			return 565
		end
		def initialize(data = nil)
			if( data == nil )
				super(565)
			else
				super(565, data)
			end
		end
	end

	class LegPrice < Quickfix::DoubleField
		def LegPrice.field
			return 566
		end
		def initialize(data = nil)
			if( data == nil )
				super(566)
			else
				super(566, data)
			end
		end
	end

	class TradSesStatusRejReason < Quickfix::IntField
		def TradSesStatusRejReason.field
			return 567
		end
		def initialize(data = nil)
			if( data == nil )
				super(567)
			else
				super(567, data)
			end
		end
	end

	class TradeRequestID < Quickfix::StringField
		def TradeRequestID.field
			return 568
		end
		def initialize(data = nil)
			if( data == nil )
				super(568)
			else
				super(568, data)
			end
		end
	end

	class TradeRequestType < Quickfix::IntField
		def TradeRequestType.field
			return 569
		end
		def initialize(data = nil)
			if( data == nil )
				super(569)
			else
				super(569, data)
			end
		end
	end

	class PreviouslyReported < Quickfix::BoolField
		def PreviouslyReported.field
			return 570
		end
		def initialize(data = nil)
			if( data == nil )
				super(570)
			else
				super(570, data)
			end
		end
	end

	class TradeReportID < Quickfix::StringField
		def TradeReportID.field
			return 571
		end
		def initialize(data = nil)
			if( data == nil )
				super(571)
			else
				super(571, data)
			end
		end
	end

	class TradeReportRefID < Quickfix::StringField
		def TradeReportRefID.field
			return 572
		end
		def initialize(data = nil)
			if( data == nil )
				super(572)
			else
				super(572, data)
			end
		end
	end

	class MatchStatus < Quickfix::CharField
		def MatchStatus.field
			return 573
		end
		def initialize(data = nil)
			if( data == nil )
				super(573)
			else
				super(573, data)
			end
		end
	end

	class MatchType < Quickfix::StringField
		def MatchType.field
			return 574
		end
		def initialize(data = nil)
			if( data == nil )
				super(574)
			else
				super(574, data)
			end
		end
	end

	class OddLot < Quickfix::BoolField
		def OddLot.field
			return 575
		end
		def initialize(data = nil)
			if( data == nil )
				super(575)
			else
				super(575, data)
			end
		end
	end

	class NoClearingInstructions < Quickfix::IntField
		def NoClearingInstructions.field
			return 576
		end
		def initialize(data = nil)
			if( data == nil )
				super(576)
			else
				super(576, data)
			end
		end
	end

	class ClearingInstruction < Quickfix::IntField
		def ClearingInstruction.field
			return 577
		end
		def initialize(data = nil)
			if( data == nil )
				super(577)
			else
				super(577, data)
			end
		end
	end

	class TradeInputSource < Quickfix::StringField
		def TradeInputSource.field
			return 578
		end
		def initialize(data = nil)
			if( data == nil )
				super(578)
			else
				super(578, data)
			end
		end
	end

	class TradeInputDevice < Quickfix::StringField
		def TradeInputDevice.field
			return 579
		end
		def initialize(data = nil)
			if( data == nil )
				super(579)
			else
				super(579, data)
			end
		end
	end

	class NoDates < Quickfix::IntField
		def NoDates.field
			return 580
		end
		def initialize(data = nil)
			if( data == nil )
				super(580)
			else
				super(580, data)
			end
		end
	end

	class AccountType < Quickfix::IntField
		def AccountType.field
			return 581
		end
		def initialize(data = nil)
			if( data == nil )
				super(581)
			else
				super(581, data)
			end
		end
	end

	class CustOrderCapacity < Quickfix::IntField
		def CustOrderCapacity.field
			return 582
		end
		def initialize(data = nil)
			if( data == nil )
				super(582)
			else
				super(582, data)
			end
		end
	end

	class ClOrdLinkID < Quickfix::StringField
		def ClOrdLinkID.field
			return 583
		end
		def initialize(data = nil)
			if( data == nil )
				super(583)
			else
				super(583, data)
			end
		end
	end

	class MassStatusReqID < Quickfix::StringField
		def MassStatusReqID.field
			return 584
		end
		def initialize(data = nil)
			if( data == nil )
				super(584)
			else
				super(584, data)
			end
		end
	end

	class MassStatusReqType < Quickfix::IntField
		def MassStatusReqType.field
			return 585
		end
		def initialize(data = nil)
			if( data == nil )
				super(585)
			else
				super(585, data)
			end
		end
	end

	class OrigOrdModTime < Quickfix::UtcTimeStampField
		def OrigOrdModTime.field
			return 586
		end
		def initialize(data = nil)
			if( data == nil )
				super(586)
			else
				super(586, data)
			end
		end
	end

	class LegSettlmntTyp < Quickfix::CharField
		def LegSettlmntTyp.field
			return 587
		end
		def initialize(data = nil)
			if( data == nil )
				super(587)
			else
				super(587, data)
			end
		end
	end

	class LegFutSettDate < Quickfix::StringField
		def LegFutSettDate.field
			return 588
		end
		def initialize(data = nil)
			if( data == nil )
				super(588)
			else
				super(588, data)
			end
		end
	end

	class DayBookingInst < Quickfix::CharField
		def DayBookingInst.field
			return 589
		end
		def initialize(data = nil)
			if( data == nil )
				super(589)
			else
				super(589, data)
			end
		end
	end

	class BookingUnit < Quickfix::CharField
		def BookingUnit.field
			return 590
		end
		def initialize(data = nil)
			if( data == nil )
				super(590)
			else
				super(590, data)
			end
		end
	end

	class PreallocMethod < Quickfix::CharField
		def PreallocMethod.field
			return 591
		end
		def initialize(data = nil)
			if( data == nil )
				super(591)
			else
				super(591, data)
			end
		end
	end

	class UnderlyingCountryOfIssue < Quickfix::StringField
		def UnderlyingCountryOfIssue.field
			return 592
		end
		def initialize(data = nil)
			if( data == nil )
				super(592)
			else
				super(592, data)
			end
		end
	end

	class UnderlyingStateOrProvinceOfIssue < Quickfix::StringField
		def UnderlyingStateOrProvinceOfIssue.field
			return 593
		end
		def initialize(data = nil)
			if( data == nil )
				super(593)
			else
				super(593, data)
			end
		end
	end

	class UnderlyingLocaleOfIssue < Quickfix::StringField
		def UnderlyingLocaleOfIssue.field
			return 594
		end
		def initialize(data = nil)
			if( data == nil )
				super(594)
			else
				super(594, data)
			end
		end
	end

	class UnderlyingInstrRegistry < Quickfix::StringField
		def UnderlyingInstrRegistry.field
			return 595
		end
		def initialize(data = nil)
			if( data == nil )
				super(595)
			else
				super(595, data)
			end
		end
	end

	class LegCountryOfIssue < Quickfix::StringField
		def LegCountryOfIssue.field
			return 596
		end
		def initialize(data = nil)
			if( data == nil )
				super(596)
			else
				super(596, data)
			end
		end
	end

	class LegStateOrProvinceOfIssue < Quickfix::StringField
		def LegStateOrProvinceOfIssue.field
			return 597
		end
		def initialize(data = nil)
			if( data == nil )
				super(597)
			else
				super(597, data)
			end
		end
	end

	class LegLocaleOfIssue < Quickfix::StringField
		def LegLocaleOfIssue.field
			return 598
		end
		def initialize(data = nil)
			if( data == nil )
				super(598)
			else
				super(598, data)
			end
		end
	end

	class LegInstrRegistry < Quickfix::StringField
		def LegInstrRegistry.field
			return 599
		end
		def initialize(data = nil)
			if( data == nil )
				super(599)
			else
				super(599, data)
			end
		end
	end

	class LegSymbol < Quickfix::StringField
		def LegSymbol.field
			return 600
		end
		def initialize(data = nil)
			if( data == nil )
				super(600)
			else
				super(600, data)
			end
		end
	end

	class LegSymbolSfx < Quickfix::StringField
		def LegSymbolSfx.field
			return 601
		end
		def initialize(data = nil)
			if( data == nil )
				super(601)
			else
				super(601, data)
			end
		end
	end

	class LegSecurityID < Quickfix::StringField
		def LegSecurityID.field
			return 602
		end
		def initialize(data = nil)
			if( data == nil )
				super(602)
			else
				super(602, data)
			end
		end
	end

	class LegSecurityIDSource < Quickfix::StringField
		def LegSecurityIDSource.field
			return 603
		end
		def initialize(data = nil)
			if( data == nil )
				super(603)
			else
				super(603, data)
			end
		end
	end

	class NoLegSecurityAltID < Quickfix::IntField
		def NoLegSecurityAltID.field
			return 604
		end
		def initialize(data = nil)
			if( data == nil )
				super(604)
			else
				super(604, data)
			end
		end
	end

	class LegSecurityAltID < Quickfix::StringField
		def LegSecurityAltID.field
			return 605
		end
		def initialize(data = nil)
			if( data == nil )
				super(605)
			else
				super(605, data)
			end
		end
	end

	class LegSecurityAltIDSource < Quickfix::StringField
		def LegSecurityAltIDSource.field
			return 606
		end
		def initialize(data = nil)
			if( data == nil )
				super(606)
			else
				super(606, data)
			end
		end
	end

	class LegProduct < Quickfix::IntField
		def LegProduct.field
			return 607
		end
		def initialize(data = nil)
			if( data == nil )
				super(607)
			else
				super(607, data)
			end
		end
	end

	class LegCFICode < Quickfix::StringField
		def LegCFICode.field
			return 608
		end
		def initialize(data = nil)
			if( data == nil )
				super(608)
			else
				super(608, data)
			end
		end
	end

	class LegSecurityType < Quickfix::StringField
		def LegSecurityType.field
			return 609
		end
		def initialize(data = nil)
			if( data == nil )
				super(609)
			else
				super(609, data)
			end
		end
	end

	class LegMaturityMonthYear < Quickfix::StringField
		def LegMaturityMonthYear.field
			return 610
		end
		def initialize(data = nil)
			if( data == nil )
				super(610)
			else
				super(610, data)
			end
		end
	end

	class LegMaturityDate < Quickfix::StringField
		def LegMaturityDate.field
			return 611
		end
		def initialize(data = nil)
			if( data == nil )
				super(611)
			else
				super(611, data)
			end
		end
	end

	class LegStrikePrice < Quickfix::DoubleField
		def LegStrikePrice.field
			return 612
		end
		def initialize(data = nil)
			if( data == nil )
				super(612)
			else
				super(612, data)
			end
		end
	end

	class LegOptAttribute < Quickfix::CharField
		def LegOptAttribute.field
			return 613
		end
		def initialize(data = nil)
			if( data == nil )
				super(613)
			else
				super(613, data)
			end
		end
	end

	class LegContractMultiplier < Quickfix::DoubleField
		def LegContractMultiplier.field
			return 614
		end
		def initialize(data = nil)
			if( data == nil )
				super(614)
			else
				super(614, data)
			end
		end
	end

	class LegCouponRate < Quickfix::DoubleField
		def LegCouponRate.field
			return 615
		end
		def initialize(data = nil)
			if( data == nil )
				super(615)
			else
				super(615, data)
			end
		end
	end

	class LegSecurityExchange < Quickfix::StringField
		def LegSecurityExchange.field
			return 616
		end
		def initialize(data = nil)
			if( data == nil )
				super(616)
			else
				super(616, data)
			end
		end
	end

	class LegIssuer < Quickfix::StringField
		def LegIssuer.field
			return 617
		end
		def initialize(data = nil)
			if( data == nil )
				super(617)
			else
				super(617, data)
			end
		end
	end

	class EncodedLegIssuerLen < Quickfix::IntField
		def EncodedLegIssuerLen.field
			return 618
		end
		def initialize(data = nil)
			if( data == nil )
				super(618)
			else
				super(618, data)
			end
		end
	end

	class EncodedLegIssuer < Quickfix::StringField
		def EncodedLegIssuer.field
			return 619
		end
		def initialize(data = nil)
			if( data == nil )
				super(619)
			else
				super(619, data)
			end
		end
	end

	class LegSecurityDesc < Quickfix::StringField
		def LegSecurityDesc.field
			return 620
		end
		def initialize(data = nil)
			if( data == nil )
				super(620)
			else
				super(620, data)
			end
		end
	end

	class EncodedLegSecurityDescLen < Quickfix::IntField
		def EncodedLegSecurityDescLen.field
			return 621
		end
		def initialize(data = nil)
			if( data == nil )
				super(621)
			else
				super(621, data)
			end
		end
	end

	class EncodedLegSecurityDesc < Quickfix::StringField
		def EncodedLegSecurityDesc.field
			return 622
		end
		def initialize(data = nil)
			if( data == nil )
				super(622)
			else
				super(622, data)
			end
		end
	end

	class LegRatioQty < Quickfix::DoubleField
		def LegRatioQty.field
			return 623
		end
		def initialize(data = nil)
			if( data == nil )
				super(623)
			else
				super(623, data)
			end
		end
	end

	class LegSide < Quickfix::CharField
		def LegSide.field
			return 624
		end
		def initialize(data = nil)
			if( data == nil )
				super(624)
			else
				super(624, data)
			end
		end
	end

	class TradingSessionSubID < Quickfix::StringField
		def TradingSessionSubID.field
			return 625
		end
		def initialize(data = nil)
			if( data == nil )
				super(625)
			else
				super(625, data)
			end
		end
	end

	class AllocType < Quickfix::IntField
		def AllocType.field
			return 626
		end
		def initialize(data = nil)
			if( data == nil )
				super(626)
			else
				super(626, data)
			end
		end
	end

	class MidPx < Quickfix::DoubleField
		def MidPx.field
			return 631
		end
		def initialize(data = nil)
			if( data == nil )
				super(631)
			else
				super(631, data)
			end
		end
	end

	class BidYield < Quickfix::DoubleField
		def BidYield.field
			return 632
		end
		def initialize(data = nil)
			if( data == nil )
				super(632)
			else
				super(632, data)
			end
		end
	end

	class MidYield < Quickfix::DoubleField
		def MidYield.field
			return 633
		end
		def initialize(data = nil)
			if( data == nil )
				super(633)
			else
				super(633, data)
			end
		end
	end

	class OfferYield < Quickfix::DoubleField
		def OfferYield.field
			return 634
		end
		def initialize(data = nil)
			if( data == nil )
				super(634)
			else
				super(634, data)
			end
		end
	end

	class ClearingFeeIndicator < Quickfix::StringField
		def ClearingFeeIndicator.field
			return 635
		end
		def initialize(data = nil)
			if( data == nil )
				super(635)
			else
				super(635, data)
			end
		end
	end

	class WorkingIndicator < Quickfix::BoolField
		def WorkingIndicator.field
			return 636
		end
		def initialize(data = nil)
			if( data == nil )
				super(636)
			else
				super(636, data)
			end
		end
	end

	class LegLastPx < Quickfix::DoubleField
		def LegLastPx.field
			return 637
		end
		def initialize(data = nil)
			if( data == nil )
				super(637)
			else
				super(637, data)
			end
		end
	end

	class PriorityIndicator < Quickfix::IntField
		def PriorityIndicator.field
			return 638
		end
		def initialize(data = nil)
			if( data == nil )
				super(638)
			else
				super(638, data)
			end
		end
	end

	class PriceImprovement < Quickfix::DoubleField
		def PriceImprovement.field
			return 639
		end
		def initialize(data = nil)
			if( data == nil )
				super(639)
			else
				super(639, data)
			end
		end
	end

	class Price2 < Quickfix::DoubleField
		def Price2.field
			return 640
		end
		def initialize(data = nil)
			if( data == nil )
				super(640)
			else
				super(640, data)
			end
		end
	end

	class LastForwardPoints2 < Quickfix::DoubleField
		def LastForwardPoints2.field
			return 641
		end
		def initialize(data = nil)
			if( data == nil )
				super(641)
			else
				super(641, data)
			end
		end
	end

	class BidForwardPoints2 < Quickfix::DoubleField
		def BidForwardPoints2.field
			return 642
		end
		def initialize(data = nil)
			if( data == nil )
				super(642)
			else
				super(642, data)
			end
		end
	end

	class OfferForwardPoints2 < Quickfix::DoubleField
		def OfferForwardPoints2.field
			return 643
		end
		def initialize(data = nil)
			if( data == nil )
				super(643)
			else
				super(643, data)
			end
		end
	end

	class RFQReqID < Quickfix::StringField
		def RFQReqID.field
			return 644
		end
		def initialize(data = nil)
			if( data == nil )
				super(644)
			else
				super(644, data)
			end
		end
	end

	class MktBidPx < Quickfix::DoubleField
		def MktBidPx.field
			return 645
		end
		def initialize(data = nil)
			if( data == nil )
				super(645)
			else
				super(645, data)
			end
		end
	end

	class MktOfferPx < Quickfix::DoubleField
		def MktOfferPx.field
			return 646
		end
		def initialize(data = nil)
			if( data == nil )
				super(646)
			else
				super(646, data)
			end
		end
	end

	class MinBidSize < Quickfix::DoubleField
		def MinBidSize.field
			return 647
		end
		def initialize(data = nil)
			if( data == nil )
				super(647)
			else
				super(647, data)
			end
		end
	end

	class MinOfferSize < Quickfix::DoubleField
		def MinOfferSize.field
			return 648
		end
		def initialize(data = nil)
			if( data == nil )
				super(648)
			else
				super(648, data)
			end
		end
	end

	class QuoteStatusReqID < Quickfix::StringField
		def QuoteStatusReqID.field
			return 649
		end
		def initialize(data = nil)
			if( data == nil )
				super(649)
			else
				super(649, data)
			end
		end
	end

	class LegalConfirm < Quickfix::BoolField
		def LegalConfirm.field
			return 650
		end
		def initialize(data = nil)
			if( data == nil )
				super(650)
			else
				super(650, data)
			end
		end
	end

	class UnderlyingLastPx < Quickfix::DoubleField
		def UnderlyingLastPx.field
			return 651
		end
		def initialize(data = nil)
			if( data == nil )
				super(651)
			else
				super(651, data)
			end
		end
	end

	class UnderlyingLastQty < Quickfix::DoubleField
		def UnderlyingLastQty.field
			return 652
		end
		def initialize(data = nil)
			if( data == nil )
				super(652)
			else
				super(652, data)
			end
		end
	end

	class LegRefID < Quickfix::StringField
		def LegRefID.field
			return 654
		end
		def initialize(data = nil)
			if( data == nil )
				super(654)
			else
				super(654, data)
			end
		end
	end

	class ContraLegRefID < Quickfix::StringField
		def ContraLegRefID.field
			return 655
		end
		def initialize(data = nil)
			if( data == nil )
				super(655)
			else
				super(655, data)
			end
		end
	end

	class SettlCurrBidFxRate < Quickfix::DoubleField
		def SettlCurrBidFxRate.field
			return 656
		end
		def initialize(data = nil)
			if( data == nil )
				super(656)
			else
				super(656, data)
			end
		end
	end

	class SettlCurrOfferFxRate < Quickfix::DoubleField
		def SettlCurrOfferFxRate.field
			return 657
		end
		def initialize(data = nil)
			if( data == nil )
				super(657)
			else
				super(657, data)
			end
		end
	end

	class QuoteRequestRejectReason < Quickfix::IntField
		def QuoteRequestRejectReason.field
			return 658
		end
		def initialize(data = nil)
			if( data == nil )
				super(658)
			else
				super(658, data)
			end
		end
	end

	class SideComplianceID < Quickfix::StringField
		def SideComplianceID.field
			return 659
		end
		def initialize(data = nil)
			if( data == nil )
				super(659)
			else
				super(659, data)
			end
		end
	end

	class IOIID < Quickfix::StringField
		def IOIID.field
			return 23
		end
		def initialize(data = nil)
			if( data == nil )
				super(23)
			else
				super(23, data)
			end
		end
	end

	class NoLinesOfText < Quickfix::IntField
		def NoLinesOfText.field
			return 33
		end
		def initialize(data = nil)
			if( data == nil )
				super(33)
			else
				super(33, data)
			end
		end
	end

	class SettlType < Quickfix::StringField
		def SettlType.field
			return 63
		end
		def initialize(data = nil)
			if( data == nil )
				super(63)
			else
				super(63, data)
			end
		end
	end

	class SettlDate < Quickfix::StringField
		def SettlDate.field
			return 64
		end
		def initialize(data = nil)
			if( data == nil )
				super(64)
			else
				super(64, data)
			end
		end
	end

	class AvgPxPrecision < Quickfix::IntField
		def AvgPxPrecision.field
			return 74
		end
		def initialize(data = nil)
			if( data == nil )
				super(74)
			else
				super(74, data)
			end
		end
	end

	class SettlDate2 < Quickfix::StringField
		def SettlDate2.field
			return 193
		end
		def initialize(data = nil)
			if( data == nil )
				super(193)
			else
				super(193, data)
			end
		end
	end

	class PegOffsetValue < Quickfix::DoubleField
		def PegOffsetValue.field
			return 211
		end
		def initialize(data = nil)
			if( data == nil )
				super(211)
			else
				super(211, data)
			end
		end
	end

	class OpenCloseSettlFlag < Quickfix::StringField
		def OpenCloseSettlFlag.field
			return 286
		end
		def initialize(data = nil)
			if( data == nil )
				super(286)
			else
				super(286, data)
			end
		end
	end

	class TotNoQuoteEntries < Quickfix::IntField
		def TotNoQuoteEntries.field
			return 304
		end
		def initialize(data = nil)
			if( data == nil )
				super(304)
			else
				super(304, data)
			end
		end
	end

	class DiscretionOffsetValue < Quickfix::DoubleField
		def DiscretionOffsetValue.field
			return 389
		end
		def initialize(data = nil)
			if( data == nil )
				super(389)
			else
				super(389, data)
			end
		end
	end

	class TotNoRelatedSym < Quickfix::IntField
		def TotNoRelatedSym.field
			return 393
		end
		def initialize(data = nil)
			if( data == nil )
				super(393)
			else
				super(393, data)
			end
		end
	end

	class BidTradeType < Quickfix::CharField
		def BidTradeType.field
			return 418
		end
		def initialize(data = nil)
			if( data == nil )
				super(418)
			else
				super(418, data)
			end
		end
	end

	class CardIssNum < Quickfix::StringField
		def CardIssNum.field
			return 491
		end
		def initialize(data = nil)
			if( data == nil )
				super(491)
			else
				super(491, data)
			end
		end
	end

	class CashDistribAgentAcctName < Quickfix::StringField
		def CashDistribAgentAcctName.field
			return 502
		end
		def initialize(data = nil)
			if( data == nil )
				super(502)
			else
				super(502, data)
			end
		end
	end

	class RegistDtls < Quickfix::StringField
		def RegistDtls.field
			return 509
		end
		def initialize(data = nil)
			if( data == nil )
				super(509)
			else
				super(509, data)
			end
		end
	end

	class TotNoSecurityTypes < Quickfix::IntField
		def TotNoSecurityTypes.field
			return 557
		end
		def initialize(data = nil)
			if( data == nil )
				super(557)
			else
				super(557, data)
			end
		end
	end

	class LegSettlType < Quickfix::StringField
		def LegSettlType.field
			return 587
		end
		def initialize(data = nil)
			if( data == nil )
				super(587)
			else
				super(587, data)
			end
		end
	end

	class LegSettlDate < Quickfix::StringField
		def LegSettlDate.field
			return 588
		end
		def initialize(data = nil)
			if( data == nil )
				super(588)
			else
				super(588, data)
			end
		end
	end

	class AcctIDSource < Quickfix::IntField
		def AcctIDSource.field
			return 660
		end
		def initialize(data = nil)
			if( data == nil )
				super(660)
			else
				super(660, data)
			end
		end
	end

	class AllocAcctIDSource < Quickfix::IntField
		def AllocAcctIDSource.field
			return 661
		end
		def initialize(data = nil)
			if( data == nil )
				super(661)
			else
				super(661, data)
			end
		end
	end

	class BenchmarkPrice < Quickfix::DoubleField
		def BenchmarkPrice.field
			return 662
		end
		def initialize(data = nil)
			if( data == nil )
				super(662)
			else
				super(662, data)
			end
		end
	end

	class BenchmarkPriceType < Quickfix::IntField
		def BenchmarkPriceType.field
			return 663
		end
		def initialize(data = nil)
			if( data == nil )
				super(663)
			else
				super(663, data)
			end
		end
	end

	class ConfirmID < Quickfix::StringField
		def ConfirmID.field
			return 664
		end
		def initialize(data = nil)
			if( data == nil )
				super(664)
			else
				super(664, data)
			end
		end
	end

	class ConfirmStatus < Quickfix::IntField
		def ConfirmStatus.field
			return 665
		end
		def initialize(data = nil)
			if( data == nil )
				super(665)
			else
				super(665, data)
			end
		end
	end

	class ConfirmTransType < Quickfix::IntField
		def ConfirmTransType.field
			return 666
		end
		def initialize(data = nil)
			if( data == nil )
				super(666)
			else
				super(666, data)
			end
		end
	end

	class ContractSettlMonth < Quickfix::StringField
		def ContractSettlMonth.field
			return 667
		end
		def initialize(data = nil)
			if( data == nil )
				super(667)
			else
				super(667, data)
			end
		end
	end

	class DeliveryForm < Quickfix::IntField
		def DeliveryForm.field
			return 668
		end
		def initialize(data = nil)
			if( data == nil )
				super(668)
			else
				super(668, data)
			end
		end
	end

	class LastParPx < Quickfix::DoubleField
		def LastParPx.field
			return 669
		end
		def initialize(data = nil)
			if( data == nil )
				super(669)
			else
				super(669, data)
			end
		end
	end

	class NoLegAllocs < Quickfix::IntField
		def NoLegAllocs.field
			return 670
		end
		def initialize(data = nil)
			if( data == nil )
				super(670)
			else
				super(670, data)
			end
		end
	end

	class LegAllocAccount < Quickfix::StringField
		def LegAllocAccount.field
			return 671
		end
		def initialize(data = nil)
			if( data == nil )
				super(671)
			else
				super(671, data)
			end
		end
	end

	class LegIndividualAllocID < Quickfix::StringField
		def LegIndividualAllocID.field
			return 672
		end
		def initialize(data = nil)
			if( data == nil )
				super(672)
			else
				super(672, data)
			end
		end
	end

	class LegAllocQty < Quickfix::DoubleField
		def LegAllocQty.field
			return 673
		end
		def initialize(data = nil)
			if( data == nil )
				super(673)
			else
				super(673, data)
			end
		end
	end

	class LegAllocAcctIDSource < Quickfix::IntField
		def LegAllocAcctIDSource.field
			return 674
		end
		def initialize(data = nil)
			if( data == nil )
				super(674)
			else
				super(674, data)
			end
		end
	end

	class LegSettlCurrency < Quickfix::StringField
		def LegSettlCurrency.field
			return 675
		end
		def initialize(data = nil)
			if( data == nil )
				super(675)
			else
				super(675, data)
			end
		end
	end

	class LegBenchmarkCurveCurrency < Quickfix::StringField
		def LegBenchmarkCurveCurrency.field
			return 676
		end
		def initialize(data = nil)
			if( data == nil )
				super(676)
			else
				super(676, data)
			end
		end
	end

	class LegBenchmarkCurveName < Quickfix::StringField
		def LegBenchmarkCurveName.field
			return 677
		end
		def initialize(data = nil)
			if( data == nil )
				super(677)
			else
				super(677, data)
			end
		end
	end

	class LegBenchmarkCurvePoint < Quickfix::StringField
		def LegBenchmarkCurvePoint.field
			return 678
		end
		def initialize(data = nil)
			if( data == nil )
				super(678)
			else
				super(678, data)
			end
		end
	end

	class LegBenchmarkPrice < Quickfix::DoubleField
		def LegBenchmarkPrice.field
			return 679
		end
		def initialize(data = nil)
			if( data == nil )
				super(679)
			else
				super(679, data)
			end
		end
	end

	class LegBenchmarkPriceType < Quickfix::IntField
		def LegBenchmarkPriceType.field
			return 680
		end
		def initialize(data = nil)
			if( data == nil )
				super(680)
			else
				super(680, data)
			end
		end
	end

	class LegBidPx < Quickfix::DoubleField
		def LegBidPx.field
			return 681
		end
		def initialize(data = nil)
			if( data == nil )
				super(681)
			else
				super(681, data)
			end
		end
	end

	class LegIOIQty < Quickfix::StringField
		def LegIOIQty.field
			return 682
		end
		def initialize(data = nil)
			if( data == nil )
				super(682)
			else
				super(682, data)
			end
		end
	end

	class NoLegStipulations < Quickfix::IntField
		def NoLegStipulations.field
			return 683
		end
		def initialize(data = nil)
			if( data == nil )
				super(683)
			else
				super(683, data)
			end
		end
	end

	class LegOfferPx < Quickfix::DoubleField
		def LegOfferPx.field
			return 684
		end
		def initialize(data = nil)
			if( data == nil )
				super(684)
			else
				super(684, data)
			end
		end
	end

	class LegPriceType < Quickfix::IntField
		def LegPriceType.field
			return 686
		end
		def initialize(data = nil)
			if( data == nil )
				super(686)
			else
				super(686, data)
			end
		end
	end

	class LegQty < Quickfix::DoubleField
		def LegQty.field
			return 687
		end
		def initialize(data = nil)
			if( data == nil )
				super(687)
			else
				super(687, data)
			end
		end
	end

	class LegStipulationType < Quickfix::StringField
		def LegStipulationType.field
			return 688
		end
		def initialize(data = nil)
			if( data == nil )
				super(688)
			else
				super(688, data)
			end
		end
	end

	class LegStipulationValue < Quickfix::StringField
		def LegStipulationValue.field
			return 689
		end
		def initialize(data = nil)
			if( data == nil )
				super(689)
			else
				super(689, data)
			end
		end
	end

	class LegSwapType < Quickfix::IntField
		def LegSwapType.field
			return 690
		end
		def initialize(data = nil)
			if( data == nil )
				super(690)
			else
				super(690, data)
			end
		end
	end

	class Pool < Quickfix::StringField
		def Pool.field
			return 691
		end
		def initialize(data = nil)
			if( data == nil )
				super(691)
			else
				super(691, data)
			end
		end
	end

	class QuotePriceType < Quickfix::IntField
		def QuotePriceType.field
			return 692
		end
		def initialize(data = nil)
			if( data == nil )
				super(692)
			else
				super(692, data)
			end
		end
	end

	class QuoteRespID < Quickfix::StringField
		def QuoteRespID.field
			return 693
		end
		def initialize(data = nil)
			if( data == nil )
				super(693)
			else
				super(693, data)
			end
		end
	end

	class QuoteRespType < Quickfix::IntField
		def QuoteRespType.field
			return 694
		end
		def initialize(data = nil)
			if( data == nil )
				super(694)
			else
				super(694, data)
			end
		end
	end

	class QuoteQualifier < Quickfix::CharField
		def QuoteQualifier.field
			return 695
		end
		def initialize(data = nil)
			if( data == nil )
				super(695)
			else
				super(695, data)
			end
		end
	end

	class YieldRedemptionDate < Quickfix::StringField
		def YieldRedemptionDate.field
			return 696
		end
		def initialize(data = nil)
			if( data == nil )
				super(696)
			else
				super(696, data)
			end
		end
	end

	class YieldRedemptionPrice < Quickfix::DoubleField
		def YieldRedemptionPrice.field
			return 697
		end
		def initialize(data = nil)
			if( data == nil )
				super(697)
			else
				super(697, data)
			end
		end
	end

	class YieldRedemptionPriceType < Quickfix::IntField
		def YieldRedemptionPriceType.field
			return 698
		end
		def initialize(data = nil)
			if( data == nil )
				super(698)
			else
				super(698, data)
			end
		end
	end

	class BenchmarkSecurityID < Quickfix::StringField
		def BenchmarkSecurityID.field
			return 699
		end
		def initialize(data = nil)
			if( data == nil )
				super(699)
			else
				super(699, data)
			end
		end
	end

	class ReversalIndicator < Quickfix::BoolField
		def ReversalIndicator.field
			return 700
		end
		def initialize(data = nil)
			if( data == nil )
				super(700)
			else
				super(700, data)
			end
		end
	end

	class YieldCalcDate < Quickfix::StringField
		def YieldCalcDate.field
			return 701
		end
		def initialize(data = nil)
			if( data == nil )
				super(701)
			else
				super(701, data)
			end
		end
	end

	class NoPositions < Quickfix::IntField
		def NoPositions.field
			return 702
		end
		def initialize(data = nil)
			if( data == nil )
				super(702)
			else
				super(702, data)
			end
		end
	end

	class PosType < Quickfix::StringField
		def PosType.field
			return 703
		end
		def initialize(data = nil)
			if( data == nil )
				super(703)
			else
				super(703, data)
			end
		end
	end

	class LongQty < Quickfix::DoubleField
		def LongQty.field
			return 704
		end
		def initialize(data = nil)
			if( data == nil )
				super(704)
			else
				super(704, data)
			end
		end
	end

	class ShortQty < Quickfix::DoubleField
		def ShortQty.field
			return 705
		end
		def initialize(data = nil)
			if( data == nil )
				super(705)
			else
				super(705, data)
			end
		end
	end

	class PosQtyStatus < Quickfix::IntField
		def PosQtyStatus.field
			return 706
		end
		def initialize(data = nil)
			if( data == nil )
				super(706)
			else
				super(706, data)
			end
		end
	end

	class PosAmtType < Quickfix::StringField
		def PosAmtType.field
			return 707
		end
		def initialize(data = nil)
			if( data == nil )
				super(707)
			else
				super(707, data)
			end
		end
	end

	class PosAmt < Quickfix::DoubleField
		def PosAmt.field
			return 708
		end
		def initialize(data = nil)
			if( data == nil )
				super(708)
			else
				super(708, data)
			end
		end
	end

	class PosTransType < Quickfix::IntField
		def PosTransType.field
			return 709
		end
		def initialize(data = nil)
			if( data == nil )
				super(709)
			else
				super(709, data)
			end
		end
	end

	class PosReqID < Quickfix::StringField
		def PosReqID.field
			return 710
		end
		def initialize(data = nil)
			if( data == nil )
				super(710)
			else
				super(710, data)
			end
		end
	end

	class NoUnderlyings < Quickfix::IntField
		def NoUnderlyings.field
			return 711
		end
		def initialize(data = nil)
			if( data == nil )
				super(711)
			else
				super(711, data)
			end
		end
	end

	class PosMaintAction < Quickfix::IntField
		def PosMaintAction.field
			return 712
		end
		def initialize(data = nil)
			if( data == nil )
				super(712)
			else
				super(712, data)
			end
		end
	end

	class OrigPosReqRefID < Quickfix::StringField
		def OrigPosReqRefID.field
			return 713
		end
		def initialize(data = nil)
			if( data == nil )
				super(713)
			else
				super(713, data)
			end
		end
	end

	class PosMaintRptRefID < Quickfix::StringField
		def PosMaintRptRefID.field
			return 714
		end
		def initialize(data = nil)
			if( data == nil )
				super(714)
			else
				super(714, data)
			end
		end
	end

	class ClearingBusinessDate < Quickfix::StringField
		def ClearingBusinessDate.field
			return 715
		end
		def initialize(data = nil)
			if( data == nil )
				super(715)
			else
				super(715, data)
			end
		end
	end

	class SettlSessID < Quickfix::StringField
		def SettlSessID.field
			return 716
		end
		def initialize(data = nil)
			if( data == nil )
				super(716)
			else
				super(716, data)
			end
		end
	end

	class SettlSessSubID < Quickfix::StringField
		def SettlSessSubID.field
			return 717
		end
		def initialize(data = nil)
			if( data == nil )
				super(717)
			else
				super(717, data)
			end
		end
	end

	class AdjustmentType < Quickfix::IntField
		def AdjustmentType.field
			return 718
		end
		def initialize(data = nil)
			if( data == nil )
				super(718)
			else
				super(718, data)
			end
		end
	end

	class ContraryInstructionIndicator < Quickfix::BoolField
		def ContraryInstructionIndicator.field
			return 719
		end
		def initialize(data = nil)
			if( data == nil )
				super(719)
			else
				super(719, data)
			end
		end
	end

	class PriorSpreadIndicator < Quickfix::BoolField
		def PriorSpreadIndicator.field
			return 720
		end
		def initialize(data = nil)
			if( data == nil )
				super(720)
			else
				super(720, data)
			end
		end
	end

	class PosMaintRptID < Quickfix::StringField
		def PosMaintRptID.field
			return 721
		end
		def initialize(data = nil)
			if( data == nil )
				super(721)
			else
				super(721, data)
			end
		end
	end

	class PosMaintStatus < Quickfix::IntField
		def PosMaintStatus.field
			return 722
		end
		def initialize(data = nil)
			if( data == nil )
				super(722)
			else
				super(722, data)
			end
		end
	end

	class PosMaintResult < Quickfix::IntField
		def PosMaintResult.field
			return 723
		end
		def initialize(data = nil)
			if( data == nil )
				super(723)
			else
				super(723, data)
			end
		end
	end

	class PosReqType < Quickfix::IntField
		def PosReqType.field
			return 724
		end
		def initialize(data = nil)
			if( data == nil )
				super(724)
			else
				super(724, data)
			end
		end
	end

	class ResponseTransportType < Quickfix::IntField
		def ResponseTransportType.field
			return 725
		end
		def initialize(data = nil)
			if( data == nil )
				super(725)
			else
				super(725, data)
			end
		end
	end

	class ResponseDestination < Quickfix::StringField
		def ResponseDestination.field
			return 726
		end
		def initialize(data = nil)
			if( data == nil )
				super(726)
			else
				super(726, data)
			end
		end
	end

	class TotalNumPosReports < Quickfix::IntField
		def TotalNumPosReports.field
			return 727
		end
		def initialize(data = nil)
			if( data == nil )
				super(727)
			else
				super(727, data)
			end
		end
	end

	class PosReqResult < Quickfix::IntField
		def PosReqResult.field
			return 728
		end
		def initialize(data = nil)
			if( data == nil )
				super(728)
			else
				super(728, data)
			end
		end
	end

	class PosReqStatus < Quickfix::IntField
		def PosReqStatus.field
			return 729
		end
		def initialize(data = nil)
			if( data == nil )
				super(729)
			else
				super(729, data)
			end
		end
	end

	class SettlPrice < Quickfix::DoubleField
		def SettlPrice.field
			return 730
		end
		def initialize(data = nil)
			if( data == nil )
				super(730)
			else
				super(730, data)
			end
		end
	end

	class SettlPriceType < Quickfix::IntField
		def SettlPriceType.field
			return 731
		end
		def initialize(data = nil)
			if( data == nil )
				super(731)
			else
				super(731, data)
			end
		end
	end

	class UnderlyingSettlPrice < Quickfix::DoubleField
		def UnderlyingSettlPrice.field
			return 732
		end
		def initialize(data = nil)
			if( data == nil )
				super(732)
			else
				super(732, data)
			end
		end
	end

	class UnderlyingSettlPriceType < Quickfix::IntField
		def UnderlyingSettlPriceType.field
			return 733
		end
		def initialize(data = nil)
			if( data == nil )
				super(733)
			else
				super(733, data)
			end
		end
	end

	class PriorSettlPrice < Quickfix::DoubleField
		def PriorSettlPrice.field
			return 734
		end
		def initialize(data = nil)
			if( data == nil )
				super(734)
			else
				super(734, data)
			end
		end
	end

	class NoQuoteQualifiers < Quickfix::IntField
		def NoQuoteQualifiers.field
			return 735
		end
		def initialize(data = nil)
			if( data == nil )
				super(735)
			else
				super(735, data)
			end
		end
	end

	class AllocSettlCurrency < Quickfix::StringField
		def AllocSettlCurrency.field
			return 736
		end
		def initialize(data = nil)
			if( data == nil )
				super(736)
			else
				super(736, data)
			end
		end
	end

	class AllocSettlCurrAmt < Quickfix::DoubleField
		def AllocSettlCurrAmt.field
			return 737
		end
		def initialize(data = nil)
			if( data == nil )
				super(737)
			else
				super(737, data)
			end
		end
	end

	class InterestAtMaturity < Quickfix::DoubleField
		def InterestAtMaturity.field
			return 738
		end
		def initialize(data = nil)
			if( data == nil )
				super(738)
			else
				super(738, data)
			end
		end
	end

	class LegDatedDate < Quickfix::StringField
		def LegDatedDate.field
			return 739
		end
		def initialize(data = nil)
			if( data == nil )
				super(739)
			else
				super(739, data)
			end
		end
	end

	class LegPool < Quickfix::StringField
		def LegPool.field
			return 740
		end
		def initialize(data = nil)
			if( data == nil )
				super(740)
			else
				super(740, data)
			end
		end
	end

	class AllocInterestAtMaturity < Quickfix::DoubleField
		def AllocInterestAtMaturity.field
			return 741
		end
		def initialize(data = nil)
			if( data == nil )
				super(741)
			else
				super(741, data)
			end
		end
	end

	class AllocAccruedInterestAmt < Quickfix::DoubleField
		def AllocAccruedInterestAmt.field
			return 742
		end
		def initialize(data = nil)
			if( data == nil )
				super(742)
			else
				super(742, data)
			end
		end
	end

	class DeliveryDate < Quickfix::StringField
		def DeliveryDate.field
			return 743
		end
		def initialize(data = nil)
			if( data == nil )
				super(743)
			else
				super(743, data)
			end
		end
	end

	class AssignmentMethod < Quickfix::CharField
		def AssignmentMethod.field
			return 744
		end
		def initialize(data = nil)
			if( data == nil )
				super(744)
			else
				super(744, data)
			end
		end
	end

	class AssignmentUnit < Quickfix::DoubleField
		def AssignmentUnit.field
			return 745
		end
		def initialize(data = nil)
			if( data == nil )
				super(745)
			else
				super(745, data)
			end
		end
	end

	class OpenInterest < Quickfix::DoubleField
		def OpenInterest.field
			return 746
		end
		def initialize(data = nil)
			if( data == nil )
				super(746)
			else
				super(746, data)
			end
		end
	end

	class ExerciseMethod < Quickfix::CharField
		def ExerciseMethod.field
			return 747
		end
		def initialize(data = nil)
			if( data == nil )
				super(747)
			else
				super(747, data)
			end
		end
	end

	class TotNumTradeReports < Quickfix::IntField
		def TotNumTradeReports.field
			return 748
		end
		def initialize(data = nil)
			if( data == nil )
				super(748)
			else
				super(748, data)
			end
		end
	end

	class TradeRequestResult < Quickfix::IntField
		def TradeRequestResult.field
			return 749
		end
		def initialize(data = nil)
			if( data == nil )
				super(749)
			else
				super(749, data)
			end
		end
	end

	class TradeRequestStatus < Quickfix::IntField
		def TradeRequestStatus.field
			return 750
		end
		def initialize(data = nil)
			if( data == nil )
				super(750)
			else
				super(750, data)
			end
		end
	end

	class TradeReportRejectReason < Quickfix::IntField
		def TradeReportRejectReason.field
			return 751
		end
		def initialize(data = nil)
			if( data == nil )
				super(751)
			else
				super(751, data)
			end
		end
	end

	class SideMultiLegReportingType < Quickfix::IntField
		def SideMultiLegReportingType.field
			return 752
		end
		def initialize(data = nil)
			if( data == nil )
				super(752)
			else
				super(752, data)
			end
		end
	end

	class NoPosAmt < Quickfix::IntField
		def NoPosAmt.field
			return 753
		end
		def initialize(data = nil)
			if( data == nil )
				super(753)
			else
				super(753, data)
			end
		end
	end

	class AutoAcceptIndicator < Quickfix::BoolField
		def AutoAcceptIndicator.field
			return 754
		end
		def initialize(data = nil)
			if( data == nil )
				super(754)
			else
				super(754, data)
			end
		end
	end

	class AllocReportID < Quickfix::StringField
		def AllocReportID.field
			return 755
		end
		def initialize(data = nil)
			if( data == nil )
				super(755)
			else
				super(755, data)
			end
		end
	end

	class NoNested2PartyIDs < Quickfix::IntField
		def NoNested2PartyIDs.field
			return 756
		end
		def initialize(data = nil)
			if( data == nil )
				super(756)
			else
				super(756, data)
			end
		end
	end

	class Nested2PartyID < Quickfix::StringField
		def Nested2PartyID.field
			return 757
		end
		def initialize(data = nil)
			if( data == nil )
				super(757)
			else
				super(757, data)
			end
		end
	end

	class Nested2PartyIDSource < Quickfix::CharField
		def Nested2PartyIDSource.field
			return 758
		end
		def initialize(data = nil)
			if( data == nil )
				super(758)
			else
				super(758, data)
			end
		end
	end

	class Nested2PartyRole < Quickfix::IntField
		def Nested2PartyRole.field
			return 759
		end
		def initialize(data = nil)
			if( data == nil )
				super(759)
			else
				super(759, data)
			end
		end
	end

	class Nested2PartySubID < Quickfix::StringField
		def Nested2PartySubID.field
			return 760
		end
		def initialize(data = nil)
			if( data == nil )
				super(760)
			else
				super(760, data)
			end
		end
	end

	class BenchmarkSecurityIDSource < Quickfix::StringField
		def BenchmarkSecurityIDSource.field
			return 761
		end
		def initialize(data = nil)
			if( data == nil )
				super(761)
			else
				super(761, data)
			end
		end
	end

	class SecuritySubType < Quickfix::StringField
		def SecuritySubType.field
			return 762
		end
		def initialize(data = nil)
			if( data == nil )
				super(762)
			else
				super(762, data)
			end
		end
	end

	class UnderlyingSecuritySubType < Quickfix::StringField
		def UnderlyingSecuritySubType.field
			return 763
		end
		def initialize(data = nil)
			if( data == nil )
				super(763)
			else
				super(763, data)
			end
		end
	end

	class LegSecuritySubType < Quickfix::StringField
		def LegSecuritySubType.field
			return 764
		end
		def initialize(data = nil)
			if( data == nil )
				super(764)
			else
				super(764, data)
			end
		end
	end

	class AllowableOneSidednessPct < Quickfix::DoubleField
		def AllowableOneSidednessPct.field
			return 765
		end
		def initialize(data = nil)
			if( data == nil )
				super(765)
			else
				super(765, data)
			end
		end
	end

	class AllowableOneSidednessValue < Quickfix::DoubleField
		def AllowableOneSidednessValue.field
			return 766
		end
		def initialize(data = nil)
			if( data == nil )
				super(766)
			else
				super(766, data)
			end
		end
	end

	class AllowableOneSidednessCurr < Quickfix::StringField
		def AllowableOneSidednessCurr.field
			return 767
		end
		def initialize(data = nil)
			if( data == nil )
				super(767)
			else
				super(767, data)
			end
		end
	end

	class NoTrdRegTimestamps < Quickfix::IntField
		def NoTrdRegTimestamps.field
			return 768
		end
		def initialize(data = nil)
			if( data == nil )
				super(768)
			else
				super(768, data)
			end
		end
	end

	class TrdRegTimestamp < Quickfix::UtcTimeStampField
		def TrdRegTimestamp.field
			return 769
		end
		def initialize(data = nil)
			if( data == nil )
				super(769)
			else
				super(769, data)
			end
		end
	end

	class TrdRegTimestampType < Quickfix::IntField
		def TrdRegTimestampType.field
			return 770
		end
		def initialize(data = nil)
			if( data == nil )
				super(770)
			else
				super(770, data)
			end
		end
	end

	class TrdRegTimestampOrigin < Quickfix::StringField
		def TrdRegTimestampOrigin.field
			return 771
		end
		def initialize(data = nil)
			if( data == nil )
				super(771)
			else
				super(771, data)
			end
		end
	end

	class ConfirmRefID < Quickfix::StringField
		def ConfirmRefID.field
			return 772
		end
		def initialize(data = nil)
			if( data == nil )
				super(772)
			else
				super(772, data)
			end
		end
	end

	class ConfirmType < Quickfix::IntField
		def ConfirmType.field
			return 773
		end
		def initialize(data = nil)
			if( data == nil )
				super(773)
			else
				super(773, data)
			end
		end
	end

	class ConfirmRejReason < Quickfix::IntField
		def ConfirmRejReason.field
			return 774
		end
		def initialize(data = nil)
			if( data == nil )
				super(774)
			else
				super(774, data)
			end
		end
	end

	class BookingType < Quickfix::IntField
		def BookingType.field
			return 775
		end
		def initialize(data = nil)
			if( data == nil )
				super(775)
			else
				super(775, data)
			end
		end
	end

	class IndividualAllocRejCode < Quickfix::IntField
		def IndividualAllocRejCode.field
			return 776
		end
		def initialize(data = nil)
			if( data == nil )
				super(776)
			else
				super(776, data)
			end
		end
	end

	class SettlInstMsgID < Quickfix::StringField
		def SettlInstMsgID.field
			return 777
		end
		def initialize(data = nil)
			if( data == nil )
				super(777)
			else
				super(777, data)
			end
		end
	end

	class NoSettlInst < Quickfix::IntField
		def NoSettlInst.field
			return 778
		end
		def initialize(data = nil)
			if( data == nil )
				super(778)
			else
				super(778, data)
			end
		end
	end

	class LastUpdateTime < Quickfix::UtcTimeStampField
		def LastUpdateTime.field
			return 779
		end
		def initialize(data = nil)
			if( data == nil )
				super(779)
			else
				super(779, data)
			end
		end
	end

	class AllocSettlInstType < Quickfix::IntField
		def AllocSettlInstType.field
			return 780
		end
		def initialize(data = nil)
			if( data == nil )
				super(780)
			else
				super(780, data)
			end
		end
	end

	class NoSettlPartyIDs < Quickfix::IntField
		def NoSettlPartyIDs.field
			return 781
		end
		def initialize(data = nil)
			if( data == nil )
				super(781)
			else
				super(781, data)
			end
		end
	end

	class SettlPartyID < Quickfix::StringField
		def SettlPartyID.field
			return 782
		end
		def initialize(data = nil)
			if( data == nil )
				super(782)
			else
				super(782, data)
			end
		end
	end

	class SettlPartyIDSource < Quickfix::CharField
		def SettlPartyIDSource.field
			return 783
		end
		def initialize(data = nil)
			if( data == nil )
				super(783)
			else
				super(783, data)
			end
		end
	end

	class SettlPartyRole < Quickfix::IntField
		def SettlPartyRole.field
			return 784
		end
		def initialize(data = nil)
			if( data == nil )
				super(784)
			else
				super(784, data)
			end
		end
	end

	class SettlPartySubID < Quickfix::StringField
		def SettlPartySubID.field
			return 785
		end
		def initialize(data = nil)
			if( data == nil )
				super(785)
			else
				super(785, data)
			end
		end
	end

	class SettlPartySubIDType < Quickfix::IntField
		def SettlPartySubIDType.field
			return 786
		end
		def initialize(data = nil)
			if( data == nil )
				super(786)
			else
				super(786, data)
			end
		end
	end

	class DlvyInstType < Quickfix::CharField
		def DlvyInstType.field
			return 787
		end
		def initialize(data = nil)
			if( data == nil )
				super(787)
			else
				super(787, data)
			end
		end
	end

	class TerminationType < Quickfix::IntField
		def TerminationType.field
			return 788
		end
		def initialize(data = nil)
			if( data == nil )
				super(788)
			else
				super(788, data)
			end
		end
	end

	class OrdStatusReqID < Quickfix::StringField
		def OrdStatusReqID.field
			return 790
		end
		def initialize(data = nil)
			if( data == nil )
				super(790)
			else
				super(790, data)
			end
		end
	end

	class SettlInstReqID < Quickfix::StringField
		def SettlInstReqID.field
			return 791
		end
		def initialize(data = nil)
			if( data == nil )
				super(791)
			else
				super(791, data)
			end
		end
	end

	class SettlInstReqRejCode < Quickfix::IntField
		def SettlInstReqRejCode.field
			return 792
		end
		def initialize(data = nil)
			if( data == nil )
				super(792)
			else
				super(792, data)
			end
		end
	end

	class SecondaryAllocID < Quickfix::StringField
		def SecondaryAllocID.field
			return 793
		end
		def initialize(data = nil)
			if( data == nil )
				super(793)
			else
				super(793, data)
			end
		end
	end

	class AllocReportType < Quickfix::IntField
		def AllocReportType.field
			return 794
		end
		def initialize(data = nil)
			if( data == nil )
				super(794)
			else
				super(794, data)
			end
		end
	end

	class AllocReportRefID < Quickfix::StringField
		def AllocReportRefID.field
			return 795
		end
		def initialize(data = nil)
			if( data == nil )
				super(795)
			else
				super(795, data)
			end
		end
	end

	class AllocCancReplaceReason < Quickfix::IntField
		def AllocCancReplaceReason.field
			return 796
		end
		def initialize(data = nil)
			if( data == nil )
				super(796)
			else
				super(796, data)
			end
		end
	end

	class CopyMsgIndicator < Quickfix::BoolField
		def CopyMsgIndicator.field
			return 797
		end
		def initialize(data = nil)
			if( data == nil )
				super(797)
			else
				super(797, data)
			end
		end
	end

	class AllocAccountType < Quickfix::IntField
		def AllocAccountType.field
			return 798
		end
		def initialize(data = nil)
			if( data == nil )
				super(798)
			else
				super(798, data)
			end
		end
	end

	class OrderAvgPx < Quickfix::DoubleField
		def OrderAvgPx.field
			return 799
		end
		def initialize(data = nil)
			if( data == nil )
				super(799)
			else
				super(799, data)
			end
		end
	end

	class OrderBookingQty < Quickfix::DoubleField
		def OrderBookingQty.field
			return 800
		end
		def initialize(data = nil)
			if( data == nil )
				super(800)
			else
				super(800, data)
			end
		end
	end

	class NoSettlPartySubIDs < Quickfix::IntField
		def NoSettlPartySubIDs.field
			return 801
		end
		def initialize(data = nil)
			if( data == nil )
				super(801)
			else
				super(801, data)
			end
		end
	end

	class NoPartySubIDs < Quickfix::IntField
		def NoPartySubIDs.field
			return 802
		end
		def initialize(data = nil)
			if( data == nil )
				super(802)
			else
				super(802, data)
			end
		end
	end

	class PartySubIDType < Quickfix::IntField
		def PartySubIDType.field
			return 803
		end
		def initialize(data = nil)
			if( data == nil )
				super(803)
			else
				super(803, data)
			end
		end
	end

	class NoNestedPartySubIDs < Quickfix::IntField
		def NoNestedPartySubIDs.field
			return 804
		end
		def initialize(data = nil)
			if( data == nil )
				super(804)
			else
				super(804, data)
			end
		end
	end

	class NestedPartySubIDType < Quickfix::IntField
		def NestedPartySubIDType.field
			return 805
		end
		def initialize(data = nil)
			if( data == nil )
				super(805)
			else
				super(805, data)
			end
		end
	end

	class NoNested2PartySubIDs < Quickfix::IntField
		def NoNested2PartySubIDs.field
			return 806
		end
		def initialize(data = nil)
			if( data == nil )
				super(806)
			else
				super(806, data)
			end
		end
	end

	class Nested2PartySubIDType < Quickfix::IntField
		def Nested2PartySubIDType.field
			return 807
		end
		def initialize(data = nil)
			if( data == nil )
				super(807)
			else
				super(807, data)
			end
		end
	end

	class AllocIntermedReqType < Quickfix::IntField
		def AllocIntermedReqType.field
			return 808
		end
		def initialize(data = nil)
			if( data == nil )
				super(808)
			else
				super(808, data)
			end
		end
	end

	class UnderlyingPx < Quickfix::DoubleField
		def UnderlyingPx.field
			return 810
		end
		def initialize(data = nil)
			if( data == nil )
				super(810)
			else
				super(810, data)
			end
		end
	end

	class PriceDelta < Quickfix::DoubleField
		def PriceDelta.field
			return 811
		end
		def initialize(data = nil)
			if( data == nil )
				super(811)
			else
				super(811, data)
			end
		end
	end

	class ApplQueueMax < Quickfix::IntField
		def ApplQueueMax.field
			return 812
		end
		def initialize(data = nil)
			if( data == nil )
				super(812)
			else
				super(812, data)
			end
		end
	end

	class ApplQueueDepth < Quickfix::IntField
		def ApplQueueDepth.field
			return 813
		end
		def initialize(data = nil)
			if( data == nil )
				super(813)
			else
				super(813, data)
			end
		end
	end

	class ApplQueueResolution < Quickfix::IntField
		def ApplQueueResolution.field
			return 814
		end
		def initialize(data = nil)
			if( data == nil )
				super(814)
			else
				super(814, data)
			end
		end
	end

	class ApplQueueAction < Quickfix::IntField
		def ApplQueueAction.field
			return 815
		end
		def initialize(data = nil)
			if( data == nil )
				super(815)
			else
				super(815, data)
			end
		end
	end

	class NoAltMDSource < Quickfix::IntField
		def NoAltMDSource.field
			return 816
		end
		def initialize(data = nil)
			if( data == nil )
				super(816)
			else
				super(816, data)
			end
		end
	end

	class AltMDSourceID < Quickfix::StringField
		def AltMDSourceID.field
			return 817
		end
		def initialize(data = nil)
			if( data == nil )
				super(817)
			else
				super(817, data)
			end
		end
	end

	class SecondaryTradeReportID < Quickfix::StringField
		def SecondaryTradeReportID.field
			return 818
		end
		def initialize(data = nil)
			if( data == nil )
				super(818)
			else
				super(818, data)
			end
		end
	end

	class AvgPxIndicator < Quickfix::IntField
		def AvgPxIndicator.field
			return 819
		end
		def initialize(data = nil)
			if( data == nil )
				super(819)
			else
				super(819, data)
			end
		end
	end

	class TradeLinkID < Quickfix::StringField
		def TradeLinkID.field
			return 820
		end
		def initialize(data = nil)
			if( data == nil )
				super(820)
			else
				super(820, data)
			end
		end
	end

	class OrderInputDevice < Quickfix::StringField
		def OrderInputDevice.field
			return 821
		end
		def initialize(data = nil)
			if( data == nil )
				super(821)
			else
				super(821, data)
			end
		end
	end

	class UnderlyingTradingSessionID < Quickfix::StringField
		def UnderlyingTradingSessionID.field
			return 822
		end
		def initialize(data = nil)
			if( data == nil )
				super(822)
			else
				super(822, data)
			end
		end
	end

	class UnderlyingTradingSessionSubID < Quickfix::StringField
		def UnderlyingTradingSessionSubID.field
			return 823
		end
		def initialize(data = nil)
			if( data == nil )
				super(823)
			else
				super(823, data)
			end
		end
	end

	class TradeLegRefID < Quickfix::StringField
		def TradeLegRefID.field
			return 824
		end
		def initialize(data = nil)
			if( data == nil )
				super(824)
			else
				super(824, data)
			end
		end
	end

	class ExchangeRule < Quickfix::StringField
		def ExchangeRule.field
			return 825
		end
		def initialize(data = nil)
			if( data == nil )
				super(825)
			else
				super(825, data)
			end
		end
	end

	class TradeAllocIndicator < Quickfix::IntField
		def TradeAllocIndicator.field
			return 826
		end
		def initialize(data = nil)
			if( data == nil )
				super(826)
			else
				super(826, data)
			end
		end
	end

	class ExpirationCycle < Quickfix::IntField
		def ExpirationCycle.field
			return 827
		end
		def initialize(data = nil)
			if( data == nil )
				super(827)
			else
				super(827, data)
			end
		end
	end

	class TrdType < Quickfix::IntField
		def TrdType.field
			return 828
		end
		def initialize(data = nil)
			if( data == nil )
				super(828)
			else
				super(828, data)
			end
		end
	end

	class TrdSubType < Quickfix::IntField
		def TrdSubType.field
			return 829
		end
		def initialize(data = nil)
			if( data == nil )
				super(829)
			else
				super(829, data)
			end
		end
	end

	class TransferReason < Quickfix::StringField
		def TransferReason.field
			return 830
		end
		def initialize(data = nil)
			if( data == nil )
				super(830)
			else
				super(830, data)
			end
		end
	end

	class TotNumAssignmentReports < Quickfix::IntField
		def TotNumAssignmentReports.field
			return 832
		end
		def initialize(data = nil)
			if( data == nil )
				super(832)
			else
				super(832, data)
			end
		end
	end

	class AsgnRptID < Quickfix::StringField
		def AsgnRptID.field
			return 833
		end
		def initialize(data = nil)
			if( data == nil )
				super(833)
			else
				super(833, data)
			end
		end
	end

	class ThresholdAmount < Quickfix::DoubleField
		def ThresholdAmount.field
			return 834
		end
		def initialize(data = nil)
			if( data == nil )
				super(834)
			else
				super(834, data)
			end
		end
	end

	class PegMoveType < Quickfix::IntField
		def PegMoveType.field
			return 835
		end
		def initialize(data = nil)
			if( data == nil )
				super(835)
			else
				super(835, data)
			end
		end
	end

	class PegOffsetType < Quickfix::IntField
		def PegOffsetType.field
			return 836
		end
		def initialize(data = nil)
			if( data == nil )
				super(836)
			else
				super(836, data)
			end
		end
	end

	class PegLimitType < Quickfix::IntField
		def PegLimitType.field
			return 837
		end
		def initialize(data = nil)
			if( data == nil )
				super(837)
			else
				super(837, data)
			end
		end
	end

	class PegRoundDirection < Quickfix::IntField
		def PegRoundDirection.field
			return 838
		end
		def initialize(data = nil)
			if( data == nil )
				super(838)
			else
				super(838, data)
			end
		end
	end

	class PeggedPrice < Quickfix::DoubleField
		def PeggedPrice.field
			return 839
		end
		def initialize(data = nil)
			if( data == nil )
				super(839)
			else
				super(839, data)
			end
		end
	end

	class PegScope < Quickfix::IntField
		def PegScope.field
			return 840
		end
		def initialize(data = nil)
			if( data == nil )
				super(840)
			else
				super(840, data)
			end
		end
	end

	class DiscretionMoveType < Quickfix::IntField
		def DiscretionMoveType.field
			return 841
		end
		def initialize(data = nil)
			if( data == nil )
				super(841)
			else
				super(841, data)
			end
		end
	end

	class DiscretionOffsetType < Quickfix::IntField
		def DiscretionOffsetType.field
			return 842
		end
		def initialize(data = nil)
			if( data == nil )
				super(842)
			else
				super(842, data)
			end
		end
	end

	class DiscretionLimitType < Quickfix::IntField
		def DiscretionLimitType.field
			return 843
		end
		def initialize(data = nil)
			if( data == nil )
				super(843)
			else
				super(843, data)
			end
		end
	end

	class DiscretionRoundDirection < Quickfix::IntField
		def DiscretionRoundDirection.field
			return 844
		end
		def initialize(data = nil)
			if( data == nil )
				super(844)
			else
				super(844, data)
			end
		end
	end

	class DiscretionPrice < Quickfix::DoubleField
		def DiscretionPrice.field
			return 845
		end
		def initialize(data = nil)
			if( data == nil )
				super(845)
			else
				super(845, data)
			end
		end
	end

	class DiscretionScope < Quickfix::IntField
		def DiscretionScope.field
			return 846
		end
		def initialize(data = nil)
			if( data == nil )
				super(846)
			else
				super(846, data)
			end
		end
	end

	class TargetStrategy < Quickfix::IntField
		def TargetStrategy.field
			return 847
		end
		def initialize(data = nil)
			if( data == nil )
				super(847)
			else
				super(847, data)
			end
		end
	end

	class TargetStrategyParameters < Quickfix::StringField
		def TargetStrategyParameters.field
			return 848
		end
		def initialize(data = nil)
			if( data == nil )
				super(848)
			else
				super(848, data)
			end
		end
	end

	class ParticipationRate < Quickfix::DoubleField
		def ParticipationRate.field
			return 849
		end
		def initialize(data = nil)
			if( data == nil )
				super(849)
			else
				super(849, data)
			end
		end
	end

	class TargetStrategyPerformance < Quickfix::DoubleField
		def TargetStrategyPerformance.field
			return 850
		end
		def initialize(data = nil)
			if( data == nil )
				super(850)
			else
				super(850, data)
			end
		end
	end

	class LastLiquidityInd < Quickfix::IntField
		def LastLiquidityInd.field
			return 851
		end
		def initialize(data = nil)
			if( data == nil )
				super(851)
			else
				super(851, data)
			end
		end
	end

	class PublishTrdIndicator < Quickfix::BoolField
		def PublishTrdIndicator.field
			return 852
		end
		def initialize(data = nil)
			if( data == nil )
				super(852)
			else
				super(852, data)
			end
		end
	end

	class ShortSaleReason < Quickfix::IntField
		def ShortSaleReason.field
			return 853
		end
		def initialize(data = nil)
			if( data == nil )
				super(853)
			else
				super(853, data)
			end
		end
	end

	class QtyType < Quickfix::IntField
		def QtyType.field
			return 854
		end
		def initialize(data = nil)
			if( data == nil )
				super(854)
			else
				super(854, data)
			end
		end
	end

	class SecondaryTrdType < Quickfix::IntField
		def SecondaryTrdType.field
			return 855
		end
		def initialize(data = nil)
			if( data == nil )
				super(855)
			else
				super(855, data)
			end
		end
	end

	class TradeReportType < Quickfix::IntField
		def TradeReportType.field
			return 856
		end
		def initialize(data = nil)
			if( data == nil )
				super(856)
			else
				super(856, data)
			end
		end
	end

	class AllocNoOrdersType < Quickfix::IntField
		def AllocNoOrdersType.field
			return 857
		end
		def initialize(data = nil)
			if( data == nil )
				super(857)
			else
				super(857, data)
			end
		end
	end

	class SharedCommission < Quickfix::DoubleField
		def SharedCommission.field
			return 858
		end
		def initialize(data = nil)
			if( data == nil )
				super(858)
			else
				super(858, data)
			end
		end
	end

	class ConfirmReqID < Quickfix::StringField
		def ConfirmReqID.field
			return 859
		end
		def initialize(data = nil)
			if( data == nil )
				super(859)
			else
				super(859, data)
			end
		end
	end

	class AvgParPx < Quickfix::DoubleField
		def AvgParPx.field
			return 860
		end
		def initialize(data = nil)
			if( data == nil )
				super(860)
			else
				super(860, data)
			end
		end
	end

	class ReportedPx < Quickfix::DoubleField
		def ReportedPx.field
			return 861
		end
		def initialize(data = nil)
			if( data == nil )
				super(861)
			else
				super(861, data)
			end
		end
	end

	class NoCapacities < Quickfix::IntField
		def NoCapacities.field
			return 862
		end
		def initialize(data = nil)
			if( data == nil )
				super(862)
			else
				super(862, data)
			end
		end
	end

	class OrderCapacityQty < Quickfix::DoubleField
		def OrderCapacityQty.field
			return 863
		end
		def initialize(data = nil)
			if( data == nil )
				super(863)
			else
				super(863, data)
			end
		end
	end

	class NoEvents < Quickfix::IntField
		def NoEvents.field
			return 864
		end
		def initialize(data = nil)
			if( data == nil )
				super(864)
			else
				super(864, data)
			end
		end
	end

	class EventType < Quickfix::IntField
		def EventType.field
			return 865
		end
		def initialize(data = nil)
			if( data == nil )
				super(865)
			else
				super(865, data)
			end
		end
	end

	class EventDate < Quickfix::StringField
		def EventDate.field
			return 866
		end
		def initialize(data = nil)
			if( data == nil )
				super(866)
			else
				super(866, data)
			end
		end
	end

	class EventPx < Quickfix::DoubleField
		def EventPx.field
			return 867
		end
		def initialize(data = nil)
			if( data == nil )
				super(867)
			else
				super(867, data)
			end
		end
	end

	class EventText < Quickfix::StringField
		def EventText.field
			return 868
		end
		def initialize(data = nil)
			if( data == nil )
				super(868)
			else
				super(868, data)
			end
		end
	end

	class PctAtRisk < Quickfix::DoubleField
		def PctAtRisk.field
			return 869
		end
		def initialize(data = nil)
			if( data == nil )
				super(869)
			else
				super(869, data)
			end
		end
	end

	class NoInstrAttrib < Quickfix::IntField
		def NoInstrAttrib.field
			return 870
		end
		def initialize(data = nil)
			if( data == nil )
				super(870)
			else
				super(870, data)
			end
		end
	end

	class InstrAttribType < Quickfix::IntField
		def InstrAttribType.field
			return 871
		end
		def initialize(data = nil)
			if( data == nil )
				super(871)
			else
				super(871, data)
			end
		end
	end

	class InstrAttribValue < Quickfix::StringField
		def InstrAttribValue.field
			return 872
		end
		def initialize(data = nil)
			if( data == nil )
				super(872)
			else
				super(872, data)
			end
		end
	end

	class DatedDate < Quickfix::StringField
		def DatedDate.field
			return 873
		end
		def initialize(data = nil)
			if( data == nil )
				super(873)
			else
				super(873, data)
			end
		end
	end

	class InterestAccrualDate < Quickfix::StringField
		def InterestAccrualDate.field
			return 874
		end
		def initialize(data = nil)
			if( data == nil )
				super(874)
			else
				super(874, data)
			end
		end
	end

	class CPProgram < Quickfix::IntField
		def CPProgram.field
			return 875
		end
		def initialize(data = nil)
			if( data == nil )
				super(875)
			else
				super(875, data)
			end
		end
	end

	class CPRegType < Quickfix::StringField
		def CPRegType.field
			return 876
		end
		def initialize(data = nil)
			if( data == nil )
				super(876)
			else
				super(876, data)
			end
		end
	end

	class UnderlyingCPProgram < Quickfix::IntField
		def UnderlyingCPProgram.field
			return 877
		end
		def initialize(data = nil)
			if( data == nil )
				super(877)
			else
				super(877, data)
			end
		end
	end

	class UnderlyingCPRegType < Quickfix::StringField
		def UnderlyingCPRegType.field
			return 878
		end
		def initialize(data = nil)
			if( data == nil )
				super(878)
			else
				super(878, data)
			end
		end
	end

	class UnderlyingQty < Quickfix::DoubleField
		def UnderlyingQty.field
			return 879
		end
		def initialize(data = nil)
			if( data == nil )
				super(879)
			else
				super(879, data)
			end
		end
	end

	class TrdMatchID < Quickfix::StringField
		def TrdMatchID.field
			return 880
		end
		def initialize(data = nil)
			if( data == nil )
				super(880)
			else
				super(880, data)
			end
		end
	end

	class SecondaryTradeReportRefID < Quickfix::StringField
		def SecondaryTradeReportRefID.field
			return 881
		end
		def initialize(data = nil)
			if( data == nil )
				super(881)
			else
				super(881, data)
			end
		end
	end

	class UnderlyingDirtyPrice < Quickfix::DoubleField
		def UnderlyingDirtyPrice.field
			return 882
		end
		def initialize(data = nil)
			if( data == nil )
				super(882)
			else
				super(882, data)
			end
		end
	end

	class UnderlyingEndPrice < Quickfix::DoubleField
		def UnderlyingEndPrice.field
			return 883
		end
		def initialize(data = nil)
			if( data == nil )
				super(883)
			else
				super(883, data)
			end
		end
	end

	class UnderlyingStartValue < Quickfix::DoubleField
		def UnderlyingStartValue.field
			return 884
		end
		def initialize(data = nil)
			if( data == nil )
				super(884)
			else
				super(884, data)
			end
		end
	end

	class UnderlyingCurrentValue < Quickfix::DoubleField
		def UnderlyingCurrentValue.field
			return 885
		end
		def initialize(data = nil)
			if( data == nil )
				super(885)
			else
				super(885, data)
			end
		end
	end

	class UnderlyingEndValue < Quickfix::DoubleField
		def UnderlyingEndValue.field
			return 886
		end
		def initialize(data = nil)
			if( data == nil )
				super(886)
			else
				super(886, data)
			end
		end
	end

	class NoUnderlyingStips < Quickfix::IntField
		def NoUnderlyingStips.field
			return 887
		end
		def initialize(data = nil)
			if( data == nil )
				super(887)
			else
				super(887, data)
			end
		end
	end

	class UnderlyingStipType < Quickfix::StringField
		def UnderlyingStipType.field
			return 888
		end
		def initialize(data = nil)
			if( data == nil )
				super(888)
			else
				super(888, data)
			end
		end
	end

	class UnderlyingStipValue < Quickfix::StringField
		def UnderlyingStipValue.field
			return 889
		end
		def initialize(data = nil)
			if( data == nil )
				super(889)
			else
				super(889, data)
			end
		end
	end

	class MaturityNetMoney < Quickfix::DoubleField
		def MaturityNetMoney.field
			return 890
		end
		def initialize(data = nil)
			if( data == nil )
				super(890)
			else
				super(890, data)
			end
		end
	end

	class MiscFeeBasis < Quickfix::IntField
		def MiscFeeBasis.field
			return 891
		end
		def initialize(data = nil)
			if( data == nil )
				super(891)
			else
				super(891, data)
			end
		end
	end

	class TotNoAllocs < Quickfix::IntField
		def TotNoAllocs.field
			return 892
		end
		def initialize(data = nil)
			if( data == nil )
				super(892)
			else
				super(892, data)
			end
		end
	end

	class LastFragment < Quickfix::BoolField
		def LastFragment.field
			return 893
		end
		def initialize(data = nil)
			if( data == nil )
				super(893)
			else
				super(893, data)
			end
		end
	end

	class CollReqID < Quickfix::StringField
		def CollReqID.field
			return 894
		end
		def initialize(data = nil)
			if( data == nil )
				super(894)
			else
				super(894, data)
			end
		end
	end

	class CollAsgnReason < Quickfix::IntField
		def CollAsgnReason.field
			return 895
		end
		def initialize(data = nil)
			if( data == nil )
				super(895)
			else
				super(895, data)
			end
		end
	end

	class CollInquiryQualifier < Quickfix::IntField
		def CollInquiryQualifier.field
			return 896
		end
		def initialize(data = nil)
			if( data == nil )
				super(896)
			else
				super(896, data)
			end
		end
	end

	class NoTrades < Quickfix::IntField
		def NoTrades.field
			return 897
		end
		def initialize(data = nil)
			if( data == nil )
				super(897)
			else
				super(897, data)
			end
		end
	end

	class MarginRatio < Quickfix::DoubleField
		def MarginRatio.field
			return 898
		end
		def initialize(data = nil)
			if( data == nil )
				super(898)
			else
				super(898, data)
			end
		end
	end

	class MarginExcess < Quickfix::DoubleField
		def MarginExcess.field
			return 899
		end
		def initialize(data = nil)
			if( data == nil )
				super(899)
			else
				super(899, data)
			end
		end
	end

	class TotalNetValue < Quickfix::DoubleField
		def TotalNetValue.field
			return 900
		end
		def initialize(data = nil)
			if( data == nil )
				super(900)
			else
				super(900, data)
			end
		end
	end

	class CashOutstanding < Quickfix::DoubleField
		def CashOutstanding.field
			return 901
		end
		def initialize(data = nil)
			if( data == nil )
				super(901)
			else
				super(901, data)
			end
		end
	end

	class CollAsgnID < Quickfix::StringField
		def CollAsgnID.field
			return 902
		end
		def initialize(data = nil)
			if( data == nil )
				super(902)
			else
				super(902, data)
			end
		end
	end

	class CollAsgnTransType < Quickfix::IntField
		def CollAsgnTransType.field
			return 903
		end
		def initialize(data = nil)
			if( data == nil )
				super(903)
			else
				super(903, data)
			end
		end
	end

	class CollRespID < Quickfix::StringField
		def CollRespID.field
			return 904
		end
		def initialize(data = nil)
			if( data == nil )
				super(904)
			else
				super(904, data)
			end
		end
	end

	class CollAsgnRespType < Quickfix::IntField
		def CollAsgnRespType.field
			return 905
		end
		def initialize(data = nil)
			if( data == nil )
				super(905)
			else
				super(905, data)
			end
		end
	end

	class CollAsgnRejectReason < Quickfix::IntField
		def CollAsgnRejectReason.field
			return 906
		end
		def initialize(data = nil)
			if( data == nil )
				super(906)
			else
				super(906, data)
			end
		end
	end

	class CollAsgnRefID < Quickfix::StringField
		def CollAsgnRefID.field
			return 907
		end
		def initialize(data = nil)
			if( data == nil )
				super(907)
			else
				super(907, data)
			end
		end
	end

	class CollRptID < Quickfix::StringField
		def CollRptID.field
			return 908
		end
		def initialize(data = nil)
			if( data == nil )
				super(908)
			else
				super(908, data)
			end
		end
	end

	class CollInquiryID < Quickfix::StringField
		def CollInquiryID.field
			return 909
		end
		def initialize(data = nil)
			if( data == nil )
				super(909)
			else
				super(909, data)
			end
		end
	end

	class CollStatus < Quickfix::IntField
		def CollStatus.field
			return 910
		end
		def initialize(data = nil)
			if( data == nil )
				super(910)
			else
				super(910, data)
			end
		end
	end

	class TotNumReports < Quickfix::IntField
		def TotNumReports.field
			return 911
		end
		def initialize(data = nil)
			if( data == nil )
				super(911)
			else
				super(911, data)
			end
		end
	end

	class LastRptRequested < Quickfix::BoolField
		def LastRptRequested.field
			return 912
		end
		def initialize(data = nil)
			if( data == nil )
				super(912)
			else
				super(912, data)
			end
		end
	end

	class AgreementDesc < Quickfix::StringField
		def AgreementDesc.field
			return 913
		end
		def initialize(data = nil)
			if( data == nil )
				super(913)
			else
				super(913, data)
			end
		end
	end

	class AgreementID < Quickfix::StringField
		def AgreementID.field
			return 914
		end
		def initialize(data = nil)
			if( data == nil )
				super(914)
			else
				super(914, data)
			end
		end
	end

	class AgreementDate < Quickfix::StringField
		def AgreementDate.field
			return 915
		end
		def initialize(data = nil)
			if( data == nil )
				super(915)
			else
				super(915, data)
			end
		end
	end

	class StartDate < Quickfix::StringField
		def StartDate.field
			return 916
		end
		def initialize(data = nil)
			if( data == nil )
				super(916)
			else
				super(916, data)
			end
		end
	end

	class EndDate < Quickfix::StringField
		def EndDate.field
			return 917
		end
		def initialize(data = nil)
			if( data == nil )
				super(917)
			else
				super(917, data)
			end
		end
	end

	class AgreementCurrency < Quickfix::StringField
		def AgreementCurrency.field
			return 918
		end
		def initialize(data = nil)
			if( data == nil )
				super(918)
			else
				super(918, data)
			end
		end
	end

	class DeliveryType < Quickfix::IntField
		def DeliveryType.field
			return 919
		end
		def initialize(data = nil)
			if( data == nil )
				super(919)
			else
				super(919, data)
			end
		end
	end

	class EndAccruedInterestAmt < Quickfix::DoubleField
		def EndAccruedInterestAmt.field
			return 920
		end
		def initialize(data = nil)
			if( data == nil )
				super(920)
			else
				super(920, data)
			end
		end
	end

	class StartCash < Quickfix::DoubleField
		def StartCash.field
			return 921
		end
		def initialize(data = nil)
			if( data == nil )
				super(921)
			else
				super(921, data)
			end
		end
	end

	class EndCash < Quickfix::DoubleField
		def EndCash.field
			return 922
		end
		def initialize(data = nil)
			if( data == nil )
				super(922)
			else
				super(922, data)
			end
		end
	end

	class UserRequestID < Quickfix::StringField
		def UserRequestID.field
			return 923
		end
		def initialize(data = nil)
			if( data == nil )
				super(923)
			else
				super(923, data)
			end
		end
	end

	class UserRequestType < Quickfix::IntField
		def UserRequestType.field
			return 924
		end
		def initialize(data = nil)
			if( data == nil )
				super(924)
			else
				super(924, data)
			end
		end
	end

	class UserStatus < Quickfix::IntField
		def UserStatus.field
			return 926
		end
		def initialize(data = nil)
			if( data == nil )
				super(926)
			else
				super(926, data)
			end
		end
	end

	class UserStatusText < Quickfix::StringField
		def UserStatusText.field
			return 927
		end
		def initialize(data = nil)
			if( data == nil )
				super(927)
			else
				super(927, data)
			end
		end
	end

	class StatusValue < Quickfix::IntField
		def StatusValue.field
			return 928
		end
		def initialize(data = nil)
			if( data == nil )
				super(928)
			else
				super(928, data)
			end
		end
	end

	class StatusText < Quickfix::StringField
		def StatusText.field
			return 929
		end
		def initialize(data = nil)
			if( data == nil )
				super(929)
			else
				super(929, data)
			end
		end
	end

	class RefCompID < Quickfix::StringField
		def RefCompID.field
			return 930
		end
		def initialize(data = nil)
			if( data == nil )
				super(930)
			else
				super(930, data)
			end
		end
	end

	class RefSubID < Quickfix::StringField
		def RefSubID.field
			return 931
		end
		def initialize(data = nil)
			if( data == nil )
				super(931)
			else
				super(931, data)
			end
		end
	end

	class NetworkResponseID < Quickfix::StringField
		def NetworkResponseID.field
			return 932
		end
		def initialize(data = nil)
			if( data == nil )
				super(932)
			else
				super(932, data)
			end
		end
	end

	class NetworkRequestID < Quickfix::StringField
		def NetworkRequestID.field
			return 933
		end
		def initialize(data = nil)
			if( data == nil )
				super(933)
			else
				super(933, data)
			end
		end
	end

	class LastNetworkResponseID < Quickfix::StringField
		def LastNetworkResponseID.field
			return 934
		end
		def initialize(data = nil)
			if( data == nil )
				super(934)
			else
				super(934, data)
			end
		end
	end

	class NetworkRequestType < Quickfix::IntField
		def NetworkRequestType.field
			return 935
		end
		def initialize(data = nil)
			if( data == nil )
				super(935)
			else
				super(935, data)
			end
		end
	end

	class NoCompIDs < Quickfix::IntField
		def NoCompIDs.field
			return 936
		end
		def initialize(data = nil)
			if( data == nil )
				super(936)
			else
				super(936, data)
			end
		end
	end

	class NetworkStatusResponseType < Quickfix::IntField
		def NetworkStatusResponseType.field
			return 937
		end
		def initialize(data = nil)
			if( data == nil )
				super(937)
			else
				super(937, data)
			end
		end
	end

	class NoCollInquiryQualifier < Quickfix::IntField
		def NoCollInquiryQualifier.field
			return 938
		end
		def initialize(data = nil)
			if( data == nil )
				super(938)
			else
				super(938, data)
			end
		end
	end

	class TrdRptStatus < Quickfix::IntField
		def TrdRptStatus.field
			return 939
		end
		def initialize(data = nil)
			if( data == nil )
				super(939)
			else
				super(939, data)
			end
		end
	end

	class AffirmStatus < Quickfix::IntField
		def AffirmStatus.field
			return 940
		end
		def initialize(data = nil)
			if( data == nil )
				super(940)
			else
				super(940, data)
			end
		end
	end

	class UnderlyingStrikeCurrency < Quickfix::StringField
		def UnderlyingStrikeCurrency.field
			return 941
		end
		def initialize(data = nil)
			if( data == nil )
				super(941)
			else
				super(941, data)
			end
		end
	end

	class LegStrikeCurrency < Quickfix::StringField
		def LegStrikeCurrency.field
			return 942
		end
		def initialize(data = nil)
			if( data == nil )
				super(942)
			else
				super(942, data)
			end
		end
	end

	class TimeBracket < Quickfix::StringField
		def TimeBracket.field
			return 943
		end
		def initialize(data = nil)
			if( data == nil )
				super(943)
			else
				super(943, data)
			end
		end
	end

	class CollAction < Quickfix::IntField
		def CollAction.field
			return 944
		end
		def initialize(data = nil)
			if( data == nil )
				super(944)
			else
				super(944, data)
			end
		end
	end

	class CollInquiryStatus < Quickfix::IntField
		def CollInquiryStatus.field
			return 945
		end
		def initialize(data = nil)
			if( data == nil )
				super(945)
			else
				super(945, data)
			end
		end
	end

	class CollInquiryResult < Quickfix::IntField
		def CollInquiryResult.field
			return 946
		end
		def initialize(data = nil)
			if( data == nil )
				super(946)
			else
				super(946, data)
			end
		end
	end

	class StrikeCurrency < Quickfix::StringField
		def StrikeCurrency.field
			return 947
		end
		def initialize(data = nil)
			if( data == nil )
				super(947)
			else
				super(947, data)
			end
		end
	end

	class NoNested3PartyIDs < Quickfix::IntField
		def NoNested3PartyIDs.field
			return 948
		end
		def initialize(data = nil)
			if( data == nil )
				super(948)
			else
				super(948, data)
			end
		end
	end

	class Nested3PartyID < Quickfix::StringField
		def Nested3PartyID.field
			return 949
		end
		def initialize(data = nil)
			if( data == nil )
				super(949)
			else
				super(949, data)
			end
		end
	end

	class Nested3PartyIDSource < Quickfix::CharField
		def Nested3PartyIDSource.field
			return 950
		end
		def initialize(data = nil)
			if( data == nil )
				super(950)
			else
				super(950, data)
			end
		end
	end

	class Nested3PartyRole < Quickfix::IntField
		def Nested3PartyRole.field
			return 951
		end
		def initialize(data = nil)
			if( data == nil )
				super(951)
			else
				super(951, data)
			end
		end
	end

	class NoNested3PartySubIDs < Quickfix::IntField
		def NoNested3PartySubIDs.field
			return 952
		end
		def initialize(data = nil)
			if( data == nil )
				super(952)
			else
				super(952, data)
			end
		end
	end

	class Nested3PartySubID < Quickfix::StringField
		def Nested3PartySubID.field
			return 953
		end
		def initialize(data = nil)
			if( data == nil )
				super(953)
			else
				super(953, data)
			end
		end
	end

	class Nested3PartySubIDType < Quickfix::IntField
		def Nested3PartySubIDType.field
			return 954
		end
		def initialize(data = nil)
			if( data == nil )
				super(954)
			else
				super(954, data)
			end
		end
	end

	class LegContractSettlMonth < Quickfix::StringField
		def LegContractSettlMonth.field
			return 955
		end
		def initialize(data = nil)
			if( data == nil )
				super(955)
			else
				super(955, data)
			end
		end
	end

	class LegInterestAccrualDate < Quickfix::StringField
		def LegInterestAccrualDate.field
			return 956
		end
		def initialize(data = nil)
			if( data == nil )
				super(956)
			else
				super(956, data)
			end
		end
	end

	class LegOrderQty < Quickfix::DoubleField
		def LegOrderQty.field
			return 685
		end
		def initialize(data = nil)
			if( data == nil )
				super(685)
			else
				super(685, data)
			end
		end
	end

	class NoStrategyParameters < Quickfix::IntField
		def NoStrategyParameters.field
			return 957
		end
		def initialize(data = nil)
			if( data == nil )
				super(957)
			else
				super(957, data)
			end
		end
	end

	class StrategyParameterName < Quickfix::StringField
		def StrategyParameterName.field
			return 958
		end
		def initialize(data = nil)
			if( data == nil )
				super(958)
			else
				super(958, data)
			end
		end
	end

	class StrategyParameterType < Quickfix::IntField
		def StrategyParameterType.field
			return 959
		end
		def initialize(data = nil)
			if( data == nil )
				super(959)
			else
				super(959, data)
			end
		end
	end

	class StrategyParameterValue < Quickfix::StringField
		def StrategyParameterValue.field
			return 960
		end
		def initialize(data = nil)
			if( data == nil )
				super(960)
			else
				super(960, data)
			end
		end
	end

	class HostCrossID < Quickfix::StringField
		def HostCrossID.field
			return 961
		end
		def initialize(data = nil)
			if( data == nil )
				super(961)
			else
				super(961, data)
			end
		end
	end

	class SideTimeInForce < Quickfix::UtcTimeStampField
		def SideTimeInForce.field
			return 962
		end
		def initialize(data = nil)
			if( data == nil )
				super(962)
			else
				super(962, data)
			end
		end
	end

	class MDReportID < Quickfix::IntField
		def MDReportID.field
			return 963
		end
		def initialize(data = nil)
			if( data == nil )
				super(963)
			else
				super(963, data)
			end
		end
	end

	class SecurityReportID < Quickfix::IntField
		def SecurityReportID.field
			return 964
		end
		def initialize(data = nil)
			if( data == nil )
				super(964)
			else
				super(964, data)
			end
		end
	end

	class SecurityStatus < Quickfix::StringField
		def SecurityStatus.field
			return 965
		end
		def initialize(data = nil)
			if( data == nil )
				super(965)
			else
				super(965, data)
			end
		end
	end

	class SettleOnOpenFlag < Quickfix::StringField
		def SettleOnOpenFlag.field
			return 966
		end
		def initialize(data = nil)
			if( data == nil )
				super(966)
			else
				super(966, data)
			end
		end
	end

	class StrikeMultiplier < Quickfix::DoubleField
		def StrikeMultiplier.field
			return 967
		end
		def initialize(data = nil)
			if( data == nil )
				super(967)
			else
				super(967, data)
			end
		end
	end

	class StrikeValue < Quickfix::DoubleField
		def StrikeValue.field
			return 968
		end
		def initialize(data = nil)
			if( data == nil )
				super(968)
			else
				super(968, data)
			end
		end
	end

	class MinPriceIncrement < Quickfix::DoubleField
		def MinPriceIncrement.field
			return 969
		end
		def initialize(data = nil)
			if( data == nil )
				super(969)
			else
				super(969, data)
			end
		end
	end

	class PositionLimit < Quickfix::IntField
		def PositionLimit.field
			return 970
		end
		def initialize(data = nil)
			if( data == nil )
				super(970)
			else
				super(970, data)
			end
		end
	end

	class NTPositionLimit < Quickfix::IntField
		def NTPositionLimit.field
			return 971
		end
		def initialize(data = nil)
			if( data == nil )
				super(971)
			else
				super(971, data)
			end
		end
	end

	class UnderlyingAllocationPercent < Quickfix::DoubleField
		def UnderlyingAllocationPercent.field
			return 972
		end
		def initialize(data = nil)
			if( data == nil )
				super(972)
			else
				super(972, data)
			end
		end
	end

	class UnderlyingCashAmount < Quickfix::DoubleField
		def UnderlyingCashAmount.field
			return 973
		end
		def initialize(data = nil)
			if( data == nil )
				super(973)
			else
				super(973, data)
			end
		end
	end

	class UnderlyingCashType < Quickfix::StringField
		def UnderlyingCashType.field
			return 974
		end
		def initialize(data = nil)
			if( data == nil )
				super(974)
			else
				super(974, data)
			end
		end
	end

	class UnderlyingSettlementType < Quickfix::IntField
		def UnderlyingSettlementType.field
			return 975
		end
		def initialize(data = nil)
			if( data == nil )
				super(975)
			else
				super(975, data)
			end
		end
	end

	class QuantityDate < Quickfix::StringField
		def QuantityDate.field
			return 976
		end
		def initialize(data = nil)
			if( data == nil )
				super(976)
			else
				super(976, data)
			end
		end
	end

	class ContIntRptID < Quickfix::StringField
		def ContIntRptID.field
			return 977
		end
		def initialize(data = nil)
			if( data == nil )
				super(977)
			else
				super(977, data)
			end
		end
	end

	class LateIndicator < Quickfix::BoolField
		def LateIndicator.field
			return 978
		end
		def initialize(data = nil)
			if( data == nil )
				super(978)
			else
				super(978, data)
			end
		end
	end

	class InputSource < Quickfix::StringField
		def InputSource.field
			return 979
		end
		def initialize(data = nil)
			if( data == nil )
				super(979)
			else
				super(979, data)
			end
		end
	end

	class SecurityUpdateAction < Quickfix::CharField
		def SecurityUpdateAction.field
			return 980
		end
		def initialize(data = nil)
			if( data == nil )
				super(980)
			else
				super(980, data)
			end
		end
	end

	class NoExpiration < Quickfix::IntField
		def NoExpiration.field
			return 981
		end
		def initialize(data = nil)
			if( data == nil )
				super(981)
			else
				super(981, data)
			end
		end
	end

	class ExpType < Quickfix::IntField
		def ExpType.field
			return 982
		end
		def initialize(data = nil)
			if( data == nil )
				super(982)
			else
				super(982, data)
			end
		end
	end

	class ExpQty < Quickfix::DoubleField
		def ExpQty.field
			return 983
		end
		def initialize(data = nil)
			if( data == nil )
				super(983)
			else
				super(983, data)
			end
		end
	end

	class NoUnderlyingAmounts < Quickfix::IntField
		def NoUnderlyingAmounts.field
			return 984
		end
		def initialize(data = nil)
			if( data == nil )
				super(984)
			else
				super(984, data)
			end
		end
	end

	class UnderlyingPayAmount < Quickfix::DoubleField
		def UnderlyingPayAmount.field
			return 985
		end
		def initialize(data = nil)
			if( data == nil )
				super(985)
			else
				super(985, data)
			end
		end
	end

	class UnderlyingCollectAmount < Quickfix::DoubleField
		def UnderlyingCollectAmount.field
			return 986
		end
		def initialize(data = nil)
			if( data == nil )
				super(986)
			else
				super(986, data)
			end
		end
	end

	class UnderlyingSettlementDate < Quickfix::StringField
		def UnderlyingSettlementDate.field
			return 987
		end
		def initialize(data = nil)
			if( data == nil )
				super(987)
			else
				super(987, data)
			end
		end
	end

	class UnderlyingSettlementStatus < Quickfix::StringField
		def UnderlyingSettlementStatus.field
			return 988
		end
		def initialize(data = nil)
			if( data == nil )
				super(988)
			else
				super(988, data)
			end
		end
	end

	class SecondaryIndividualAllocID < Quickfix::StringField
		def SecondaryIndividualAllocID.field
			return 989
		end
		def initialize(data = nil)
			if( data == nil )
				super(989)
			else
				super(989, data)
			end
		end
	end

	class LegReportID < Quickfix::StringField
		def LegReportID.field
			return 990
		end
		def initialize(data = nil)
			if( data == nil )
				super(990)
			else
				super(990, data)
			end
		end
	end

	class RndPx < Quickfix::DoubleField
		def RndPx.field
			return 991
		end
		def initialize(data = nil)
			if( data == nil )
				super(991)
			else
				super(991, data)
			end
		end
	end

	class IndividualAllocType < Quickfix::IntField
		def IndividualAllocType.field
			return 992
		end
		def initialize(data = nil)
			if( data == nil )
				super(992)
			else
				super(992, data)
			end
		end
	end

	class AllocCustomerCapacity < Quickfix::StringField
		def AllocCustomerCapacity.field
			return 993
		end
		def initialize(data = nil)
			if( data == nil )
				super(993)
			else
				super(993, data)
			end
		end
	end

	class TierCode < Quickfix::StringField
		def TierCode.field
			return 994
		end
		def initialize(data = nil)
			if( data == nil )
				super(994)
			else
				super(994, data)
			end
		end
	end

	class UnitofMeasure < Quickfix::StringField
		def UnitofMeasure.field
			return 996
		end
		def initialize(data = nil)
			if( data == nil )
				super(996)
			else
				super(996, data)
			end
		end
	end

	class TimeUnit < Quickfix::StringField
		def TimeUnit.field
			return 997
		end
		def initialize(data = nil)
			if( data == nil )
				super(997)
			else
				super(997, data)
			end
		end
	end

	class UnderlyingUnitofMeasure < Quickfix::StringField
		def UnderlyingUnitofMeasure.field
			return 998
		end
		def initialize(data = nil)
			if( data == nil )
				super(998)
			else
				super(998, data)
			end
		end
	end

	class LegUnitofMeasure < Quickfix::StringField
		def LegUnitofMeasure.field
			return 999
		end
		def initialize(data = nil)
			if( data == nil )
				super(999)
			else
				super(999, data)
			end
		end
	end

	class UnderlyingTimeUnit < Quickfix::StringField
		def UnderlyingTimeUnit.field
			return 1000
		end
		def initialize(data = nil)
			if( data == nil )
				super(1000)
			else
				super(1000, data)
			end
		end
	end

	class LegTimeUnit < Quickfix::StringField
		def LegTimeUnit.field
			return 1001
		end
		def initialize(data = nil)
			if( data == nil )
				super(1001)
			else
				super(1001, data)
			end
		end
	end

	class AllocMethod < Quickfix::IntField
		def AllocMethod.field
			return 1002
		end
		def initialize(data = nil)
			if( data == nil )
				super(1002)
			else
				super(1002, data)
			end
		end
	end

	class TradeID < Quickfix::StringField
		def TradeID.field
			return 1003
		end
		def initialize(data = nil)
			if( data == nil )
				super(1003)
			else
				super(1003, data)
			end
		end
	end

	class SideTradeReportID < Quickfix::StringField
		def SideTradeReportID.field
			return 1005
		end
		def initialize(data = nil)
			if( data == nil )
				super(1005)
			else
				super(1005, data)
			end
		end
	end

	class SideFillStationCd < Quickfix::StringField
		def SideFillStationCd.field
			return 1006
		end
		def initialize(data = nil)
			if( data == nil )
				super(1006)
			else
				super(1006, data)
			end
		end
	end

	class SideReasonCd < Quickfix::StringField
		def SideReasonCd.field
			return 1007
		end
		def initialize(data = nil)
			if( data == nil )
				super(1007)
			else
				super(1007, data)
			end
		end
	end

	class SideTrdSubTyp < Quickfix::IntField
		def SideTrdSubTyp.field
			return 1008
		end
		def initialize(data = nil)
			if( data == nil )
				super(1008)
			else
				super(1008, data)
			end
		end
	end

	class SideQty < Quickfix::IntField
		def SideQty.field
			return 1009
		end
		def initialize(data = nil)
			if( data == nil )
				super(1009)
			else
				super(1009, data)
			end
		end
	end

	class MessageEventSource < Quickfix::StringField
		def MessageEventSource.field
			return 1011
		end
		def initialize(data = nil)
			if( data == nil )
				super(1011)
			else
				super(1011, data)
			end
		end
	end

	class SideTrdRegTimestamp < Quickfix::UtcTimeStampField
		def SideTrdRegTimestamp.field
			return 1012
		end
		def initialize(data = nil)
			if( data == nil )
				super(1012)
			else
				super(1012, data)
			end
		end
	end

	class SideTrdRegTimestampType < Quickfix::IntField
		def SideTrdRegTimestampType.field
			return 1013
		end
		def initialize(data = nil)
			if( data == nil )
				super(1013)
			else
				super(1013, data)
			end
		end
	end

	class SideTrdRegTimestampSrc < Quickfix::StringField
		def SideTrdRegTimestampSrc.field
			return 1014
		end
		def initialize(data = nil)
			if( data == nil )
				super(1014)
			else
				super(1014, data)
			end
		end
	end

	class AsOfIndicator < Quickfix::CharField
		def AsOfIndicator.field
			return 1015
		end
		def initialize(data = nil)
			if( data == nil )
				super(1015)
			else
				super(1015, data)
			end
		end
	end

	class NoSideTrdRegTS < Quickfix::IntField
		def NoSideTrdRegTS.field
			return 1016
		end
		def initialize(data = nil)
			if( data == nil )
				super(1016)
			else
				super(1016, data)
			end
		end
	end

	class LegOptionRatio < Quickfix::DoubleField
		def LegOptionRatio.field
			return 1017
		end
		def initialize(data = nil)
			if( data == nil )
				super(1017)
			else
				super(1017, data)
			end
		end
	end

	class NoInstrumentParties < Quickfix::IntField
		def NoInstrumentParties.field
			return 1018
		end
		def initialize(data = nil)
			if( data == nil )
				super(1018)
			else
				super(1018, data)
			end
		end
	end

	class InstrumentPartyID < Quickfix::StringField
		def InstrumentPartyID.field
			return 1019
		end
		def initialize(data = nil)
			if( data == nil )
				super(1019)
			else
				super(1019, data)
			end
		end
	end

	class TradeVolume < Quickfix::DoubleField
		def TradeVolume.field
			return 1020
		end
		def initialize(data = nil)
			if( data == nil )
				super(1020)
			else
				super(1020, data)
			end
		end
	end

	class MDBookType < Quickfix::IntField
		def MDBookType.field
			return 1021
		end
		def initialize(data = nil)
			if( data == nil )
				super(1021)
			else
				super(1021, data)
			end
		end
	end

	class MDFeedType < Quickfix::StringField
		def MDFeedType.field
			return 1022
		end
		def initialize(data = nil)
			if( data == nil )
				super(1022)
			else
				super(1022, data)
			end
		end
	end

	class MDPriceLevel < Quickfix::IntField
		def MDPriceLevel.field
			return 1023
		end
		def initialize(data = nil)
			if( data == nil )
				super(1023)
			else
				super(1023, data)
			end
		end
	end

	class MDOriginType < Quickfix::IntField
		def MDOriginType.field
			return 1024
		end
		def initialize(data = nil)
			if( data == nil )
				super(1024)
			else
				super(1024, data)
			end
		end
	end

	class FirstPx < Quickfix::DoubleField
		def FirstPx.field
			return 1025
		end
		def initialize(data = nil)
			if( data == nil )
				super(1025)
			else
				super(1025, data)
			end
		end
	end

	class MDEntrySpotRate < Quickfix::DoubleField
		def MDEntrySpotRate.field
			return 1026
		end
		def initialize(data = nil)
			if( data == nil )
				super(1026)
			else
				super(1026, data)
			end
		end
	end

	class MDEntryForwardPoints < Quickfix::DoubleField
		def MDEntryForwardPoints.field
			return 1027
		end
		def initialize(data = nil)
			if( data == nil )
				super(1027)
			else
				super(1027, data)
			end
		end
	end

	class ManualOrderIndicator < Quickfix::BoolField
		def ManualOrderIndicator.field
			return 1028
		end
		def initialize(data = nil)
			if( data == nil )
				super(1028)
			else
				super(1028, data)
			end
		end
	end

	class CustDirectedOrder < Quickfix::BoolField
		def CustDirectedOrder.field
			return 1029
		end
		def initialize(data = nil)
			if( data == nil )
				super(1029)
			else
				super(1029, data)
			end
		end
	end

	class ReceivedDeptID < Quickfix::StringField
		def ReceivedDeptID.field
			return 1030
		end
		def initialize(data = nil)
			if( data == nil )
				super(1030)
			else
				super(1030, data)
			end
		end
	end

	class CustOrderHandlingInst < Quickfix::StringField
		def CustOrderHandlingInst.field
			return 1031
		end
		def initialize(data = nil)
			if( data == nil )
				super(1031)
			else
				super(1031, data)
			end
		end
	end

	class OrderHandlingInstSource < Quickfix::IntField
		def OrderHandlingInstSource.field
			return 1032
		end
		def initialize(data = nil)
			if( data == nil )
				super(1032)
			else
				super(1032, data)
			end
		end
	end

	class DeskType < Quickfix::StringField
		def DeskType.field
			return 1033
		end
		def initialize(data = nil)
			if( data == nil )
				super(1033)
			else
				super(1033, data)
			end
		end
	end

	class DeskTypeSource < Quickfix::IntField
		def DeskTypeSource.field
			return 1034
		end
		def initialize(data = nil)
			if( data == nil )
				super(1034)
			else
				super(1034, data)
			end
		end
	end

	class DeskOrderHandlingInst < Quickfix::StringField
		def DeskOrderHandlingInst.field
			return 1035
		end
		def initialize(data = nil)
			if( data == nil )
				super(1035)
			else
				super(1035, data)
			end
		end
	end

	class ExecAckStatus < Quickfix::CharField
		def ExecAckStatus.field
			return 1036
		end
		def initialize(data = nil)
			if( data == nil )
				super(1036)
			else
				super(1036, data)
			end
		end
	end

	class UnderlyingDeliveryAmount < Quickfix::DoubleField
		def UnderlyingDeliveryAmount.field
			return 1037
		end
		def initialize(data = nil)
			if( data == nil )
				super(1037)
			else
				super(1037, data)
			end
		end
	end

	class UnderlyingCapValue < Quickfix::DoubleField
		def UnderlyingCapValue.field
			return 1038
		end
		def initialize(data = nil)
			if( data == nil )
				super(1038)
			else
				super(1038, data)
			end
		end
	end

	class UnderlyingSettlMethod < Quickfix::StringField
		def UnderlyingSettlMethod.field
			return 1039
		end
		def initialize(data = nil)
			if( data == nil )
				super(1039)
			else
				super(1039, data)
			end
		end
	end

	class SecondaryTradeID < Quickfix::StringField
		def SecondaryTradeID.field
			return 1040
		end
		def initialize(data = nil)
			if( data == nil )
				super(1040)
			else
				super(1040, data)
			end
		end
	end

	class FirmTradeID < Quickfix::StringField
		def FirmTradeID.field
			return 1041
		end
		def initialize(data = nil)
			if( data == nil )
				super(1041)
			else
				super(1041, data)
			end
		end
	end

	class SecondaryFirmTradeID < Quickfix::StringField
		def SecondaryFirmTradeID.field
			return 1042
		end
		def initialize(data = nil)
			if( data == nil )
				super(1042)
			else
				super(1042, data)
			end
		end
	end

	class CollApplType < Quickfix::IntField
		def CollApplType.field
			return 1043
		end
		def initialize(data = nil)
			if( data == nil )
				super(1043)
			else
				super(1043, data)
			end
		end
	end

	class UnderlyingAdjustedQuantity < Quickfix::DoubleField
		def UnderlyingAdjustedQuantity.field
			return 1044
		end
		def initialize(data = nil)
			if( data == nil )
				super(1044)
			else
				super(1044, data)
			end
		end
	end

	class UnderlyingFXRate < Quickfix::DoubleField
		def UnderlyingFXRate.field
			return 1045
		end
		def initialize(data = nil)
			if( data == nil )
				super(1045)
			else
				super(1045, data)
			end
		end
	end

	class UnderlyingFXRateCalc < Quickfix::CharField
		def UnderlyingFXRateCalc.field
			return 1046
		end
		def initialize(data = nil)
			if( data == nil )
				super(1046)
			else
				super(1046, data)
			end
		end
	end

	class AllocPositionEffect < Quickfix::CharField
		def AllocPositionEffect.field
			return 1047
		end
		def initialize(data = nil)
			if( data == nil )
				super(1047)
			else
				super(1047, data)
			end
		end
	end

	class DealingCapacity < Quickfix::CharField
		def DealingCapacity.field
			return 1048
		end
		def initialize(data = nil)
			if( data == nil )
				super(1048)
			else
				super(1048, data)
			end
		end
	end

	class InstrmtAssignmentMethod < Quickfix::CharField
		def InstrmtAssignmentMethod.field
			return 1049
		end
		def initialize(data = nil)
			if( data == nil )
				super(1049)
			else
				super(1049, data)
			end
		end
	end

	class InstrumentPartyIDSource < Quickfix::CharField
		def InstrumentPartyIDSource.field
			return 1050
		end
		def initialize(data = nil)
			if( data == nil )
				super(1050)
			else
				super(1050, data)
			end
		end
	end

	class InstrumentPartyRole < Quickfix::IntField
		def InstrumentPartyRole.field
			return 1051
		end
		def initialize(data = nil)
			if( data == nil )
				super(1051)
			else
				super(1051, data)
			end
		end
	end

	class NoInstrumentPartySubIDs < Quickfix::IntField
		def NoInstrumentPartySubIDs.field
			return 1052
		end
		def initialize(data = nil)
			if( data == nil )
				super(1052)
			else
				super(1052, data)
			end
		end
	end

	class InstrumentPartySubID < Quickfix::StringField
		def InstrumentPartySubID.field
			return 1053
		end
		def initialize(data = nil)
			if( data == nil )
				super(1053)
			else
				super(1053, data)
			end
		end
	end

	class InstrumentPartySubIDType < Quickfix::IntField
		def InstrumentPartySubIDType.field
			return 1054
		end
		def initialize(data = nil)
			if( data == nil )
				super(1054)
			else
				super(1054, data)
			end
		end
	end

	class PositionCurrency < Quickfix::StringField
		def PositionCurrency.field
			return 1055
		end
		def initialize(data = nil)
			if( data == nil )
				super(1055)
			else
				super(1055, data)
			end
		end
	end

	class CalculatedCcyLastQty < Quickfix::DoubleField
		def CalculatedCcyLastQty.field
			return 1056
		end
		def initialize(data = nil)
			if( data == nil )
				super(1056)
			else
				super(1056, data)
			end
		end
	end

	class AggressorIndicator < Quickfix::BoolField
		def AggressorIndicator.field
			return 1057
		end
		def initialize(data = nil)
			if( data == nil )
				super(1057)
			else
				super(1057, data)
			end
		end
	end

	class NoUndlyInstrumentParties < Quickfix::IntField
		def NoUndlyInstrumentParties.field
			return 1058
		end
		def initialize(data = nil)
			if( data == nil )
				super(1058)
			else
				super(1058, data)
			end
		end
	end

	class UndlyInstrumentPartyID < Quickfix::StringField
		def UndlyInstrumentPartyID.field
			return 1059
		end
		def initialize(data = nil)
			if( data == nil )
				super(1059)
			else
				super(1059, data)
			end
		end
	end

	class UndlyInstrumentPartyIDSource < Quickfix::CharField
		def UndlyInstrumentPartyIDSource.field
			return 1060
		end
		def initialize(data = nil)
			if( data == nil )
				super(1060)
			else
				super(1060, data)
			end
		end
	end

	class UndlyInstrumentPartyRole < Quickfix::IntField
		def UndlyInstrumentPartyRole.field
			return 1061
		end
		def initialize(data = nil)
			if( data == nil )
				super(1061)
			else
				super(1061, data)
			end
		end
	end

	class NoUndlyInstrumentPartySubIDs < Quickfix::IntField
		def NoUndlyInstrumentPartySubIDs.field
			return 1062
		end
		def initialize(data = nil)
			if( data == nil )
				super(1062)
			else
				super(1062, data)
			end
		end
	end

	class UndlyInstrumentPartySubID < Quickfix::StringField
		def UndlyInstrumentPartySubID.field
			return 1063
		end
		def initialize(data = nil)
			if( data == nil )
				super(1063)
			else
				super(1063, data)
			end
		end
	end

	class UndlyInstrumentPartySubIDType < Quickfix::IntField
		def UndlyInstrumentPartySubIDType.field
			return 1064
		end
		def initialize(data = nil)
			if( data == nil )
				super(1064)
			else
				super(1064, data)
			end
		end
	end

	class BidSwapPoints < Quickfix::DoubleField
		def BidSwapPoints.field
			return 1065
		end
		def initialize(data = nil)
			if( data == nil )
				super(1065)
			else
				super(1065, data)
			end
		end
	end

	class OfferSwapPoints < Quickfix::DoubleField
		def OfferSwapPoints.field
			return 1066
		end
		def initialize(data = nil)
			if( data == nil )
				super(1066)
			else
				super(1066, data)
			end
		end
	end

	class LegBidForwardPoints < Quickfix::DoubleField
		def LegBidForwardPoints.field
			return 1067
		end
		def initialize(data = nil)
			if( data == nil )
				super(1067)
			else
				super(1067, data)
			end
		end
	end

	class LegOfferForwardPoints < Quickfix::DoubleField
		def LegOfferForwardPoints.field
			return 1068
		end
		def initialize(data = nil)
			if( data == nil )
				super(1068)
			else
				super(1068, data)
			end
		end
	end

	class SwapPoints < Quickfix::DoubleField
		def SwapPoints.field
			return 1069
		end
		def initialize(data = nil)
			if( data == nil )
				super(1069)
			else
				super(1069, data)
			end
		end
	end

	class MDQuoteType < Quickfix::IntField
		def MDQuoteType.field
			return 1070
		end
		def initialize(data = nil)
			if( data == nil )
				super(1070)
			else
				super(1070, data)
			end
		end
	end

	class LastSwapPoints < Quickfix::DoubleField
		def LastSwapPoints.field
			return 1071
		end
		def initialize(data = nil)
			if( data == nil )
				super(1071)
			else
				super(1071, data)
			end
		end
	end

	class SideGrossTradeAmt < Quickfix::DoubleField
		def SideGrossTradeAmt.field
			return 1072
		end
		def initialize(data = nil)
			if( data == nil )
				super(1072)
			else
				super(1072, data)
			end
		end
	end

	class LegLastForwardPoints < Quickfix::DoubleField
		def LegLastForwardPoints.field
			return 1073
		end
		def initialize(data = nil)
			if( data == nil )
				super(1073)
			else
				super(1073, data)
			end
		end
	end

	class LegCalculatedCcyLastQty < Quickfix::DoubleField
		def LegCalculatedCcyLastQty.field
			return 1074
		end
		def initialize(data = nil)
			if( data == nil )
				super(1074)
			else
				super(1074, data)
			end
		end
	end

	class LegGrossTradeAmt < Quickfix::DoubleField
		def LegGrossTradeAmt.field
			return 1075
		end
		def initialize(data = nil)
			if( data == nil )
				super(1075)
			else
				super(1075, data)
			end
		end
	end

	class MaturityTime < Quickfix::StringField
		def MaturityTime.field
			return 1079
		end
		def initialize(data = nil)
			if( data == nil )
				super(1079)
			else
				super(1079, data)
			end
		end
	end

	class RefOrderID < Quickfix::StringField
		def RefOrderID.field
			return 1080
		end
		def initialize(data = nil)
			if( data == nil )
				super(1080)
			else
				super(1080, data)
			end
		end
	end

	class RefOrderIDSource < Quickfix::CharField
		def RefOrderIDSource.field
			return 1081
		end
		def initialize(data = nil)
			if( data == nil )
				super(1081)
			else
				super(1081, data)
			end
		end
	end

	class SecondaryDisplayQty < Quickfix::DoubleField
		def SecondaryDisplayQty.field
			return 1082
		end
		def initialize(data = nil)
			if( data == nil )
				super(1082)
			else
				super(1082, data)
			end
		end
	end

	class DisplayWhen < Quickfix::CharField
		def DisplayWhen.field
			return 1083
		end
		def initialize(data = nil)
			if( data == nil )
				super(1083)
			else
				super(1083, data)
			end
		end
	end

	class DisplayMethod < Quickfix::CharField
		def DisplayMethod.field
			return 1084
		end
		def initialize(data = nil)
			if( data == nil )
				super(1084)
			else
				super(1084, data)
			end
		end
	end

	class DisplayLowQty < Quickfix::DoubleField
		def DisplayLowQty.field
			return 1085
		end
		def initialize(data = nil)
			if( data == nil )
				super(1085)
			else
				super(1085, data)
			end
		end
	end

	class DisplayHighQty < Quickfix::DoubleField
		def DisplayHighQty.field
			return 1086
		end
		def initialize(data = nil)
			if( data == nil )
				super(1086)
			else
				super(1086, data)
			end
		end
	end

	class DisplayMinIncr < Quickfix::DoubleField
		def DisplayMinIncr.field
			return 1087
		end
		def initialize(data = nil)
			if( data == nil )
				super(1087)
			else
				super(1087, data)
			end
		end
	end

	class RefreshQty < Quickfix::DoubleField
		def RefreshQty.field
			return 1088
		end
		def initialize(data = nil)
			if( data == nil )
				super(1088)
			else
				super(1088, data)
			end
		end
	end

	class MatchIncrement < Quickfix::DoubleField
		def MatchIncrement.field
			return 1089
		end
		def initialize(data = nil)
			if( data == nil )
				super(1089)
			else
				super(1089, data)
			end
		end
	end

	class MaxPriceLevels < Quickfix::IntField
		def MaxPriceLevels.field
			return 1090
		end
		def initialize(data = nil)
			if( data == nil )
				super(1090)
			else
				super(1090, data)
			end
		end
	end

	class PreTradeAnonymity < Quickfix::BoolField
		def PreTradeAnonymity.field
			return 1091
		end
		def initialize(data = nil)
			if( data == nil )
				super(1091)
			else
				super(1091, data)
			end
		end
	end

	class PriceProtectionScope < Quickfix::CharField
		def PriceProtectionScope.field
			return 1092
		end
		def initialize(data = nil)
			if( data == nil )
				super(1092)
			else
				super(1092, data)
			end
		end
	end

	class LotType < Quickfix::CharField
		def LotType.field
			return 1093
		end
		def initialize(data = nil)
			if( data == nil )
				super(1093)
			else
				super(1093, data)
			end
		end
	end

	class PegPriceType < Quickfix::IntField
		def PegPriceType.field
			return 1094
		end
		def initialize(data = nil)
			if( data == nil )
				super(1094)
			else
				super(1094, data)
			end
		end
	end

	class PeggedRefPrice < Quickfix::DoubleField
		def PeggedRefPrice.field
			return 1095
		end
		def initialize(data = nil)
			if( data == nil )
				super(1095)
			else
				super(1095, data)
			end
		end
	end

	class PegSecurityIDSource < Quickfix::StringField
		def PegSecurityIDSource.field
			return 1096
		end
		def initialize(data = nil)
			if( data == nil )
				super(1096)
			else
				super(1096, data)
			end
		end
	end

	class PegSecurityID < Quickfix::StringField
		def PegSecurityID.field
			return 1097
		end
		def initialize(data = nil)
			if( data == nil )
				super(1097)
			else
				super(1097, data)
			end
		end
	end

	class PegSymbol < Quickfix::StringField
		def PegSymbol.field
			return 1098
		end
		def initialize(data = nil)
			if( data == nil )
				super(1098)
			else
				super(1098, data)
			end
		end
	end

	class PegSecurityDesc < Quickfix::StringField
		def PegSecurityDesc.field
			return 1099
		end
		def initialize(data = nil)
			if( data == nil )
				super(1099)
			else
				super(1099, data)
			end
		end
	end

	class TriggerType < Quickfix::CharField
		def TriggerType.field
			return 1100
		end
		def initialize(data = nil)
			if( data == nil )
				super(1100)
			else
				super(1100, data)
			end
		end
	end

	class TriggerAction < Quickfix::CharField
		def TriggerAction.field
			return 1101
		end
		def initialize(data = nil)
			if( data == nil )
				super(1101)
			else
				super(1101, data)
			end
		end
	end

	class TriggerPrice < Quickfix::DoubleField
		def TriggerPrice.field
			return 1102
		end
		def initialize(data = nil)
			if( data == nil )
				super(1102)
			else
				super(1102, data)
			end
		end
	end

	class TriggerSymbol < Quickfix::StringField
		def TriggerSymbol.field
			return 1103
		end
		def initialize(data = nil)
			if( data == nil )
				super(1103)
			else
				super(1103, data)
			end
		end
	end

	class TriggerSecurityID < Quickfix::StringField
		def TriggerSecurityID.field
			return 1104
		end
		def initialize(data = nil)
			if( data == nil )
				super(1104)
			else
				super(1104, data)
			end
		end
	end

	class TriggerSecurityIDSource < Quickfix::StringField
		def TriggerSecurityIDSource.field
			return 1105
		end
		def initialize(data = nil)
			if( data == nil )
				super(1105)
			else
				super(1105, data)
			end
		end
	end

	class TriggerSecurityDesc < Quickfix::StringField
		def TriggerSecurityDesc.field
			return 1106
		end
		def initialize(data = nil)
			if( data == nil )
				super(1106)
			else
				super(1106, data)
			end
		end
	end

	class TriggerPriceType < Quickfix::CharField
		def TriggerPriceType.field
			return 1107
		end
		def initialize(data = nil)
			if( data == nil )
				super(1107)
			else
				super(1107, data)
			end
		end
	end

	class TriggerPriceTypeScope < Quickfix::CharField
		def TriggerPriceTypeScope.field
			return 1108
		end
		def initialize(data = nil)
			if( data == nil )
				super(1108)
			else
				super(1108, data)
			end
		end
	end

	class TriggerPriceDirection < Quickfix::CharField
		def TriggerPriceDirection.field
			return 1109
		end
		def initialize(data = nil)
			if( data == nil )
				super(1109)
			else
				super(1109, data)
			end
		end
	end

	class TriggerNewPrice < Quickfix::DoubleField
		def TriggerNewPrice.field
			return 1110
		end
		def initialize(data = nil)
			if( data == nil )
				super(1110)
			else
				super(1110, data)
			end
		end
	end

	class TriggerOrderType < Quickfix::CharField
		def TriggerOrderType.field
			return 1111
		end
		def initialize(data = nil)
			if( data == nil )
				super(1111)
			else
				super(1111, data)
			end
		end
	end

	class TriggerNewQty < Quickfix::DoubleField
		def TriggerNewQty.field
			return 1112
		end
		def initialize(data = nil)
			if( data == nil )
				super(1112)
			else
				super(1112, data)
			end
		end
	end

	class TriggerTradingSessionID < Quickfix::StringField
		def TriggerTradingSessionID.field
			return 1113
		end
		def initialize(data = nil)
			if( data == nil )
				super(1113)
			else
				super(1113, data)
			end
		end
	end

	class TriggerTradingSessionSubID < Quickfix::StringField
		def TriggerTradingSessionSubID.field
			return 1114
		end
		def initialize(data = nil)
			if( data == nil )
				super(1114)
			else
				super(1114, data)
			end
		end
	end

	class OrderCategory < Quickfix::CharField
		def OrderCategory.field
			return 1115
		end
		def initialize(data = nil)
			if( data == nil )
				super(1115)
			else
				super(1115, data)
			end
		end
	end

	class NoRootPartyIDs < Quickfix::IntField
		def NoRootPartyIDs.field
			return 1116
		end
		def initialize(data = nil)
			if( data == nil )
				super(1116)
			else
				super(1116, data)
			end
		end
	end

	class RootPartyID < Quickfix::StringField
		def RootPartyID.field
			return 1117
		end
		def initialize(data = nil)
			if( data == nil )
				super(1117)
			else
				super(1117, data)
			end
		end
	end

	class RootPartyIDSource < Quickfix::CharField
		def RootPartyIDSource.field
			return 1118
		end
		def initialize(data = nil)
			if( data == nil )
				super(1118)
			else
				super(1118, data)
			end
		end
	end

	class RootPartyRole < Quickfix::IntField
		def RootPartyRole.field
			return 1119
		end
		def initialize(data = nil)
			if( data == nil )
				super(1119)
			else
				super(1119, data)
			end
		end
	end

	class NoRootPartySubIDs < Quickfix::IntField
		def NoRootPartySubIDs.field
			return 1120
		end
		def initialize(data = nil)
			if( data == nil )
				super(1120)
			else
				super(1120, data)
			end
		end
	end

	class RootPartySubID < Quickfix::StringField
		def RootPartySubID.field
			return 1121
		end
		def initialize(data = nil)
			if( data == nil )
				super(1121)
			else
				super(1121, data)
			end
		end
	end

	class RootPartySubIDType < Quickfix::IntField
		def RootPartySubIDType.field
			return 1122
		end
		def initialize(data = nil)
			if( data == nil )
				super(1122)
			else
				super(1122, data)
			end
		end
	end

	class TradeHandlingInstr < Quickfix::CharField
		def TradeHandlingInstr.field
			return 1123
		end
		def initialize(data = nil)
			if( data == nil )
				super(1123)
			else
				super(1123, data)
			end
		end
	end

	class OrigTradeHandlingInstr < Quickfix::CharField
		def OrigTradeHandlingInstr.field
			return 1124
		end
		def initialize(data = nil)
			if( data == nil )
				super(1124)
			else
				super(1124, data)
			end
		end
	end

	class OrigTradeDate < Quickfix::StringField
		def OrigTradeDate.field
			return 1125
		end
		def initialize(data = nil)
			if( data == nil )
				super(1125)
			else
				super(1125, data)
			end
		end
	end

	class OrigTradeID < Quickfix::StringField
		def OrigTradeID.field
			return 1126
		end
		def initialize(data = nil)
			if( data == nil )
				super(1126)
			else
				super(1126, data)
			end
		end
	end

	class OrigSecondaryTradeID < Quickfix::StringField
		def OrigSecondaryTradeID.field
			return 1127
		end
		def initialize(data = nil)
			if( data == nil )
				super(1127)
			else
				super(1127, data)
			end
		end
	end

	class TZTransactTime < Quickfix::StringField
		def TZTransactTime.field
			return 1132
		end
		def initialize(data = nil)
			if( data == nil )
				super(1132)
			else
				super(1132, data)
			end
		end
	end

	class ExDestinationIDSource < Quickfix::CharField
		def ExDestinationIDSource.field
			return 1133
		end
		def initialize(data = nil)
			if( data == nil )
				super(1133)
			else
				super(1133, data)
			end
		end
	end

	class ReportedPxDiff < Quickfix::BoolField
		def ReportedPxDiff.field
			return 1134
		end
		def initialize(data = nil)
			if( data == nil )
				super(1134)
			else
				super(1134, data)
			end
		end
	end

	class RptSys < Quickfix::StringField
		def RptSys.field
			return 1135
		end
		def initialize(data = nil)
			if( data == nil )
				super(1135)
			else
				super(1135, data)
			end
		end
	end

	class AllocClearingFeeIndicator < Quickfix::StringField
		def AllocClearingFeeIndicator.field
			return 1136
		end
		def initialize(data = nil)
			if( data == nil )
				super(1136)
			else
				super(1136, data)
			end
		end
	end

	class DisplayQty < Quickfix::DoubleField
		def DisplayQty.field
			return 1138
		end
		def initialize(data = nil)
			if( data == nil )
				super(1138)
			else
				super(1138, data)
			end
		end
	end

	class ExchangeSpecialInstructions < Quickfix::StringField
		def ExchangeSpecialInstructions.field
			return 1139
		end
		def initialize(data = nil)
			if( data == nil )
				super(1139)
			else
				super(1139, data)
			end
		end
	end

	class ExpirationQtyType < Quickfix::IntField
		def ExpirationQtyType.field
			return 982
		end
		def initialize(data = nil)
			if( data == nil )
				super(982)
			else
				super(982, data)
			end
		end
	end

	class UnitOfMeasure < Quickfix::StringField
		def UnitOfMeasure.field
			return 996
		end
		def initialize(data = nil)
			if( data == nil )
				super(996)
			else
				super(996, data)
			end
		end
	end

	class UnderlyingUnitOfMeasure < Quickfix::StringField
		def UnderlyingUnitOfMeasure.field
			return 998
		end
		def initialize(data = nil)
			if( data == nil )
				super(998)
			else
				super(998, data)
			end
		end
	end

	class LegUnitOfMeasure < Quickfix::StringField
		def LegUnitOfMeasure.field
			return 999
		end
		def initialize(data = nil)
			if( data == nil )
				super(999)
			else
				super(999, data)
			end
		end
	end

	class UnderlyingMaturityTime < Quickfix::StringField
		def UnderlyingMaturityTime.field
			return 1213
		end
		def initialize(data = nil)
			if( data == nil )
				super(1213)
			else
				super(1213, data)
			end
		end
	end

	class LegMaturityTime < Quickfix::StringField
		def LegMaturityTime.field
			return 1212
		end
		def initialize(data = nil)
			if( data == nil )
				super(1212)
			else
				super(1212, data)
			end
		end
	end

	class MaxTradeVol < Quickfix::DoubleField
		def MaxTradeVol.field
			return 1140
		end
		def initialize(data = nil)
			if( data == nil )
				super(1140)
			else
				super(1140, data)
			end
		end
	end

	class NoMDFeedTypes < Quickfix::IntField
		def NoMDFeedTypes.field
			return 1141
		end
		def initialize(data = nil)
			if( data == nil )
				super(1141)
			else
				super(1141, data)
			end
		end
	end

	class MatchAlgorithm < Quickfix::StringField
		def MatchAlgorithm.field
			return 1142
		end
		def initialize(data = nil)
			if( data == nil )
				super(1142)
			else
				super(1142, data)
			end
		end
	end

	class MaxPriceVariation < Quickfix::DoubleField
		def MaxPriceVariation.field
			return 1143
		end
		def initialize(data = nil)
			if( data == nil )
				super(1143)
			else
				super(1143, data)
			end
		end
	end

	class ImpliedMarketIndicator < Quickfix::IntField
		def ImpliedMarketIndicator.field
			return 1144
		end
		def initialize(data = nil)
			if( data == nil )
				super(1144)
			else
				super(1144, data)
			end
		end
	end

	class EventTime < Quickfix::UtcTimeStampField
		def EventTime.field
			return 1145
		end
		def initialize(data = nil)
			if( data == nil )
				super(1145)
			else
				super(1145, data)
			end
		end
	end

	class MinPriceIncrementAmount < Quickfix::DoubleField
		def MinPriceIncrementAmount.field
			return 1146
		end
		def initialize(data = nil)
			if( data == nil )
				super(1146)
			else
				super(1146, data)
			end
		end
	end

	class UnitOfMeasureQty < Quickfix::DoubleField
		def UnitOfMeasureQty.field
			return 1147
		end
		def initialize(data = nil)
			if( data == nil )
				super(1147)
			else
				super(1147, data)
			end
		end
	end

	class LowLimitPrice < Quickfix::DoubleField
		def LowLimitPrice.field
			return 1148
		end
		def initialize(data = nil)
			if( data == nil )
				super(1148)
			else
				super(1148, data)
			end
		end
	end

	class HighLimitPrice < Quickfix::DoubleField
		def HighLimitPrice.field
			return 1149
		end
		def initialize(data = nil)
			if( data == nil )
				super(1149)
			else
				super(1149, data)
			end
		end
	end

	class TradingReferencePrice < Quickfix::DoubleField
		def TradingReferencePrice.field
			return 1150
		end
		def initialize(data = nil)
			if( data == nil )
				super(1150)
			else
				super(1150, data)
			end
		end
	end

	class SecurityGroup < Quickfix::StringField
		def SecurityGroup.field
			return 1151
		end
		def initialize(data = nil)
			if( data == nil )
				super(1151)
			else
				super(1151, data)
			end
		end
	end

	class LegNumber < Quickfix::IntField
		def LegNumber.field
			return 1152
		end
		def initialize(data = nil)
			if( data == nil )
				super(1152)
			else
				super(1152, data)
			end
		end
	end

	class SettlementCycleNo < Quickfix::IntField
		def SettlementCycleNo.field
			return 1153
		end
		def initialize(data = nil)
			if( data == nil )
				super(1153)
			else
				super(1153, data)
			end
		end
	end

	class SideCurrency < Quickfix::StringField
		def SideCurrency.field
			return 1154
		end
		def initialize(data = nil)
			if( data == nil )
				super(1154)
			else
				super(1154, data)
			end
		end
	end

	class SideSettlCurrency < Quickfix::StringField
		def SideSettlCurrency.field
			return 1155
		end
		def initialize(data = nil)
			if( data == nil )
				super(1155)
			else
				super(1155, data)
			end
		end
	end

	class CcyAmt < Quickfix::DoubleField
		def CcyAmt.field
			return 1157
		end
		def initialize(data = nil)
			if( data == nil )
				super(1157)
			else
				super(1157, data)
			end
		end
	end

	class NoSettlDetails < Quickfix::IntField
		def NoSettlDetails.field
			return 1158
		end
		def initialize(data = nil)
			if( data == nil )
				super(1158)
			else
				super(1158, data)
			end
		end
	end

	class SettlObligMode < Quickfix::IntField
		def SettlObligMode.field
			return 1159
		end
		def initialize(data = nil)
			if( data == nil )
				super(1159)
			else
				super(1159, data)
			end
		end
	end

	class SettlObligMsgID < Quickfix::StringField
		def SettlObligMsgID.field
			return 1160
		end
		def initialize(data = nil)
			if( data == nil )
				super(1160)
			else
				super(1160, data)
			end
		end
	end

	class SettlObligID < Quickfix::StringField
		def SettlObligID.field
			return 1161
		end
		def initialize(data = nil)
			if( data == nil )
				super(1161)
			else
				super(1161, data)
			end
		end
	end

	class SettlObligTransType < Quickfix::CharField
		def SettlObligTransType.field
			return 1162
		end
		def initialize(data = nil)
			if( data == nil )
				super(1162)
			else
				super(1162, data)
			end
		end
	end

	class SettlObligRefID < Quickfix::StringField
		def SettlObligRefID.field
			return 1163
		end
		def initialize(data = nil)
			if( data == nil )
				super(1163)
			else
				super(1163, data)
			end
		end
	end

	class SettlObligSource < Quickfix::CharField
		def SettlObligSource.field
			return 1164
		end
		def initialize(data = nil)
			if( data == nil )
				super(1164)
			else
				super(1164, data)
			end
		end
	end

	class NoSettlOblig < Quickfix::IntField
		def NoSettlOblig.field
			return 1165
		end
		def initialize(data = nil)
			if( data == nil )
				super(1165)
			else
				super(1165, data)
			end
		end
	end

	class QuoteMsgID < Quickfix::StringField
		def QuoteMsgID.field
			return 1166
		end
		def initialize(data = nil)
			if( data == nil )
				super(1166)
			else
				super(1166, data)
			end
		end
	end

	class QuoteEntryStatus < Quickfix::IntField
		def QuoteEntryStatus.field
			return 1167
		end
		def initialize(data = nil)
			if( data == nil )
				super(1167)
			else
				super(1167, data)
			end
		end
	end

	class TotNoCxldQuotes < Quickfix::IntField
		def TotNoCxldQuotes.field
			return 1168
		end
		def initialize(data = nil)
			if( data == nil )
				super(1168)
			else
				super(1168, data)
			end
		end
	end

	class TotNoAccQuotes < Quickfix::IntField
		def TotNoAccQuotes.field
			return 1169
		end
		def initialize(data = nil)
			if( data == nil )
				super(1169)
			else
				super(1169, data)
			end
		end
	end

	class TotNoRejQuotes < Quickfix::IntField
		def TotNoRejQuotes.field
			return 1170
		end
		def initialize(data = nil)
			if( data == nil )
				super(1170)
			else
				super(1170, data)
			end
		end
	end

	class PrivateQuote < Quickfix::BoolField
		def PrivateQuote.field
			return 1171
		end
		def initialize(data = nil)
			if( data == nil )
				super(1171)
			else
				super(1171, data)
			end
		end
	end

	class RespondentType < Quickfix::IntField
		def RespondentType.field
			return 1172
		end
		def initialize(data = nil)
			if( data == nil )
				super(1172)
			else
				super(1172, data)
			end
		end
	end

	class MDSubBookType < Quickfix::IntField
		def MDSubBookType.field
			return 1173
		end
		def initialize(data = nil)
			if( data == nil )
				super(1173)
			else
				super(1173, data)
			end
		end
	end

	class SecurityTradingEvent < Quickfix::IntField
		def SecurityTradingEvent.field
			return 1174
		end
		def initialize(data = nil)
			if( data == nil )
				super(1174)
			else
				super(1174, data)
			end
		end
	end

	class NoStatsIndicators < Quickfix::IntField
		def NoStatsIndicators.field
			return 1175
		end
		def initialize(data = nil)
			if( data == nil )
				super(1175)
			else
				super(1175, data)
			end
		end
	end

	class StatsType < Quickfix::IntField
		def StatsType.field
			return 1176
		end
		def initialize(data = nil)
			if( data == nil )
				super(1176)
			else
				super(1176, data)
			end
		end
	end

	class NoOfSecSizes < Quickfix::IntField
		def NoOfSecSizes.field
			return 1177
		end
		def initialize(data = nil)
			if( data == nil )
				super(1177)
			else
				super(1177, data)
			end
		end
	end

	class MDSecSizeType < Quickfix::IntField
		def MDSecSizeType.field
			return 1178
		end
		def initialize(data = nil)
			if( data == nil )
				super(1178)
			else
				super(1178, data)
			end
		end
	end

	class MDSecSize < Quickfix::DoubleField
		def MDSecSize.field
			return 1179
		end
		def initialize(data = nil)
			if( data == nil )
				super(1179)
			else
				super(1179, data)
			end
		end
	end

	class ApplID < Quickfix::StringField
		def ApplID.field
			return 1180
		end
		def initialize(data = nil)
			if( data == nil )
				super(1180)
			else
				super(1180, data)
			end
		end
	end

	class ApplSeqNum < Quickfix::IntField
		def ApplSeqNum.field
			return 1181
		end
		def initialize(data = nil)
			if( data == nil )
				super(1181)
			else
				super(1181, data)
			end
		end
	end

	class ApplBegSeqNum < Quickfix::IntField
		def ApplBegSeqNum.field
			return 1182
		end
		def initialize(data = nil)
			if( data == nil )
				super(1182)
			else
				super(1182, data)
			end
		end
	end

	class ApplEndSeqNum < Quickfix::IntField
		def ApplEndSeqNum.field
			return 1183
		end
		def initialize(data = nil)
			if( data == nil )
				super(1183)
			else
				super(1183, data)
			end
		end
	end

	class SecurityXMLLen < Quickfix::IntField
		def SecurityXMLLen.field
			return 1184
		end
		def initialize(data = nil)
			if( data == nil )
				super(1184)
			else
				super(1184, data)
			end
		end
	end

	class SecurityXML < Quickfix::StringField
		def SecurityXML.field
			return 1185
		end
		def initialize(data = nil)
			if( data == nil )
				super(1185)
			else
				super(1185, data)
			end
		end
	end

	class SecurityXMLSchema < Quickfix::StringField
		def SecurityXMLSchema.field
			return 1186
		end
		def initialize(data = nil)
			if( data == nil )
				super(1186)
			else
				super(1186, data)
			end
		end
	end

	class RefreshIndicator < Quickfix::BoolField
		def RefreshIndicator.field
			return 1187
		end
		def initialize(data = nil)
			if( data == nil )
				super(1187)
			else
				super(1187, data)
			end
		end
	end

	class Volatility < Quickfix::DoubleField
		def Volatility.field
			return 1188
		end
		def initialize(data = nil)
			if( data == nil )
				super(1188)
			else
				super(1188, data)
			end
		end
	end

	class TimeToExpiration < Quickfix::DoubleField
		def TimeToExpiration.field
			return 1189
		end
		def initialize(data = nil)
			if( data == nil )
				super(1189)
			else
				super(1189, data)
			end
		end
	end

	class RiskFreeRate < Quickfix::DoubleField
		def RiskFreeRate.field
			return 1190
		end
		def initialize(data = nil)
			if( data == nil )
				super(1190)
			else
				super(1190, data)
			end
		end
	end

	class PriceUnitOfMeasure < Quickfix::StringField
		def PriceUnitOfMeasure.field
			return 1191
		end
		def initialize(data = nil)
			if( data == nil )
				super(1191)
			else
				super(1191, data)
			end
		end
	end

	class PriceUnitOfMeasureQty < Quickfix::DoubleField
		def PriceUnitOfMeasureQty.field
			return 1192
		end
		def initialize(data = nil)
			if( data == nil )
				super(1192)
			else
				super(1192, data)
			end
		end
	end

	class SettlMethod < Quickfix::StringField
		def SettlMethod.field
			return 1193
		end
		def initialize(data = nil)
			if( data == nil )
				super(1193)
			else
				super(1193, data)
			end
		end
	end

	class ExerciseStyle < Quickfix::IntField
		def ExerciseStyle.field
			return 1194
		end
		def initialize(data = nil)
			if( data == nil )
				super(1194)
			else
				super(1194, data)
			end
		end
	end

	class UnderlyingExerciseStyle < Quickfix::IntField
		def UnderlyingExerciseStyle.field
			return 1419
		end
		def initialize(data = nil)
			if( data == nil )
				super(1419)
			else
				super(1419, data)
			end
		end
	end

	class LegExerciseStyle < Quickfix::IntField
		def LegExerciseStyle.field
			return 1420
		end
		def initialize(data = nil)
			if( data == nil )
				super(1420)
			else
				super(1420, data)
			end
		end
	end

	class OptPayAmount < Quickfix::DoubleField
		def OptPayAmount.field
			return 1195
		end
		def initialize(data = nil)
			if( data == nil )
				super(1195)
			else
				super(1195, data)
			end
		end
	end

	class PriceQuoteMethod < Quickfix::StringField
		def PriceQuoteMethod.field
			return 1196
		end
		def initialize(data = nil)
			if( data == nil )
				super(1196)
			else
				super(1196, data)
			end
		end
	end

	class FuturesValuationMethod < Quickfix::StringField
		def FuturesValuationMethod.field
			return 1197
		end
		def initialize(data = nil)
			if( data == nil )
				super(1197)
			else
				super(1197, data)
			end
		end
	end

	class ListMethod < Quickfix::IntField
		def ListMethod.field
			return 1198
		end
		def initialize(data = nil)
			if( data == nil )
				super(1198)
			else
				super(1198, data)
			end
		end
	end

	class CapPrice < Quickfix::DoubleField
		def CapPrice.field
			return 1199
		end
		def initialize(data = nil)
			if( data == nil )
				super(1199)
			else
				super(1199, data)
			end
		end
	end

	class FloorPrice < Quickfix::DoubleField
		def FloorPrice.field
			return 1200
		end
		def initialize(data = nil)
			if( data == nil )
				super(1200)
			else
				super(1200, data)
			end
		end
	end

	class NoStrikeRules < Quickfix::IntField
		def NoStrikeRules.field
			return 1201
		end
		def initialize(data = nil)
			if( data == nil )
				super(1201)
			else
				super(1201, data)
			end
		end
	end

	class StartStrikePxRange < Quickfix::DoubleField
		def StartStrikePxRange.field
			return 1202
		end
		def initialize(data = nil)
			if( data == nil )
				super(1202)
			else
				super(1202, data)
			end
		end
	end

	class EndStrikePxRange < Quickfix::DoubleField
		def EndStrikePxRange.field
			return 1203
		end
		def initialize(data = nil)
			if( data == nil )
				super(1203)
			else
				super(1203, data)
			end
		end
	end

	class StrikeIncrement < Quickfix::DoubleField
		def StrikeIncrement.field
			return 1204
		end
		def initialize(data = nil)
			if( data == nil )
				super(1204)
			else
				super(1204, data)
			end
		end
	end

	class NoTickRules < Quickfix::IntField
		def NoTickRules.field
			return 1205
		end
		def initialize(data = nil)
			if( data == nil )
				super(1205)
			else
				super(1205, data)
			end
		end
	end

	class StartTickPriceRange < Quickfix::DoubleField
		def StartTickPriceRange.field
			return 1206
		end
		def initialize(data = nil)
			if( data == nil )
				super(1206)
			else
				super(1206, data)
			end
		end
	end

	class EndTickPriceRange < Quickfix::DoubleField
		def EndTickPriceRange.field
			return 1207
		end
		def initialize(data = nil)
			if( data == nil )
				super(1207)
			else
				super(1207, data)
			end
		end
	end

	class TickIncrement < Quickfix::DoubleField
		def TickIncrement.field
			return 1208
		end
		def initialize(data = nil)
			if( data == nil )
				super(1208)
			else
				super(1208, data)
			end
		end
	end

	class TickRuleType < Quickfix::IntField
		def TickRuleType.field
			return 1209
		end
		def initialize(data = nil)
			if( data == nil )
				super(1209)
			else
				super(1209, data)
			end
		end
	end

	class NestedInstrAttribType < Quickfix::IntField
		def NestedInstrAttribType.field
			return 1210
		end
		def initialize(data = nil)
			if( data == nil )
				super(1210)
			else
				super(1210, data)
			end
		end
	end

	class NestedInstrAttribValue < Quickfix::StringField
		def NestedInstrAttribValue.field
			return 1211
		end
		def initialize(data = nil)
			if( data == nil )
				super(1211)
			else
				super(1211, data)
			end
		end
	end

	class DerivativeSymbol < Quickfix::StringField
		def DerivativeSymbol.field
			return 1214
		end
		def initialize(data = nil)
			if( data == nil )
				super(1214)
			else
				super(1214, data)
			end
		end
	end

	class DerivativeSymbolSfx < Quickfix::StringField
		def DerivativeSymbolSfx.field
			return 1215
		end
		def initialize(data = nil)
			if( data == nil )
				super(1215)
			else
				super(1215, data)
			end
		end
	end

	class DerivativeSecurityID < Quickfix::StringField
		def DerivativeSecurityID.field
			return 1216
		end
		def initialize(data = nil)
			if( data == nil )
				super(1216)
			else
				super(1216, data)
			end
		end
	end

	class DerivativeSecurityIDSource < Quickfix::StringField
		def DerivativeSecurityIDSource.field
			return 1217
		end
		def initialize(data = nil)
			if( data == nil )
				super(1217)
			else
				super(1217, data)
			end
		end
	end

	class NoDerivativeSecurityAltID < Quickfix::IntField
		def NoDerivativeSecurityAltID.field
			return 1218
		end
		def initialize(data = nil)
			if( data == nil )
				super(1218)
			else
				super(1218, data)
			end
		end
	end

	class DerivativeSecurityAltID < Quickfix::StringField
		def DerivativeSecurityAltID.field
			return 1219
		end
		def initialize(data = nil)
			if( data == nil )
				super(1219)
			else
				super(1219, data)
			end
		end
	end

	class DerivativeSecurityAltIDSource < Quickfix::StringField
		def DerivativeSecurityAltIDSource.field
			return 1220
		end
		def initialize(data = nil)
			if( data == nil )
				super(1220)
			else
				super(1220, data)
			end
		end
	end

	class SecondaryLowLimitPrice < Quickfix::DoubleField
		def SecondaryLowLimitPrice.field
			return 1221
		end
		def initialize(data = nil)
			if( data == nil )
				super(1221)
			else
				super(1221, data)
			end
		end
	end

	class SecondaryHighLimitPrice < Quickfix::DoubleField
		def SecondaryHighLimitPrice.field
			return 1230
		end
		def initialize(data = nil)
			if( data == nil )
				super(1230)
			else
				super(1230, data)
			end
		end
	end

	class MaturityRuleID < Quickfix::StringField
		def MaturityRuleID.field
			return 1222
		end
		def initialize(data = nil)
			if( data == nil )
				super(1222)
			else
				super(1222, data)
			end
		end
	end

	class StrikeRuleID < Quickfix::StringField
		def StrikeRuleID.field
			return 1223
		end
		def initialize(data = nil)
			if( data == nil )
				super(1223)
			else
				super(1223, data)
			end
		end
	end

	class DerivativeOptPayAmount < Quickfix::DoubleField
		def DerivativeOptPayAmount.field
			return 1225
		end
		def initialize(data = nil)
			if( data == nil )
				super(1225)
			else
				super(1225, data)
			end
		end
	end

	class EndMaturityMonthYear < Quickfix::StringField
		def EndMaturityMonthYear.field
			return 1226
		end
		def initialize(data = nil)
			if( data == nil )
				super(1226)
			else
				super(1226, data)
			end
		end
	end

	class ProductComplex < Quickfix::StringField
		def ProductComplex.field
			return 1227
		end
		def initialize(data = nil)
			if( data == nil )
				super(1227)
			else
				super(1227, data)
			end
		end
	end

	class DerivativeProductComplex < Quickfix::StringField
		def DerivativeProductComplex.field
			return 1228
		end
		def initialize(data = nil)
			if( data == nil )
				super(1228)
			else
				super(1228, data)
			end
		end
	end

	class MaturityMonthYearIncrement < Quickfix::IntField
		def MaturityMonthYearIncrement.field
			return 1229
		end
		def initialize(data = nil)
			if( data == nil )
				super(1229)
			else
				super(1229, data)
			end
		end
	end

	class MinLotSize < Quickfix::DoubleField
		def MinLotSize.field
			return 1231
		end
		def initialize(data = nil)
			if( data == nil )
				super(1231)
			else
				super(1231, data)
			end
		end
	end

	class NoExecInstRules < Quickfix::IntField
		def NoExecInstRules.field
			return 1232
		end
		def initialize(data = nil)
			if( data == nil )
				super(1232)
			else
				super(1232, data)
			end
		end
	end

	class NoLotTypeRules < Quickfix::IntField
		def NoLotTypeRules.field
			return 1234
		end
		def initialize(data = nil)
			if( data == nil )
				super(1234)
			else
				super(1234, data)
			end
		end
	end

	class NoMatchRules < Quickfix::IntField
		def NoMatchRules.field
			return 1235
		end
		def initialize(data = nil)
			if( data == nil )
				super(1235)
			else
				super(1235, data)
			end
		end
	end

	class NoMaturityRules < Quickfix::IntField
		def NoMaturityRules.field
			return 1236
		end
		def initialize(data = nil)
			if( data == nil )
				super(1236)
			else
				super(1236, data)
			end
		end
	end

	class NoOrdTypeRules < Quickfix::IntField
		def NoOrdTypeRules.field
			return 1237
		end
		def initialize(data = nil)
			if( data == nil )
				super(1237)
			else
				super(1237, data)
			end
		end
	end

	class NoTimeInForceRules < Quickfix::IntField
		def NoTimeInForceRules.field
			return 1239
		end
		def initialize(data = nil)
			if( data == nil )
				super(1239)
			else
				super(1239, data)
			end
		end
	end

	class SecondaryTradingReferencePrice < Quickfix::DoubleField
		def SecondaryTradingReferencePrice.field
			return 1240
		end
		def initialize(data = nil)
			if( data == nil )
				super(1240)
			else
				super(1240, data)
			end
		end
	end

	class StartMaturityMonthYear < Quickfix::StringField
		def StartMaturityMonthYear.field
			return 1241
		end
		def initialize(data = nil)
			if( data == nil )
				super(1241)
			else
				super(1241, data)
			end
		end
	end

	class FlexProductEligibilityIndicator < Quickfix::BoolField
		def FlexProductEligibilityIndicator.field
			return 1242
		end
		def initialize(data = nil)
			if( data == nil )
				super(1242)
			else
				super(1242, data)
			end
		end
	end

	class DerivFlexProductEligibilityIndicator < Quickfix::BoolField
		def DerivFlexProductEligibilityIndicator.field
			return 1243
		end
		def initialize(data = nil)
			if( data == nil )
				super(1243)
			else
				super(1243, data)
			end
		end
	end

	class FlexibleIndicator < Quickfix::BoolField
		def FlexibleIndicator.field
			return 1244
		end
		def initialize(data = nil)
			if( data == nil )
				super(1244)
			else
				super(1244, data)
			end
		end
	end

	class TradingCurrency < Quickfix::StringField
		def TradingCurrency.field
			return 1245
		end
		def initialize(data = nil)
			if( data == nil )
				super(1245)
			else
				super(1245, data)
			end
		end
	end

	class DerivativeProduct < Quickfix::IntField
		def DerivativeProduct.field
			return 1246
		end
		def initialize(data = nil)
			if( data == nil )
				super(1246)
			else
				super(1246, data)
			end
		end
	end

	class DerivativeSecurityGroup < Quickfix::StringField
		def DerivativeSecurityGroup.field
			return 1247
		end
		def initialize(data = nil)
			if( data == nil )
				super(1247)
			else
				super(1247, data)
			end
		end
	end

	class DerivativeCFICode < Quickfix::StringField
		def DerivativeCFICode.field
			return 1248
		end
		def initialize(data = nil)
			if( data == nil )
				super(1248)
			else
				super(1248, data)
			end
		end
	end

	class DerivativeSecurityType < Quickfix::StringField
		def DerivativeSecurityType.field
			return 1249
		end
		def initialize(data = nil)
			if( data == nil )
				super(1249)
			else
				super(1249, data)
			end
		end
	end

	class DerivativeSecuritySubType < Quickfix::StringField
		def DerivativeSecuritySubType.field
			return 1250
		end
		def initialize(data = nil)
			if( data == nil )
				super(1250)
			else
				super(1250, data)
			end
		end
	end

	class DerivativeMaturityMonthYear < Quickfix::StringField
		def DerivativeMaturityMonthYear.field
			return 1251
		end
		def initialize(data = nil)
			if( data == nil )
				super(1251)
			else
				super(1251, data)
			end
		end
	end

	class DerivativeMaturityDate < Quickfix::StringField
		def DerivativeMaturityDate.field
			return 1252
		end
		def initialize(data = nil)
			if( data == nil )
				super(1252)
			else
				super(1252, data)
			end
		end
	end

	class DerivativeMaturityTime < Quickfix::StringField
		def DerivativeMaturityTime.field
			return 1253
		end
		def initialize(data = nil)
			if( data == nil )
				super(1253)
			else
				super(1253, data)
			end
		end
	end

	class DerivativeSettleOnOpenFlag < Quickfix::StringField
		def DerivativeSettleOnOpenFlag.field
			return 1254
		end
		def initialize(data = nil)
			if( data == nil )
				super(1254)
			else
				super(1254, data)
			end
		end
	end

	class DerivativeInstrmtAssignmentMethod < Quickfix::CharField
		def DerivativeInstrmtAssignmentMethod.field
			return 1255
		end
		def initialize(data = nil)
			if( data == nil )
				super(1255)
			else
				super(1255, data)
			end
		end
	end

	class DerivativeSecurityStatus < Quickfix::StringField
		def DerivativeSecurityStatus.field
			return 1256
		end
		def initialize(data = nil)
			if( data == nil )
				super(1256)
			else
				super(1256, data)
			end
		end
	end

	class DerivativeInstrRegistry < Quickfix::StringField
		def DerivativeInstrRegistry.field
			return 1257
		end
		def initialize(data = nil)
			if( data == nil )
				super(1257)
			else
				super(1257, data)
			end
		end
	end

	class DerivativeCountryOfIssue < Quickfix::StringField
		def DerivativeCountryOfIssue.field
			return 1258
		end
		def initialize(data = nil)
			if( data == nil )
				super(1258)
			else
				super(1258, data)
			end
		end
	end

	class DerivativeStateOrProvinceOfIssue < Quickfix::StringField
		def DerivativeStateOrProvinceOfIssue.field
			return 1259
		end
		def initialize(data = nil)
			if( data == nil )
				super(1259)
			else
				super(1259, data)
			end
		end
	end

	class DerivativeLocaleOfIssue < Quickfix::StringField
		def DerivativeLocaleOfIssue.field
			return 1260
		end
		def initialize(data = nil)
			if( data == nil )
				super(1260)
			else
				super(1260, data)
			end
		end
	end

	class DerivativeStrikePrice < Quickfix::DoubleField
		def DerivativeStrikePrice.field
			return 1261
		end
		def initialize(data = nil)
			if( data == nil )
				super(1261)
			else
				super(1261, data)
			end
		end
	end

	class DerivativeStrikeCurrency < Quickfix::StringField
		def DerivativeStrikeCurrency.field
			return 1262
		end
		def initialize(data = nil)
			if( data == nil )
				super(1262)
			else
				super(1262, data)
			end
		end
	end

	class DerivativeStrikeMultiplier < Quickfix::DoubleField
		def DerivativeStrikeMultiplier.field
			return 1263
		end
		def initialize(data = nil)
			if( data == nil )
				super(1263)
			else
				super(1263, data)
			end
		end
	end

	class DerivativeStrikeValue < Quickfix::DoubleField
		def DerivativeStrikeValue.field
			return 1264
		end
		def initialize(data = nil)
			if( data == nil )
				super(1264)
			else
				super(1264, data)
			end
		end
	end

	class DerivativeOptAttribute < Quickfix::CharField
		def DerivativeOptAttribute.field
			return 1265
		end
		def initialize(data = nil)
			if( data == nil )
				super(1265)
			else
				super(1265, data)
			end
		end
	end

	class DerivativeContractMultiplier < Quickfix::DoubleField
		def DerivativeContractMultiplier.field
			return 1266
		end
		def initialize(data = nil)
			if( data == nil )
				super(1266)
			else
				super(1266, data)
			end
		end
	end

	class DerivativeMinPriceIncrement < Quickfix::DoubleField
		def DerivativeMinPriceIncrement.field
			return 1267
		end
		def initialize(data = nil)
			if( data == nil )
				super(1267)
			else
				super(1267, data)
			end
		end
	end

	class DerivativeMinPriceIncrementAmount < Quickfix::DoubleField
		def DerivativeMinPriceIncrementAmount.field
			return 1268
		end
		def initialize(data = nil)
			if( data == nil )
				super(1268)
			else
				super(1268, data)
			end
		end
	end

	class DerivativeUnitOfMeasure < Quickfix::StringField
		def DerivativeUnitOfMeasure.field
			return 1269
		end
		def initialize(data = nil)
			if( data == nil )
				super(1269)
			else
				super(1269, data)
			end
		end
	end

	class DerivativeUnitOfMeasureQty < Quickfix::DoubleField
		def DerivativeUnitOfMeasureQty.field
			return 1270
		end
		def initialize(data = nil)
			if( data == nil )
				super(1270)
			else
				super(1270, data)
			end
		end
	end

	class DerivativeTimeUnit < Quickfix::StringField
		def DerivativeTimeUnit.field
			return 1271
		end
		def initialize(data = nil)
			if( data == nil )
				super(1271)
			else
				super(1271, data)
			end
		end
	end

	class DerivativeSecurityExchange < Quickfix::StringField
		def DerivativeSecurityExchange.field
			return 1272
		end
		def initialize(data = nil)
			if( data == nil )
				super(1272)
			else
				super(1272, data)
			end
		end
	end

	class DerivativePositionLimit < Quickfix::IntField
		def DerivativePositionLimit.field
			return 1273
		end
		def initialize(data = nil)
			if( data == nil )
				super(1273)
			else
				super(1273, data)
			end
		end
	end

	class DerivativeNTPositionLimit < Quickfix::IntField
		def DerivativeNTPositionLimit.field
			return 1274
		end
		def initialize(data = nil)
			if( data == nil )
				super(1274)
			else
				super(1274, data)
			end
		end
	end

	class DerivativeIssuer < Quickfix::StringField
		def DerivativeIssuer.field
			return 1275
		end
		def initialize(data = nil)
			if( data == nil )
				super(1275)
			else
				super(1275, data)
			end
		end
	end

	class DerivativeIssueDate < Quickfix::StringField
		def DerivativeIssueDate.field
			return 1276
		end
		def initialize(data = nil)
			if( data == nil )
				super(1276)
			else
				super(1276, data)
			end
		end
	end

	class DerivativeEncodedIssuerLen < Quickfix::IntField
		def DerivativeEncodedIssuerLen.field
			return 1277
		end
		def initialize(data = nil)
			if( data == nil )
				super(1277)
			else
				super(1277, data)
			end
		end
	end

	class DerivativeEncodedIssuer < Quickfix::StringField
		def DerivativeEncodedIssuer.field
			return 1278
		end
		def initialize(data = nil)
			if( data == nil )
				super(1278)
			else
				super(1278, data)
			end
		end
	end

	class DerivativeSecurityDesc < Quickfix::StringField
		def DerivativeSecurityDesc.field
			return 1279
		end
		def initialize(data = nil)
			if( data == nil )
				super(1279)
			else
				super(1279, data)
			end
		end
	end

	class DerivativeEncodedSecurityDescLen < Quickfix::IntField
		def DerivativeEncodedSecurityDescLen.field
			return 1280
		end
		def initialize(data = nil)
			if( data == nil )
				super(1280)
			else
				super(1280, data)
			end
		end
	end

	class DerivativeEncodedSecurityDesc < Quickfix::StringField
		def DerivativeEncodedSecurityDesc.field
			return 1281
		end
		def initialize(data = nil)
			if( data == nil )
				super(1281)
			else
				super(1281, data)
			end
		end
	end

	class DerivativeSecurityXMLLen < Quickfix::IntField
		def DerivativeSecurityXMLLen.field
			return 1282
		end
		def initialize(data = nil)
			if( data == nil )
				super(1282)
			else
				super(1282, data)
			end
		end
	end

	class DerivativeSecurityXML < Quickfix::StringField
		def DerivativeSecurityXML.field
			return 1283
		end
		def initialize(data = nil)
			if( data == nil )
				super(1283)
			else
				super(1283, data)
			end
		end
	end

	class DerivativeSecurityXMLSchema < Quickfix::StringField
		def DerivativeSecurityXMLSchema.field
			return 1284
		end
		def initialize(data = nil)
			if( data == nil )
				super(1284)
			else
				super(1284, data)
			end
		end
	end

	class DerivativeContractSettlMonth < Quickfix::StringField
		def DerivativeContractSettlMonth.field
			return 1285
		end
		def initialize(data = nil)
			if( data == nil )
				super(1285)
			else
				super(1285, data)
			end
		end
	end

	class NoDerivativeEvents < Quickfix::IntField
		def NoDerivativeEvents.field
			return 1286
		end
		def initialize(data = nil)
			if( data == nil )
				super(1286)
			else
				super(1286, data)
			end
		end
	end

	class DerivativeEventType < Quickfix::IntField
		def DerivativeEventType.field
			return 1287
		end
		def initialize(data = nil)
			if( data == nil )
				super(1287)
			else
				super(1287, data)
			end
		end
	end

	class DerivativeEventDate < Quickfix::StringField
		def DerivativeEventDate.field
			return 1288
		end
		def initialize(data = nil)
			if( data == nil )
				super(1288)
			else
				super(1288, data)
			end
		end
	end

	class DerivativeEventTime < Quickfix::UtcTimeStampField
		def DerivativeEventTime.field
			return 1289
		end
		def initialize(data = nil)
			if( data == nil )
				super(1289)
			else
				super(1289, data)
			end
		end
	end

	class DerivativeEventPx < Quickfix::DoubleField
		def DerivativeEventPx.field
			return 1290
		end
		def initialize(data = nil)
			if( data == nil )
				super(1290)
			else
				super(1290, data)
			end
		end
	end

	class DerivativeEventText < Quickfix::StringField
		def DerivativeEventText.field
			return 1291
		end
		def initialize(data = nil)
			if( data == nil )
				super(1291)
			else
				super(1291, data)
			end
		end
	end

	class NoDerivativeInstrumentParties < Quickfix::IntField
		def NoDerivativeInstrumentParties.field
			return 1292
		end
		def initialize(data = nil)
			if( data == nil )
				super(1292)
			else
				super(1292, data)
			end
		end
	end

	class DerivativeInstrumentPartyID < Quickfix::StringField
		def DerivativeInstrumentPartyID.field
			return 1293
		end
		def initialize(data = nil)
			if( data == nil )
				super(1293)
			else
				super(1293, data)
			end
		end
	end

	class DerivativeInstrumentPartyIDSource < Quickfix::CharField
		def DerivativeInstrumentPartyIDSource.field
			return 1294
		end
		def initialize(data = nil)
			if( data == nil )
				super(1294)
			else
				super(1294, data)
			end
		end
	end

	class DerivativeInstrumentPartyRole < Quickfix::IntField
		def DerivativeInstrumentPartyRole.field
			return 1295
		end
		def initialize(data = nil)
			if( data == nil )
				super(1295)
			else
				super(1295, data)
			end
		end
	end

	class NoDerivativeInstrumentPartySubIDs < Quickfix::IntField
		def NoDerivativeInstrumentPartySubIDs.field
			return 1296
		end
		def initialize(data = nil)
			if( data == nil )
				super(1296)
			else
				super(1296, data)
			end
		end
	end

	class DerivativeInstrumentPartySubID < Quickfix::StringField
		def DerivativeInstrumentPartySubID.field
			return 1297
		end
		def initialize(data = nil)
			if( data == nil )
				super(1297)
			else
				super(1297, data)
			end
		end
	end

	class DerivativeInstrumentPartySubIDType < Quickfix::IntField
		def DerivativeInstrumentPartySubIDType.field
			return 1298
		end
		def initialize(data = nil)
			if( data == nil )
				super(1298)
			else
				super(1298, data)
			end
		end
	end

	class DerivativeExerciseStyle < Quickfix::IntField
		def DerivativeExerciseStyle.field
			return 1299
		end
		def initialize(data = nil)
			if( data == nil )
				super(1299)
			else
				super(1299, data)
			end
		end
	end

	class MarketSegmentID < Quickfix::StringField
		def MarketSegmentID.field
			return 1300
		end
		def initialize(data = nil)
			if( data == nil )
				super(1300)
			else
				super(1300, data)
			end
		end
	end

	class MarketID < Quickfix::StringField
		def MarketID.field
			return 1301
		end
		def initialize(data = nil)
			if( data == nil )
				super(1301)
			else
				super(1301, data)
			end
		end
	end

	class MaturityMonthYearIncrementUnits < Quickfix::IntField
		def MaturityMonthYearIncrementUnits.field
			return 1302
		end
		def initialize(data = nil)
			if( data == nil )
				super(1302)
			else
				super(1302, data)
			end
		end
	end

	class MaturityMonthYearFormat < Quickfix::IntField
		def MaturityMonthYearFormat.field
			return 1303
		end
		def initialize(data = nil)
			if( data == nil )
				super(1303)
			else
				super(1303, data)
			end
		end
	end

	class StrikeExerciseStyle < Quickfix::IntField
		def StrikeExerciseStyle.field
			return 1304
		end
		def initialize(data = nil)
			if( data == nil )
				super(1304)
			else
				super(1304, data)
			end
		end
	end

	class SecondaryPriceLimitType < Quickfix::IntField
		def SecondaryPriceLimitType.field
			return 1305
		end
		def initialize(data = nil)
			if( data == nil )
				super(1305)
			else
				super(1305, data)
			end
		end
	end

	class PriceLimitType < Quickfix::IntField
		def PriceLimitType.field
			return 1306
		end
		def initialize(data = nil)
			if( data == nil )
				super(1306)
			else
				super(1306, data)
			end
		end
	end

	class ExecInstValue < Quickfix::StringField
		def ExecInstValue.field
			return 1308
		end
		def initialize(data = nil)
			if( data == nil )
				super(1308)
			else
				super(1308, data)
			end
		end
	end

	class NoTradingSessionRules < Quickfix::IntField
		def NoTradingSessionRules.field
			return 1309
		end
		def initialize(data = nil)
			if( data == nil )
				super(1309)
			else
				super(1309, data)
			end
		end
	end

	class NoMarketSegments < Quickfix::IntField
		def NoMarketSegments.field
			return 1310
		end
		def initialize(data = nil)
			if( data == nil )
				super(1310)
			else
				super(1310, data)
			end
		end
	end

	class NoDerivativeInstrAttrib < Quickfix::IntField
		def NoDerivativeInstrAttrib.field
			return 1311
		end
		def initialize(data = nil)
			if( data == nil )
				super(1311)
			else
				super(1311, data)
			end
		end
	end

	class NoNestedInstrAttrib < Quickfix::IntField
		def NoNestedInstrAttrib.field
			return 1312
		end
		def initialize(data = nil)
			if( data == nil )
				super(1312)
			else
				super(1312, data)
			end
		end
	end

	class DerivativeInstrAttribType < Quickfix::IntField
		def DerivativeInstrAttribType.field
			return 1313
		end
		def initialize(data = nil)
			if( data == nil )
				super(1313)
			else
				super(1313, data)
			end
		end
	end

	class DerivativeInstrAttribValue < Quickfix::StringField
		def DerivativeInstrAttribValue.field
			return 1314
		end
		def initialize(data = nil)
			if( data == nil )
				super(1314)
			else
				super(1314, data)
			end
		end
	end

	class DerivativePriceUnitOfMeasure < Quickfix::StringField
		def DerivativePriceUnitOfMeasure.field
			return 1315
		end
		def initialize(data = nil)
			if( data == nil )
				super(1315)
			else
				super(1315, data)
			end
		end
	end

	class DerivativePriceUnitOfMeasureQty < Quickfix::DoubleField
		def DerivativePriceUnitOfMeasureQty.field
			return 1316
		end
		def initialize(data = nil)
			if( data == nil )
				super(1316)
			else
				super(1316, data)
			end
		end
	end

	class DerivativeSettlMethod < Quickfix::StringField
		def DerivativeSettlMethod.field
			return 1317
		end
		def initialize(data = nil)
			if( data == nil )
				super(1317)
			else
				super(1317, data)
			end
		end
	end

	class DerivativePriceQuoteMethod < Quickfix::StringField
		def DerivativePriceQuoteMethod.field
			return 1318
		end
		def initialize(data = nil)
			if( data == nil )
				super(1318)
			else
				super(1318, data)
			end
		end
	end

	class DerivativeFuturesValuationMethod < Quickfix::StringField
		def DerivativeFuturesValuationMethod.field
			return 1319
		end
		def initialize(data = nil)
			if( data == nil )
				super(1319)
			else
				super(1319, data)
			end
		end
	end

	class DerivativeListMethod < Quickfix::IntField
		def DerivativeListMethod.field
			return 1320
		end
		def initialize(data = nil)
			if( data == nil )
				super(1320)
			else
				super(1320, data)
			end
		end
	end

	class DerivativeCapPrice < Quickfix::DoubleField
		def DerivativeCapPrice.field
			return 1321
		end
		def initialize(data = nil)
			if( data == nil )
				super(1321)
			else
				super(1321, data)
			end
		end
	end

	class DerivativeFloorPrice < Quickfix::DoubleField
		def DerivativeFloorPrice.field
			return 1322
		end
		def initialize(data = nil)
			if( data == nil )
				super(1322)
			else
				super(1322, data)
			end
		end
	end

	class DerivativePutOrCall < Quickfix::IntField
		def DerivativePutOrCall.field
			return 1323
		end
		def initialize(data = nil)
			if( data == nil )
				super(1323)
			else
				super(1323, data)
			end
		end
	end

	class ListUpdateAction < Quickfix::CharField
		def ListUpdateAction.field
			return 1324
		end
		def initialize(data = nil)
			if( data == nil )
				super(1324)
			else
				super(1324, data)
			end
		end
	end

	class LegPutOrCall < Quickfix::IntField
		def LegPutOrCall.field
			return 1358
		end
		def initialize(data = nil)
			if( data == nil )
				super(1358)
			else
				super(1358, data)
			end
		end
	end

	class LegUnitOfMeasureQty < Quickfix::DoubleField
		def LegUnitOfMeasureQty.field
			return 1224
		end
		def initialize(data = nil)
			if( data == nil )
				super(1224)
			else
				super(1224, data)
			end
		end
	end

	class LegPriceUnitOfMeasure < Quickfix::StringField
		def LegPriceUnitOfMeasure.field
			return 1421
		end
		def initialize(data = nil)
			if( data == nil )
				super(1421)
			else
				super(1421, data)
			end
		end
	end

	class LegPriceUnitOfMeasureQty < Quickfix::DoubleField
		def LegPriceUnitOfMeasureQty.field
			return 1422
		end
		def initialize(data = nil)
			if( data == nil )
				super(1422)
			else
				super(1422, data)
			end
		end
	end

	class UnderlyingUnitOfMeasureQty < Quickfix::DoubleField
		def UnderlyingUnitOfMeasureQty.field
			return 1423
		end
		def initialize(data = nil)
			if( data == nil )
				super(1423)
			else
				super(1423, data)
			end
		end
	end

	class UnderlyingPriceUnitOfMeasure < Quickfix::StringField
		def UnderlyingPriceUnitOfMeasure.field
			return 1424
		end
		def initialize(data = nil)
			if( data == nil )
				super(1424)
			else
				super(1424, data)
			end
		end
	end

	class UnderlyingPriceUnitOfMeasureQty < Quickfix::DoubleField
		def UnderlyingPriceUnitOfMeasureQty.field
			return 1425
		end
		def initialize(data = nil)
			if( data == nil )
				super(1425)
			else
				super(1425, data)
			end
		end
	end

	class MarketReqID < Quickfix::StringField
		def MarketReqID.field
			return 1393
		end
		def initialize(data = nil)
			if( data == nil )
				super(1393)
			else
				super(1393, data)
			end
		end
	end

	class MarketReportID < Quickfix::StringField
		def MarketReportID.field
			return 1394
		end
		def initialize(data = nil)
			if( data == nil )
				super(1394)
			else
				super(1394, data)
			end
		end
	end

	class MarketUpdateAction < Quickfix::CharField
		def MarketUpdateAction.field
			return 1395
		end
		def initialize(data = nil)
			if( data == nil )
				super(1395)
			else
				super(1395, data)
			end
		end
	end

	class MarketSegmentDesc < Quickfix::StringField
		def MarketSegmentDesc.field
			return 1396
		end
		def initialize(data = nil)
			if( data == nil )
				super(1396)
			else
				super(1396, data)
			end
		end
	end

	class EncodedMktSegmDescLen < Quickfix::IntField
		def EncodedMktSegmDescLen.field
			return 1397
		end
		def initialize(data = nil)
			if( data == nil )
				super(1397)
			else
				super(1397, data)
			end
		end
	end

	class EncodedMktSegmDesc < Quickfix::StringField
		def EncodedMktSegmDesc.field
			return 1398
		end
		def initialize(data = nil)
			if( data == nil )
				super(1398)
			else
				super(1398, data)
			end
		end
	end

	class ParentMktSegmID < Quickfix::StringField
		def ParentMktSegmID.field
			return 1325
		end
		def initialize(data = nil)
			if( data == nil )
				super(1325)
			else
				super(1325, data)
			end
		end
	end

	class TradingSessionDesc < Quickfix::StringField
		def TradingSessionDesc.field
			return 1326
		end
		def initialize(data = nil)
			if( data == nil )
				super(1326)
			else
				super(1326, data)
			end
		end
	end

	class TradSesUpdateAction < Quickfix::CharField
		def TradSesUpdateAction.field
			return 1327
		end
		def initialize(data = nil)
			if( data == nil )
				super(1327)
			else
				super(1327, data)
			end
		end
	end

	class RejectText < Quickfix::StringField
		def RejectText.field
			return 1328
		end
		def initialize(data = nil)
			if( data == nil )
				super(1328)
			else
				super(1328, data)
			end
		end
	end

	class FeeMultiplier < Quickfix::DoubleField
		def FeeMultiplier.field
			return 1329
		end
		def initialize(data = nil)
			if( data == nil )
				super(1329)
			else
				super(1329, data)
			end
		end
	end

	class UnderlyingLegSymbol < Quickfix::StringField
		def UnderlyingLegSymbol.field
			return 1330
		end
		def initialize(data = nil)
			if( data == nil )
				super(1330)
			else
				super(1330, data)
			end
		end
	end

	class UnderlyingLegSymbolSfx < Quickfix::StringField
		def UnderlyingLegSymbolSfx.field
			return 1331
		end
		def initialize(data = nil)
			if( data == nil )
				super(1331)
			else
				super(1331, data)
			end
		end
	end

	class UnderlyingLegSecurityID < Quickfix::StringField
		def UnderlyingLegSecurityID.field
			return 1332
		end
		def initialize(data = nil)
			if( data == nil )
				super(1332)
			else
				super(1332, data)
			end
		end
	end

	class UnderlyingLegSecurityIDSource < Quickfix::StringField
		def UnderlyingLegSecurityIDSource.field
			return 1333
		end
		def initialize(data = nil)
			if( data == nil )
				super(1333)
			else
				super(1333, data)
			end
		end
	end

	class NoUnderlyingLegSecurityAltID < Quickfix::IntField
		def NoUnderlyingLegSecurityAltID.field
			return 1334
		end
		def initialize(data = nil)
			if( data == nil )
				super(1334)
			else
				super(1334, data)
			end
		end
	end

	class UnderlyingLegSecurityAltID < Quickfix::StringField
		def UnderlyingLegSecurityAltID.field
			return 1335
		end
		def initialize(data = nil)
			if( data == nil )
				super(1335)
			else
				super(1335, data)
			end
		end
	end

	class UnderlyingLegSecurityAltIDSource < Quickfix::StringField
		def UnderlyingLegSecurityAltIDSource.field
			return 1336
		end
		def initialize(data = nil)
			if( data == nil )
				super(1336)
			else
				super(1336, data)
			end
		end
	end

	class UnderlyingLegSecurityType < Quickfix::StringField
		def UnderlyingLegSecurityType.field
			return 1337
		end
		def initialize(data = nil)
			if( data == nil )
				super(1337)
			else
				super(1337, data)
			end
		end
	end

	class UnderlyingLegSecuritySubType < Quickfix::StringField
		def UnderlyingLegSecuritySubType.field
			return 1338
		end
		def initialize(data = nil)
			if( data == nil )
				super(1338)
			else
				super(1338, data)
			end
		end
	end

	class UnderlyingLegMaturityMonthYear < Quickfix::StringField
		def UnderlyingLegMaturityMonthYear.field
			return 1339
		end
		def initialize(data = nil)
			if( data == nil )
				super(1339)
			else
				super(1339, data)
			end
		end
	end

	class UnderlyingLegPutOrCall < Quickfix::IntField
		def UnderlyingLegPutOrCall.field
			return 1343
		end
		def initialize(data = nil)
			if( data == nil )
				super(1343)
			else
				super(1343, data)
			end
		end
	end

	class UnderlyingLegStrikePrice < Quickfix::DoubleField
		def UnderlyingLegStrikePrice.field
			return 1340
		end
		def initialize(data = nil)
			if( data == nil )
				super(1340)
			else
				super(1340, data)
			end
		end
	end

	class UnderlyingLegSecurityExchange < Quickfix::StringField
		def UnderlyingLegSecurityExchange.field
			return 1341
		end
		def initialize(data = nil)
			if( data == nil )
				super(1341)
			else
				super(1341, data)
			end
		end
	end

	class NoOfLegUnderlyings < Quickfix::IntField
		def NoOfLegUnderlyings.field
			return 1342
		end
		def initialize(data = nil)
			if( data == nil )
				super(1342)
			else
				super(1342, data)
			end
		end
	end

	class UnderlyingLegCFICode < Quickfix::StringField
		def UnderlyingLegCFICode.field
			return 1344
		end
		def initialize(data = nil)
			if( data == nil )
				super(1344)
			else
				super(1344, data)
			end
		end
	end

	class UnderlyingLegMaturityDate < Quickfix::StringField
		def UnderlyingLegMaturityDate.field
			return 1345
		end
		def initialize(data = nil)
			if( data == nil )
				super(1345)
			else
				super(1345, data)
			end
		end
	end

	class UnderlyingLegMaturityTime < Quickfix::StringField
		def UnderlyingLegMaturityTime.field
			return 1405
		end
		def initialize(data = nil)
			if( data == nil )
				super(1405)
			else
				super(1405, data)
			end
		end
	end

	class UnderlyingLegOptAttribute < Quickfix::CharField
		def UnderlyingLegOptAttribute.field
			return 1391
		end
		def initialize(data = nil)
			if( data == nil )
				super(1391)
			else
				super(1391, data)
			end
		end
	end

	class UnderlyingLegSecurityDesc < Quickfix::StringField
		def UnderlyingLegSecurityDesc.field
			return 1392
		end
		def initialize(data = nil)
			if( data == nil )
				super(1392)
			else
				super(1392, data)
			end
		end
	end

	class DefaultVerIndicator < Quickfix::BoolField
		def DefaultVerIndicator.field
			return 1410
		end
		def initialize(data = nil)
			if( data == nil )
				super(1410)
			else
				super(1410, data)
			end
		end
	end

	class NoUsernames < Quickfix::IntField
		def NoUsernames.field
			return 809
		end
		def initialize(data = nil)
			if( data == nil )
				super(809)
			else
				super(809, data)
			end
		end
	end

	class LegAllocSettlCurrency < Quickfix::StringField
		def LegAllocSettlCurrency.field
			return 1367
		end
		def initialize(data = nil)
			if( data == nil )
				super(1367)
			else
				super(1367, data)
			end
		end
	end

	class TotNoFills < Quickfix::IntField
		def TotNoFills.field
			return 1361
		end
		def initialize(data = nil)
			if( data == nil )
				super(1361)
			else
				super(1361, data)
			end
		end
	end

	class NoFills < Quickfix::IntField
		def NoFills.field
			return 1362
		end
		def initialize(data = nil)
			if( data == nil )
				super(1362)
			else
				super(1362, data)
			end
		end
	end

	class FillExecID < Quickfix::StringField
		def FillExecID.field
			return 1363
		end
		def initialize(data = nil)
			if( data == nil )
				super(1363)
			else
				super(1363, data)
			end
		end
	end

	class FillPx < Quickfix::DoubleField
		def FillPx.field
			return 1364
		end
		def initialize(data = nil)
			if( data == nil )
				super(1364)
			else
				super(1364, data)
			end
		end
	end

	class FillQty < Quickfix::DoubleField
		def FillQty.field
			return 1365
		end
		def initialize(data = nil)
			if( data == nil )
				super(1365)
			else
				super(1365, data)
			end
		end
	end

	class LegAllocID < Quickfix::StringField
		def LegAllocID.field
			return 1366
		end
		def initialize(data = nil)
			if( data == nil )
				super(1366)
			else
				super(1366, data)
			end
		end
	end

	class TradSesEvent < Quickfix::IntField
		def TradSesEvent.field
			return 1368
		end
		def initialize(data = nil)
			if( data == nil )
				super(1368)
			else
				super(1368, data)
			end
		end
	end

	class MassActionReportID < Quickfix::StringField
		def MassActionReportID.field
			return 1369
		end
		def initialize(data = nil)
			if( data == nil )
				super(1369)
			else
				super(1369, data)
			end
		end
	end

	class NoNotAffectedOrders < Quickfix::IntField
		def NoNotAffectedOrders.field
			return 1370
		end
		def initialize(data = nil)
			if( data == nil )
				super(1370)
			else
				super(1370, data)
			end
		end
	end

	class NotAffectedOrderID < Quickfix::StringField
		def NotAffectedOrderID.field
			return 1371
		end
		def initialize(data = nil)
			if( data == nil )
				super(1371)
			else
				super(1371, data)
			end
		end
	end

	class NotAffOrigClOrdID < Quickfix::StringField
		def NotAffOrigClOrdID.field
			return 1372
		end
		def initialize(data = nil)
			if( data == nil )
				super(1372)
			else
				super(1372, data)
			end
		end
	end

	class MassActionType < Quickfix::IntField
		def MassActionType.field
			return 1373
		end
		def initialize(data = nil)
			if( data == nil )
				super(1373)
			else
				super(1373, data)
			end
		end
	end

	class MassActionScope < Quickfix::IntField
		def MassActionScope.field
			return 1374
		end
		def initialize(data = nil)
			if( data == nil )
				super(1374)
			else
				super(1374, data)
			end
		end
	end

	class MassActionResponse < Quickfix::IntField
		def MassActionResponse.field
			return 1375
		end
		def initialize(data = nil)
			if( data == nil )
				super(1375)
			else
				super(1375, data)
			end
		end
	end

	class MassActionRejectReason < Quickfix::IntField
		def MassActionRejectReason.field
			return 1376
		end
		def initialize(data = nil)
			if( data == nil )
				super(1376)
			else
				super(1376, data)
			end
		end
	end

	class MultilegModel < Quickfix::IntField
		def MultilegModel.field
			return 1377
		end
		def initialize(data = nil)
			if( data == nil )
				super(1377)
			else
				super(1377, data)
			end
		end
	end

	class MultilegPriceMethod < Quickfix::IntField
		def MultilegPriceMethod.field
			return 1378
		end
		def initialize(data = nil)
			if( data == nil )
				super(1378)
			else
				super(1378, data)
			end
		end
	end

	class LegVolatility < Quickfix::DoubleField
		def LegVolatility.field
			return 1379
		end
		def initialize(data = nil)
			if( data == nil )
				super(1379)
			else
				super(1379, data)
			end
		end
	end

	class DividendYield < Quickfix::DoubleField
		def DividendYield.field
			return 1380
		end
		def initialize(data = nil)
			if( data == nil )
				super(1380)
			else
				super(1380, data)
			end
		end
	end

	class LegDividendYield < Quickfix::DoubleField
		def LegDividendYield.field
			return 1381
		end
		def initialize(data = nil)
			if( data == nil )
				super(1381)
			else
				super(1381, data)
			end
		end
	end

	class CurrencyRatio < Quickfix::DoubleField
		def CurrencyRatio.field
			return 1382
		end
		def initialize(data = nil)
			if( data == nil )
				super(1382)
			else
				super(1382, data)
			end
		end
	end

	class LegCurrencyRatio < Quickfix::DoubleField
		def LegCurrencyRatio.field
			return 1383
		end
		def initialize(data = nil)
			if( data == nil )
				super(1383)
			else
				super(1383, data)
			end
		end
	end

	class LegExecInst < Quickfix::StringField
		def LegExecInst.field
			return 1384
		end
		def initialize(data = nil)
			if( data == nil )
				super(1384)
			else
				super(1384, data)
			end
		end
	end

	class ContingencyType < Quickfix::IntField
		def ContingencyType.field
			return 1385
		end
		def initialize(data = nil)
			if( data == nil )
				super(1385)
			else
				super(1385, data)
			end
		end
	end

	class ListRejectReason < Quickfix::IntField
		def ListRejectReason.field
			return 1386
		end
		def initialize(data = nil)
			if( data == nil )
				super(1386)
			else
				super(1386, data)
			end
		end
	end

	class NoTrdRepIndicators < Quickfix::IntField
		def NoTrdRepIndicators.field
			return 1387
		end
		def initialize(data = nil)
			if( data == nil )
				super(1387)
			else
				super(1387, data)
			end
		end
	end

	class TrdRepPartyRole < Quickfix::IntField
		def TrdRepPartyRole.field
			return 1388
		end
		def initialize(data = nil)
			if( data == nil )
				super(1388)
			else
				super(1388, data)
			end
		end
	end

	class TrdRepIndicator < Quickfix::BoolField
		def TrdRepIndicator.field
			return 1389
		end
		def initialize(data = nil)
			if( data == nil )
				super(1389)
			else
				super(1389, data)
			end
		end
	end

	class TradePublishIndicator < Quickfix::IntField
		def TradePublishIndicator.field
			return 1390
		end
		def initialize(data = nil)
			if( data == nil )
				super(1390)
			else
				super(1390, data)
			end
		end
	end

	class ApplReqID < Quickfix::StringField
		def ApplReqID.field
			return 1346
		end
		def initialize(data = nil)
			if( data == nil )
				super(1346)
			else
				super(1346, data)
			end
		end
	end

	class ApplReqType < Quickfix::IntField
		def ApplReqType.field
			return 1347
		end
		def initialize(data = nil)
			if( data == nil )
				super(1347)
			else
				super(1347, data)
			end
		end
	end

	class ApplResponseType < Quickfix::IntField
		def ApplResponseType.field
			return 1348
		end
		def initialize(data = nil)
			if( data == nil )
				super(1348)
			else
				super(1348, data)
			end
		end
	end

	class ApplTotalMessageCount < Quickfix::IntField
		def ApplTotalMessageCount.field
			return 1349
		end
		def initialize(data = nil)
			if( data == nil )
				super(1349)
			else
				super(1349, data)
			end
		end
	end

	class ApplLastSeqNum < Quickfix::IntField
		def ApplLastSeqNum.field
			return 1350
		end
		def initialize(data = nil)
			if( data == nil )
				super(1350)
			else
				super(1350, data)
			end
		end
	end

	class NoApplIDs < Quickfix::IntField
		def NoApplIDs.field
			return 1351
		end
		def initialize(data = nil)
			if( data == nil )
				super(1351)
			else
				super(1351, data)
			end
		end
	end

	class ApplResendFlag < Quickfix::BoolField
		def ApplResendFlag.field
			return 1352
		end
		def initialize(data = nil)
			if( data == nil )
				super(1352)
			else
				super(1352, data)
			end
		end
	end

	class ApplResponseID < Quickfix::StringField
		def ApplResponseID.field
			return 1353
		end
		def initialize(data = nil)
			if( data == nil )
				super(1353)
			else
				super(1353, data)
			end
		end
	end

	class ApplResponseError < Quickfix::IntField
		def ApplResponseError.field
			return 1354
		end
		def initialize(data = nil)
			if( data == nil )
				super(1354)
			else
				super(1354, data)
			end
		end
	end

	class RefApplID < Quickfix::StringField
		def RefApplID.field
			return 1355
		end
		def initialize(data = nil)
			if( data == nil )
				super(1355)
			else
				super(1355, data)
			end
		end
	end

	class ApplReportID < Quickfix::StringField
		def ApplReportID.field
			return 1356
		end
		def initialize(data = nil)
			if( data == nil )
				super(1356)
			else
				super(1356, data)
			end
		end
	end

	class RefApplLastSeqNum < Quickfix::IntField
		def RefApplLastSeqNum.field
			return 1357
		end
		def initialize(data = nil)
			if( data == nil )
				super(1357)
			else
				super(1357, data)
			end
		end
	end

	class ApplNewSeqNum < Quickfix::IntField
		def ApplNewSeqNum.field
			return 1399
		end
		def initialize(data = nil)
			if( data == nil )
				super(1399)
			else
				super(1399, data)
			end
		end
	end

	class ApplReportType < Quickfix::IntField
		def ApplReportType.field
			return 1426
		end
		def initialize(data = nil)
			if( data == nil )
				super(1426)
			else
				super(1426, data)
			end
		end
	end

	class Nested4PartySubIDType < Quickfix::IntField
		def Nested4PartySubIDType.field
			return 1411
		end
		def initialize(data = nil)
			if( data == nil )
				super(1411)
			else
				super(1411, data)
			end
		end
	end

	class Nested4PartySubID < Quickfix::StringField
		def Nested4PartySubID.field
			return 1412
		end
		def initialize(data = nil)
			if( data == nil )
				super(1412)
			else
				super(1412, data)
			end
		end
	end

	class NoNested4PartySubIDs < Quickfix::IntField
		def NoNested4PartySubIDs.field
			return 1413
		end
		def initialize(data = nil)
			if( data == nil )
				super(1413)
			else
				super(1413, data)
			end
		end
	end

	class NoNested4PartyIDs < Quickfix::IntField
		def NoNested4PartyIDs.field
			return 1414
		end
		def initialize(data = nil)
			if( data == nil )
				super(1414)
			else
				super(1414, data)
			end
		end
	end

	class Nested4PartyID < Quickfix::StringField
		def Nested4PartyID.field
			return 1415
		end
		def initialize(data = nil)
			if( data == nil )
				super(1415)
			else
				super(1415, data)
			end
		end
	end

	class Nested4PartyIDSource < Quickfix::CharField
		def Nested4PartyIDSource.field
			return 1416
		end
		def initialize(data = nil)
			if( data == nil )
				super(1416)
			else
				super(1416, data)
			end
		end
	end

	class Nested4PartyRole < Quickfix::IntField
		def Nested4PartyRole.field
			return 1417
		end
		def initialize(data = nil)
			if( data == nil )
				super(1417)
			else
				super(1417, data)
			end
		end
	end

	class LegLastQty < Quickfix::DoubleField
		def LegLastQty.field
			return 1418
		end
		def initialize(data = nil)
			if( data == nil )
				super(1418)
			else
				super(1418, data)
			end
		end
	end

	class HaltReasonInt < Quickfix::IntField
		def HaltReasonInt.field
			return 327
		end
		def initialize(data = nil)
			if( data == nil )
				super(327)
			else
				super(327, data)
			end
		end
	end

	class SideTrdSubType < Quickfix::IntField
		def SideTrdSubType.field
			return 1008
		end
		def initialize(data = nil)
			if( data == nil )
				super(1008)
			else
				super(1008, data)
			end
		end
	end

	class SideLastQty < Quickfix::DoubleField
		def SideLastQty.field
			return 1009
		end
		def initialize(data = nil)
			if( data == nil )
				super(1009)
			else
				super(1009, data)
			end
		end
	end

	class UnderlyingInstrumentPartyID < Quickfix::StringField
		def UnderlyingInstrumentPartyID.field
			return 1059
		end
		def initialize(data = nil)
			if( data == nil )
				super(1059)
			else
				super(1059, data)
			end
		end
	end

	class UnderlyingInstrumentPartyIDSource < Quickfix::CharField
		def UnderlyingInstrumentPartyIDSource.field
			return 1060
		end
		def initialize(data = nil)
			if( data == nil )
				super(1060)
			else
				super(1060, data)
			end
		end
	end

	class UnderlyingInstrumentPartyRole < Quickfix::IntField
		def UnderlyingInstrumentPartyRole.field
			return 1061
		end
		def initialize(data = nil)
			if( data == nil )
				super(1061)
			else
				super(1061, data)
			end
		end
	end

	class UnderlyingInstrumentPartySubID < Quickfix::StringField
		def UnderlyingInstrumentPartySubID.field
			return 1063
		end
		def initialize(data = nil)
			if( data == nil )
				super(1063)
			else
				super(1063, data)
			end
		end
	end

	class UnderlyingInstrumentPartySubIDType < Quickfix::IntField
		def UnderlyingInstrumentPartySubIDType.field
			return 1064
		end
		def initialize(data = nil)
			if( data == nil )
				super(1064)
			else
				super(1064, data)
			end
		end
	end

	class OptPayoutAmount < Quickfix::DoubleField
		def OptPayoutAmount.field
			return 1195
		end
		def initialize(data = nil)
			if( data == nil )
				super(1195)
			else
				super(1195, data)
			end
		end
	end

	class ValuationMethod < Quickfix::StringField
		def ValuationMethod.field
			return 1197
		end
		def initialize(data = nil)
			if( data == nil )
				super(1197)
			else
				super(1197, data)
			end
		end
	end

	class DerivativeValuationMethod < Quickfix::StringField
		def DerivativeValuationMethod.field
			return 1319
		end
		def initialize(data = nil)
			if( data == nil )
				super(1319)
			else
				super(1319, data)
			end
		end
	end

	class SideExecID < Quickfix::StringField
		def SideExecID.field
			return 1427
		end
		def initialize(data = nil)
			if( data == nil )
				super(1427)
			else
				super(1427, data)
			end
		end
	end

	class OrderDelay < Quickfix::IntField
		def OrderDelay.field
			return 1428
		end
		def initialize(data = nil)
			if( data == nil )
				super(1428)
			else
				super(1428, data)
			end
		end
	end

	class OrderDelayUnit < Quickfix::IntField
		def OrderDelayUnit.field
			return 1429
		end
		def initialize(data = nil)
			if( data == nil )
				super(1429)
			else
				super(1429, data)
			end
		end
	end

	class VenueType < Quickfix::CharField
		def VenueType.field
			return 1430
		end
		def initialize(data = nil)
			if( data == nil )
				super(1430)
			else
				super(1430, data)
			end
		end
	end

	class RefOrdIDReason < Quickfix::IntField
		def RefOrdIDReason.field
			return 1431
		end
		def initialize(data = nil)
			if( data == nil )
				super(1431)
			else
				super(1431, data)
			end
		end
	end

	class OrigCustOrderCapacity < Quickfix::IntField
		def OrigCustOrderCapacity.field
			return 1432
		end
		def initialize(data = nil)
			if( data == nil )
				super(1432)
			else
				super(1432, data)
			end
		end
	end

	class RefApplReqID < Quickfix::StringField
		def RefApplReqID.field
			return 1433
		end
		def initialize(data = nil)
			if( data == nil )
				super(1433)
			else
				super(1433, data)
			end
		end
	end

	class ModelType < Quickfix::IntField
		def ModelType.field
			return 1434
		end
		def initialize(data = nil)
			if( data == nil )
				super(1434)
			else
				super(1434, data)
			end
		end
	end

	class ContractMultiplierUnit < Quickfix::IntField
		def ContractMultiplierUnit.field
			return 1435
		end
		def initialize(data = nil)
			if( data == nil )
				super(1435)
			else
				super(1435, data)
			end
		end
	end

	class LegContractMultiplierUnit < Quickfix::IntField
		def LegContractMultiplierUnit.field
			return 1436
		end
		def initialize(data = nil)
			if( data == nil )
				super(1436)
			else
				super(1436, data)
			end
		end
	end

	class UnderlyingContractMultiplierUnit < Quickfix::IntField
		def UnderlyingContractMultiplierUnit.field
			return 1437
		end
		def initialize(data = nil)
			if( data == nil )
				super(1437)
			else
				super(1437, data)
			end
		end
	end

	class DerivativeContractMultiplierUnit < Quickfix::IntField
		def DerivativeContractMultiplierUnit.field
			return 1438
		end
		def initialize(data = nil)
			if( data == nil )
				super(1438)
			else
				super(1438, data)
			end
		end
	end

	class FlowScheduleType < Quickfix::IntField
		def FlowScheduleType.field
			return 1439
		end
		def initialize(data = nil)
			if( data == nil )
				super(1439)
			else
				super(1439, data)
			end
		end
	end

	class LegFlowScheduleType < Quickfix::IntField
		def LegFlowScheduleType.field
			return 1440
		end
		def initialize(data = nil)
			if( data == nil )
				super(1440)
			else
				super(1440, data)
			end
		end
	end

	class UnderlyingFlowScheduleType < Quickfix::IntField
		def UnderlyingFlowScheduleType.field
			return 1441
		end
		def initialize(data = nil)
			if( data == nil )
				super(1441)
			else
				super(1441, data)
			end
		end
	end

	class DerivativeFlowScheduleType < Quickfix::IntField
		def DerivativeFlowScheduleType.field
			return 1442
		end
		def initialize(data = nil)
			if( data == nil )
				super(1442)
			else
				super(1442, data)
			end
		end
	end

	class FillLiquidityInd < Quickfix::IntField
		def FillLiquidityInd.field
			return 1443
		end
		def initialize(data = nil)
			if( data == nil )
				super(1443)
			else
				super(1443, data)
			end
		end
	end

	class SideLiquidityInd < Quickfix::IntField
		def SideLiquidityInd.field
			return 1444
		end
		def initialize(data = nil)
			if( data == nil )
				super(1444)
			else
				super(1444, data)
			end
		end
	end

	class NoRateSources < Quickfix::IntField
		def NoRateSources.field
			return 1445
		end
		def initialize(data = nil)
			if( data == nil )
				super(1445)
			else
				super(1445, data)
			end
		end
	end

	class RateSource < Quickfix::IntField
		def RateSource.field
			return 1446
		end
		def initialize(data = nil)
			if( data == nil )
				super(1446)
			else
				super(1446, data)
			end
		end
	end

	class RateSourceType < Quickfix::IntField
		def RateSourceType.field
			return 1447
		end
		def initialize(data = nil)
			if( data == nil )
				super(1447)
			else
				super(1447, data)
			end
		end
	end

	class ReferencePage < Quickfix::StringField
		def ReferencePage.field
			return 1448
		end
		def initialize(data = nil)
			if( data == nil )
				super(1448)
			else
				super(1448, data)
			end
		end
	end

	class RestructuringType < Quickfix::StringField
		def RestructuringType.field
			return 1449
		end
		def initialize(data = nil)
			if( data == nil )
				super(1449)
			else
				super(1449, data)
			end
		end
	end

	class Seniority < Quickfix::StringField
		def Seniority.field
			return 1450
		end
		def initialize(data = nil)
			if( data == nil )
				super(1450)
			else
				super(1450, data)
			end
		end
	end

	class NotionalPercentageOutstanding < Quickfix::DoubleField
		def NotionalPercentageOutstanding.field
			return 1451
		end
		def initialize(data = nil)
			if( data == nil )
				super(1451)
			else
				super(1451, data)
			end
		end
	end

	class OriginalNotionalPercentageOutstanding < Quickfix::DoubleField
		def OriginalNotionalPercentageOutstanding.field
			return 1452
		end
		def initialize(data = nil)
			if( data == nil )
				super(1452)
			else
				super(1452, data)
			end
		end
	end

	class UnderlyingRestructuringType < Quickfix::StringField
		def UnderlyingRestructuringType.field
			return 1453
		end
		def initialize(data = nil)
			if( data == nil )
				super(1453)
			else
				super(1453, data)
			end
		end
	end

	class UnderlyingSeniority < Quickfix::StringField
		def UnderlyingSeniority.field
			return 1454
		end
		def initialize(data = nil)
			if( data == nil )
				super(1454)
			else
				super(1454, data)
			end
		end
	end

	class UnderlyingNotionalPercentageOutstanding < Quickfix::DoubleField
		def UnderlyingNotionalPercentageOutstanding.field
			return 1455
		end
		def initialize(data = nil)
			if( data == nil )
				super(1455)
			else
				super(1455, data)
			end
		end
	end

	class UnderlyingOriginalNotionalPercentageOutstanding < Quickfix::DoubleField
		def UnderlyingOriginalNotionalPercentageOutstanding.field
			return 1456
		end
		def initialize(data = nil)
			if( data == nil )
				super(1456)
			else
				super(1456, data)
			end
		end
	end

	class AttachmentPoint < Quickfix::DoubleField
		def AttachmentPoint.field
			return 1457
		end
		def initialize(data = nil)
			if( data == nil )
				super(1457)
			else
				super(1457, data)
			end
		end
	end

	class DetachmentPoint < Quickfix::DoubleField
		def DetachmentPoint.field
			return 1458
		end
		def initialize(data = nil)
			if( data == nil )
				super(1458)
			else
				super(1458, data)
			end
		end
	end

	class UnderlyingAttachmentPoint < Quickfix::DoubleField
		def UnderlyingAttachmentPoint.field
			return 1459
		end
		def initialize(data = nil)
			if( data == nil )
				super(1459)
			else
				super(1459, data)
			end
		end
	end

	class UnderlyingDetachmentPoint < Quickfix::DoubleField
		def UnderlyingDetachmentPoint.field
			return 1460
		end
		def initialize(data = nil)
			if( data == nil )
				super(1460)
			else
				super(1460, data)
			end
		end
	end

	class NoTargetPartyIDs < Quickfix::IntField
		def NoTargetPartyIDs.field
			return 1461
		end
		def initialize(data = nil)
			if( data == nil )
				super(1461)
			else
				super(1461, data)
			end
		end
	end

	class TargetPartyID < Quickfix::StringField
		def TargetPartyID.field
			return 1462
		end
		def initialize(data = nil)
			if( data == nil )
				super(1462)
			else
				super(1462, data)
			end
		end
	end

	class TargetPartyIDSource < Quickfix::CharField
		def TargetPartyIDSource.field
			return 1463
		end
		def initialize(data = nil)
			if( data == nil )
				super(1463)
			else
				super(1463, data)
			end
		end
	end

	class TargetPartyRole < Quickfix::IntField
		def TargetPartyRole.field
			return 1464
		end
		def initialize(data = nil)
			if( data == nil )
				super(1464)
			else
				super(1464, data)
			end
		end
	end

	class SecurityListID < Quickfix::StringField
		def SecurityListID.field
			return 1465
		end
		def initialize(data = nil)
			if( data == nil )
				super(1465)
			else
				super(1465, data)
			end
		end
	end

	class SecurityListRefID < Quickfix::StringField
		def SecurityListRefID.field
			return 1466
		end
		def initialize(data = nil)
			if( data == nil )
				super(1466)
			else
				super(1466, data)
			end
		end
	end

	class SecurityListDesc < Quickfix::StringField
		def SecurityListDesc.field
			return 1467
		end
		def initialize(data = nil)
			if( data == nil )
				super(1467)
			else
				super(1467, data)
			end
		end
	end

	class EncodedSecurityListDescLen < Quickfix::IntField
		def EncodedSecurityListDescLen.field
			return 1468
		end
		def initialize(data = nil)
			if( data == nil )
				super(1468)
			else
				super(1468, data)
			end
		end
	end

	class EncodedSecurityListDesc < Quickfix::StringField
		def EncodedSecurityListDesc.field
			return 1469
		end
		def initialize(data = nil)
			if( data == nil )
				super(1469)
			else
				super(1469, data)
			end
		end
	end

	class SecurityListType < Quickfix::IntField
		def SecurityListType.field
			return 1470
		end
		def initialize(data = nil)
			if( data == nil )
				super(1470)
			else
				super(1470, data)
			end
		end
	end

	class SecurityListTypeSource < Quickfix::IntField
		def SecurityListTypeSource.field
			return 1471
		end
		def initialize(data = nil)
			if( data == nil )
				super(1471)
			else
				super(1471, data)
			end
		end
	end

	class NewsID < Quickfix::StringField
		def NewsID.field
			return 1472
		end
		def initialize(data = nil)
			if( data == nil )
				super(1472)
			else
				super(1472, data)
			end
		end
	end

	class NewsCategory < Quickfix::IntField
		def NewsCategory.field
			return 1473
		end
		def initialize(data = nil)
			if( data == nil )
				super(1473)
			else
				super(1473, data)
			end
		end
	end

	class LanguageCode < Quickfix::StringField
		def LanguageCode.field
			return 1474
		end
		def initialize(data = nil)
			if( data == nil )
				super(1474)
			else
				super(1474, data)
			end
		end
	end

	class NoNewsRefIDs < Quickfix::IntField
		def NoNewsRefIDs.field
			return 1475
		end
		def initialize(data = nil)
			if( data == nil )
				super(1475)
			else
				super(1475, data)
			end
		end
	end

	class NewsRefID < Quickfix::StringField
		def NewsRefID.field
			return 1476
		end
		def initialize(data = nil)
			if( data == nil )
				super(1476)
			else
				super(1476, data)
			end
		end
	end

	class NewsRefType < Quickfix::IntField
		def NewsRefType.field
			return 1477
		end
		def initialize(data = nil)
			if( data == nil )
				super(1477)
			else
				super(1477, data)
			end
		end
	end

	class StrikePriceDeterminationMethod < Quickfix::IntField
		def StrikePriceDeterminationMethod.field
			return 1478
		end
		def initialize(data = nil)
			if( data == nil )
				super(1478)
			else
				super(1478, data)
			end
		end
	end

	class StrikePriceBoundaryMethod < Quickfix::IntField
		def StrikePriceBoundaryMethod.field
			return 1479
		end
		def initialize(data = nil)
			if( data == nil )
				super(1479)
			else
				super(1479, data)
			end
		end
	end

	class StrikePriceBoundaryPrecision < Quickfix::DoubleField
		def StrikePriceBoundaryPrecision.field
			return 1480
		end
		def initialize(data = nil)
			if( data == nil )
				super(1480)
			else
				super(1480, data)
			end
		end
	end

	class UnderlyingPriceDeterminationMethod < Quickfix::IntField
		def UnderlyingPriceDeterminationMethod.field
			return 1481
		end
		def initialize(data = nil)
			if( data == nil )
				super(1481)
			else
				super(1481, data)
			end
		end
	end

	class OptPayoutType < Quickfix::IntField
		def OptPayoutType.field
			return 1482
		end
		def initialize(data = nil)
			if( data == nil )
				super(1482)
			else
				super(1482, data)
			end
		end
	end

	class NoComplexEvents < Quickfix::IntField
		def NoComplexEvents.field
			return 1483
		end
		def initialize(data = nil)
			if( data == nil )
				super(1483)
			else
				super(1483, data)
			end
		end
	end

	class ComplexEventType < Quickfix::IntField
		def ComplexEventType.field
			return 1484
		end
		def initialize(data = nil)
			if( data == nil )
				super(1484)
			else
				super(1484, data)
			end
		end
	end

	class ComplexOptPayoutAmount < Quickfix::DoubleField
		def ComplexOptPayoutAmount.field
			return 1485
		end
		def initialize(data = nil)
			if( data == nil )
				super(1485)
			else
				super(1485, data)
			end
		end
	end

	class ComplexEventPrice < Quickfix::DoubleField
		def ComplexEventPrice.field
			return 1486
		end
		def initialize(data = nil)
			if( data == nil )
				super(1486)
			else
				super(1486, data)
			end
		end
	end

	class ComplexEventPriceBoundaryMethod < Quickfix::IntField
		def ComplexEventPriceBoundaryMethod.field
			return 1487
		end
		def initialize(data = nil)
			if( data == nil )
				super(1487)
			else
				super(1487, data)
			end
		end
	end

	class ComplexEventPriceBoundaryPrecision < Quickfix::DoubleField
		def ComplexEventPriceBoundaryPrecision.field
			return 1488
		end
		def initialize(data = nil)
			if( data == nil )
				super(1488)
			else
				super(1488, data)
			end
		end
	end

	class ComplexEventPriceTimeType < Quickfix::IntField
		def ComplexEventPriceTimeType.field
			return 1489
		end
		def initialize(data = nil)
			if( data == nil )
				super(1489)
			else
				super(1489, data)
			end
		end
	end

	class ComplexEventCondition < Quickfix::IntField
		def ComplexEventCondition.field
			return 1490
		end
		def initialize(data = nil)
			if( data == nil )
				super(1490)
			else
				super(1490, data)
			end
		end
	end

	class NoComplexEventDates < Quickfix::IntField
		def NoComplexEventDates.field
			return 1491
		end
		def initialize(data = nil)
			if( data == nil )
				super(1491)
			else
				super(1491, data)
			end
		end
	end

	class ComplexEventStartDate < Quickfix::UtcDateField
		def ComplexEventStartDate.field
			return 1492
		end
		def initialize(data = nil)
			if( data == nil )
				super(1492)
			else
				super(1492, data)
			end
		end
	end

	class ComplexEventEndDate < Quickfix::UtcDateField
		def ComplexEventEndDate.field
			return 1493
		end
		def initialize(data = nil)
			if( data == nil )
				super(1493)
			else
				super(1493, data)
			end
		end
	end

	class NoComplexEventTimes < Quickfix::IntField
		def NoComplexEventTimes.field
			return 1494
		end
		def initialize(data = nil)
			if( data == nil )
				super(1494)
			else
				super(1494, data)
			end
		end
	end

	class ComplexEventStartTime < Quickfix::UtcTimeOnlyField
		def ComplexEventStartTime.field
			return 1495
		end
		def initialize(data = nil)
			if( data == nil )
				super(1495)
			else
				super(1495, data)
			end
		end
	end

	class ComplexEventEndTime < Quickfix::UtcTimeOnlyField
		def ComplexEventEndTime.field
			return 1496
		end
		def initialize(data = nil)
			if( data == nil )
				super(1496)
			else
				super(1496, data)
			end
		end
	end

	class StreamAsgnReqID < Quickfix::StringField
		def StreamAsgnReqID.field
			return 1497
		end
		def initialize(data = nil)
			if( data == nil )
				super(1497)
			else
				super(1497, data)
			end
		end
	end

	class StreamAsgnReqType < Quickfix::IntField
		def StreamAsgnReqType.field
			return 1498
		end
		def initialize(data = nil)
			if( data == nil )
				super(1498)
			else
				super(1498, data)
			end
		end
	end

	class NoAsgnReqs < Quickfix::IntField
		def NoAsgnReqs.field
			return 1499
		end
		def initialize(data = nil)
			if( data == nil )
				super(1499)
			else
				super(1499, data)
			end
		end
	end

	class MDStreamID < Quickfix::StringField
		def MDStreamID.field
			return 1500
		end
		def initialize(data = nil)
			if( data == nil )
				super(1500)
			else
				super(1500, data)
			end
		end
	end

	class StreamAsgnRptID < Quickfix::StringField
		def StreamAsgnRptID.field
			return 1501
		end
		def initialize(data = nil)
			if( data == nil )
				super(1501)
			else
				super(1501, data)
			end
		end
	end

	class StreamAsgnRejReason < Quickfix::IntField
		def StreamAsgnRejReason.field
			return 1502
		end
		def initialize(data = nil)
			if( data == nil )
				super(1502)
			else
				super(1502, data)
			end
		end
	end

	class StreamAsgnAckType < Quickfix::IntField
		def StreamAsgnAckType.field
			return 1503
		end
		def initialize(data = nil)
			if( data == nil )
				super(1503)
			else
				super(1503, data)
			end
		end
	end

	class StreamAsgnType < Quickfix::IntField
		def StreamAsgnType.field
			return 1617
		end
		def initialize(data = nil)
			if( data == nil )
				super(1617)
			else
				super(1617, data)
			end
		end
	end

	class RelSymTransactTime < Quickfix::UtcTimeStampField
		def RelSymTransactTime.field
			return 1504
		end
		def initialize(data = nil)
			if( data == nil )
				super(1504)
			else
				super(1504, data)
			end
		end
	end

	class FillYieldType < Quickfix::StringField
		def FillYieldType.field
			return 1622
		end
		def initialize(data = nil)
			if( data == nil )
				super(1622)
			else
				super(1622, data)
			end
		end
	end

	class FillYield < Quickfix::DoubleField
		def FillYield.field
			return 1623
		end
		def initialize(data = nil)
			if( data == nil )
				super(1623)
			else
				super(1623, data)
			end
		end
	end

	class NoMatchInst < Quickfix::IntField
		def NoMatchInst.field
			return 1624
		end
		def initialize(data = nil)
			if( data == nil )
				super(1624)
			else
				super(1624, data)
			end
		end
	end

	class MatchInst < Quickfix::IntField
		def MatchInst.field
			return 1625
		end
		def initialize(data = nil)
			if( data == nil )
				super(1625)
			else
				super(1625, data)
			end
		end
	end

	class MatchAttribTagID < Quickfix::StringField
		def MatchAttribTagID.field
			return 1626
		end
		def initialize(data = nil)
			if( data == nil )
				super(1626)
			else
				super(1626, data)
			end
		end
	end

	class MatchAttribValue < Quickfix::StringField
		def MatchAttribValue.field
			return 1627
		end
		def initialize(data = nil)
			if( data == nil )
				super(1627)
			else
				super(1627, data)
			end
		end
	end

	class MatchInstMarketID < Quickfix::StringField
		def MatchInstMarketID.field
			return 1673
		end
		def initialize(data = nil)
			if( data == nil )
				super(1673)
			else
				super(1673, data)
			end
		end
	end

	class TriggerScope < Quickfix::IntField
		def TriggerScope.field
			return 1628
		end
		def initialize(data = nil)
			if( data == nil )
				super(1628)
			else
				super(1628, data)
			end
		end
	end

	class ExposureDuration < Quickfix::IntField
		def ExposureDuration.field
			return 1629
		end
		def initialize(data = nil)
			if( data == nil )
				super(1629)
			else
				super(1629, data)
			end
		end
	end

	class NoLimitAmts < Quickfix::IntField
		def NoLimitAmts.field
			return 1630
		end
		def initialize(data = nil)
			if( data == nil )
				super(1630)
			else
				super(1630, data)
			end
		end
	end

	class LimitAmtType < Quickfix::IntField
		def LimitAmtType.field
			return 1631
		end
		def initialize(data = nil)
			if( data == nil )
				super(1631)
			else
				super(1631, data)
			end
		end
	end

	class LastLimitAmt < Quickfix::DoubleField
		def LastLimitAmt.field
			return 1632
		end
		def initialize(data = nil)
			if( data == nil )
				super(1632)
			else
				super(1632, data)
			end
		end
	end

	class LimitAmtRemaining < Quickfix::DoubleField
		def LimitAmtRemaining.field
			return 1633
		end
		def initialize(data = nil)
			if( data == nil )
				super(1633)
			else
				super(1633, data)
			end
		end
	end

	class LimitAmtCurrency < Quickfix::StringField
		def LimitAmtCurrency.field
			return 1634
		end
		def initialize(data = nil)
			if( data == nil )
				super(1634)
			else
				super(1634, data)
			end
		end
	end

	class MarginReqmtInqID < Quickfix::StringField
		def MarginReqmtInqID.field
			return 1635
		end
		def initialize(data = nil)
			if( data == nil )
				super(1635)
			else
				super(1635, data)
			end
		end
	end

	class NoMarginReqmtInqQualifier < Quickfix::IntField
		def NoMarginReqmtInqQualifier.field
			return 1636
		end
		def initialize(data = nil)
			if( data == nil )
				super(1636)
			else
				super(1636, data)
			end
		end
	end

	class MarginReqmtInqQualifier < Quickfix::IntField
		def MarginReqmtInqQualifier.field
			return 1637
		end
		def initialize(data = nil)
			if( data == nil )
				super(1637)
			else
				super(1637, data)
			end
		end
	end

	class MarginReqmtRptType < Quickfix::IntField
		def MarginReqmtRptType.field
			return 1638
		end
		def initialize(data = nil)
			if( data == nil )
				super(1638)
			else
				super(1638, data)
			end
		end
	end

	class MarginClass < Quickfix::StringField
		def MarginClass.field
			return 1639
		end
		def initialize(data = nil)
			if( data == nil )
				super(1639)
			else
				super(1639, data)
			end
		end
	end

	class MarginReqmtInqStatus < Quickfix::IntField
		def MarginReqmtInqStatus.field
			return 1640
		end
		def initialize(data = nil)
			if( data == nil )
				super(1640)
			else
				super(1640, data)
			end
		end
	end

	class MarginReqmtInqResult < Quickfix::IntField
		def MarginReqmtInqResult.field
			return 1641
		end
		def initialize(data = nil)
			if( data == nil )
				super(1641)
			else
				super(1641, data)
			end
		end
	end

	class MarginReqmtRptID < Quickfix::StringField
		def MarginReqmtRptID.field
			return 1642
		end
		def initialize(data = nil)
			if( data == nil )
				super(1642)
			else
				super(1642, data)
			end
		end
	end

	class NoMarginAmt < Quickfix::IntField
		def NoMarginAmt.field
			return 1643
		end
		def initialize(data = nil)
			if( data == nil )
				super(1643)
			else
				super(1643, data)
			end
		end
	end

	class MarginAmtType < Quickfix::IntField
		def MarginAmtType.field
			return 1644
		end
		def initialize(data = nil)
			if( data == nil )
				super(1644)
			else
				super(1644, data)
			end
		end
	end

	class MarginAmt < Quickfix::DoubleField
		def MarginAmt.field
			return 1645
		end
		def initialize(data = nil)
			if( data == nil )
				super(1645)
			else
				super(1645, data)
			end
		end
	end

	class MarginAmtCcy < Quickfix::StringField
		def MarginAmtCcy.field
			return 1646
		end
		def initialize(data = nil)
			if( data == nil )
				super(1646)
			else
				super(1646, data)
			end
		end
	end

	class NoRelatedInstruments < Quickfix::IntField
		def NoRelatedInstruments.field
			return 1647
		end
		def initialize(data = nil)
			if( data == nil )
				super(1647)
			else
				super(1647, data)
			end
		end
	end

	class RelatedInstrumentType < Quickfix::IntField
		def RelatedInstrumentType.field
			return 1648
		end
		def initialize(data = nil)
			if( data == nil )
				super(1648)
			else
				super(1648, data)
			end
		end
	end

	class RelatedSymbol < Quickfix::StringField
		def RelatedSymbol.field
			return 1649
		end
		def initialize(data = nil)
			if( data == nil )
				super(1649)
			else
				super(1649, data)
			end
		end
	end

	class RelatedSecurityID < Quickfix::StringField
		def RelatedSecurityID.field
			return 1650
		end
		def initialize(data = nil)
			if( data == nil )
				super(1650)
			else
				super(1650, data)
			end
		end
	end

	class RelatedSecurityIDSource < Quickfix::StringField
		def RelatedSecurityIDSource.field
			return 1651
		end
		def initialize(data = nil)
			if( data == nil )
				super(1651)
			else
				super(1651, data)
			end
		end
	end

	class RelatedSecurityType < Quickfix::StringField
		def RelatedSecurityType.field
			return 1652
		end
		def initialize(data = nil)
			if( data == nil )
				super(1652)
			else
				super(1652, data)
			end
		end
	end

	class RelatedMaturityMonthYear < Quickfix::StringField
		def RelatedMaturityMonthYear.field
			return 1653
		end
		def initialize(data = nil)
			if( data == nil )
				super(1653)
			else
				super(1653, data)
			end
		end
	end

	class CoveredQty < Quickfix::DoubleField
		def CoveredQty.field
			return 1654
		end
		def initialize(data = nil)
			if( data == nil )
				super(1654)
			else
				super(1654, data)
			end
		end
	end

	class MarketMakerActivity < Quickfix::IntField
		def MarketMakerActivity.field
			return 1655
		end
		def initialize(data = nil)
			if( data == nil )
				super(1655)
			else
				super(1655, data)
			end
		end
	end

	class PartyDetailsListRequestID < Quickfix::StringField
		def PartyDetailsListRequestID.field
			return 1505
		end
		def initialize(data = nil)
			if( data == nil )
				super(1505)
			else
				super(1505, data)
			end
		end
	end

	class NoRequestedPartyRoles < Quickfix::IntField
		def NoRequestedPartyRoles.field
			return 1508
		end
		def initialize(data = nil)
			if( data == nil )
				super(1508)
			else
				super(1508, data)
			end
		end
	end

	class RequestedPartyRole < Quickfix::IntField
		def RequestedPartyRole.field
			return 1509
		end
		def initialize(data = nil)
			if( data == nil )
				super(1509)
			else
				super(1509, data)
			end
		end
	end

	class PartyDetailsListReportID < Quickfix::StringField
		def PartyDetailsListReportID.field
			return 1510
		end
		def initialize(data = nil)
			if( data == nil )
				super(1510)
			else
				super(1510, data)
			end
		end
	end

	class RequestResult < Quickfix::IntField
		def RequestResult.field
			return 1511
		end
		def initialize(data = nil)
			if( data == nil )
				super(1511)
			else
				super(1511, data)
			end
		end
	end

	class TotNoParties < Quickfix::IntField
		def TotNoParties.field
			return 1512
		end
		def initialize(data = nil)
			if( data == nil )
				super(1512)
			else
				super(1512, data)
			end
		end
	end

	class NoPartyRelationships < Quickfix::IntField
		def NoPartyRelationships.field
			return 1514
		end
		def initialize(data = nil)
			if( data == nil )
				super(1514)
			else
				super(1514, data)
			end
		end
	end

	class PartyRelationship < Quickfix::IntField
		def PartyRelationship.field
			return 1515
		end
		def initialize(data = nil)
			if( data == nil )
				super(1515)
			else
				super(1515, data)
			end
		end
	end

	class NoPartyDetailAltID < Quickfix::IntField
		def NoPartyDetailAltID.field
			return 1516
		end
		def initialize(data = nil)
			if( data == nil )
				super(1516)
			else
				super(1516, data)
			end
		end
	end

	class PartyDetailAltID < Quickfix::StringField
		def PartyDetailAltID.field
			return 1517
		end
		def initialize(data = nil)
			if( data == nil )
				super(1517)
			else
				super(1517, data)
			end
		end
	end

	class PartyDetailAltIDSource < Quickfix::CharField
		def PartyDetailAltIDSource.field
			return 1518
		end
		def initialize(data = nil)
			if( data == nil )
				super(1518)
			else
				super(1518, data)
			end
		end
	end

	class NoPartyDetailAltSubIDs < Quickfix::IntField
		def NoPartyDetailAltSubIDs.field
			return 1519
		end
		def initialize(data = nil)
			if( data == nil )
				super(1519)
			else
				super(1519, data)
			end
		end
	end

	class PartyDetailAltSubID < Quickfix::StringField
		def PartyDetailAltSubID.field
			return 1520
		end
		def initialize(data = nil)
			if( data == nil )
				super(1520)
			else
				super(1520, data)
			end
		end
	end

	class PartyDetailAltSubIDType < Quickfix::IntField
		def PartyDetailAltSubIDType.field
			return 1521
		end
		def initialize(data = nil)
			if( data == nil )
				super(1521)
			else
				super(1521, data)
			end
		end
	end

	class NoRiskLimitTypes < Quickfix::IntField
		def NoRiskLimitTypes.field
			return 1529
		end
		def initialize(data = nil)
			if( data == nil )
				super(1529)
			else
				super(1529, data)
			end
		end
	end

	class RiskLimitType < Quickfix::IntField
		def RiskLimitType.field
			return 1530
		end
		def initialize(data = nil)
			if( data == nil )
				super(1530)
			else
				super(1530, data)
			end
		end
	end

	class RiskLimitAmount < Quickfix::DoubleField
		def RiskLimitAmount.field
			return 1531
		end
		def initialize(data = nil)
			if( data == nil )
				super(1531)
			else
				super(1531, data)
			end
		end
	end

	class RiskLimitCurrency < Quickfix::StringField
		def RiskLimitCurrency.field
			return 1532
		end
		def initialize(data = nil)
			if( data == nil )
				super(1532)
			else
				super(1532, data)
			end
		end
	end

	class RiskLimitPlatform < Quickfix::StringField
		def RiskLimitPlatform.field
			return 1533
		end
		def initialize(data = nil)
			if( data == nil )
				super(1533)
			else
				super(1533, data)
			end
		end
	end

	class NoRiskInstrumentScopes < Quickfix::IntField
		def NoRiskInstrumentScopes.field
			return 1534
		end
		def initialize(data = nil)
			if( data == nil )
				super(1534)
			else
				super(1534, data)
			end
		end
	end

	class InstrumentScopeOperator < Quickfix::IntField
		def InstrumentScopeOperator.field
			return 1535
		end
		def initialize(data = nil)
			if( data == nil )
				super(1535)
			else
				super(1535, data)
			end
		end
	end

	class InstrumentScopeSymbol < Quickfix::StringField
		def InstrumentScopeSymbol.field
			return 1536
		end
		def initialize(data = nil)
			if( data == nil )
				super(1536)
			else
				super(1536, data)
			end
		end
	end

	class InstrumentScopeSymbolSfx < Quickfix::StringField
		def InstrumentScopeSymbolSfx.field
			return 1537
		end
		def initialize(data = nil)
			if( data == nil )
				super(1537)
			else
				super(1537, data)
			end
		end
	end

	class InstrumentScopeSecurityID < Quickfix::StringField
		def InstrumentScopeSecurityID.field
			return 1538
		end
		def initialize(data = nil)
			if( data == nil )
				super(1538)
			else
				super(1538, data)
			end
		end
	end

	class InstrumentScopeSecurityIDSource < Quickfix::StringField
		def InstrumentScopeSecurityIDSource.field
			return 1539
		end
		def initialize(data = nil)
			if( data == nil )
				super(1539)
			else
				super(1539, data)
			end
		end
	end

	class NoInstrumentScopeSecurityAltID < Quickfix::IntField
		def NoInstrumentScopeSecurityAltID.field
			return 1540
		end
		def initialize(data = nil)
			if( data == nil )
				super(1540)
			else
				super(1540, data)
			end
		end
	end

	class InstrumentScopeSecurityAltID < Quickfix::StringField
		def InstrumentScopeSecurityAltID.field
			return 1541
		end
		def initialize(data = nil)
			if( data == nil )
				super(1541)
			else
				super(1541, data)
			end
		end
	end

	class InstrumentScopeSecurityAltIDSource < Quickfix::StringField
		def InstrumentScopeSecurityAltIDSource.field
			return 1542
		end
		def initialize(data = nil)
			if( data == nil )
				super(1542)
			else
				super(1542, data)
			end
		end
	end

	class InstrumentScopeProduct < Quickfix::IntField
		def InstrumentScopeProduct.field
			return 1543
		end
		def initialize(data = nil)
			if( data == nil )
				super(1543)
			else
				super(1543, data)
			end
		end
	end

	class InstrumentScopeProductComplex < Quickfix::StringField
		def InstrumentScopeProductComplex.field
			return 1544
		end
		def initialize(data = nil)
			if( data == nil )
				super(1544)
			else
				super(1544, data)
			end
		end
	end

	class InstrumentScopeSecurityGroup < Quickfix::StringField
		def InstrumentScopeSecurityGroup.field
			return 1545
		end
		def initialize(data = nil)
			if( data == nil )
				super(1545)
			else
				super(1545, data)
			end
		end
	end

	class InstrumentScopeCFICode < Quickfix::StringField
		def InstrumentScopeCFICode.field
			return 1546
		end
		def initialize(data = nil)
			if( data == nil )
				super(1546)
			else
				super(1546, data)
			end
		end
	end

	class InstrumentScopeSecurityType < Quickfix::StringField
		def InstrumentScopeSecurityType.field
			return 1547
		end
		def initialize(data = nil)
			if( data == nil )
				super(1547)
			else
				super(1547, data)
			end
		end
	end

	class InstrumentScopeSecuritySubType < Quickfix::StringField
		def InstrumentScopeSecuritySubType.field
			return 1548
		end
		def initialize(data = nil)
			if( data == nil )
				super(1548)
			else
				super(1548, data)
			end
		end
	end

	class InstrumentScopeMaturityMonthYear < Quickfix::StringField
		def InstrumentScopeMaturityMonthYear.field
			return 1549
		end
		def initialize(data = nil)
			if( data == nil )
				super(1549)
			else
				super(1549, data)
			end
		end
	end

	class InstrumentScopeMaturityTime < Quickfix::StringField
		def InstrumentScopeMaturityTime.field
			return 1550
		end
		def initialize(data = nil)
			if( data == nil )
				super(1550)
			else
				super(1550, data)
			end
		end
	end

	class InstrumentScopeRestructuringType < Quickfix::StringField
		def InstrumentScopeRestructuringType.field
			return 1551
		end
		def initialize(data = nil)
			if( data == nil )
				super(1551)
			else
				super(1551, data)
			end
		end
	end

	class InstrumentScopeSeniority < Quickfix::StringField
		def InstrumentScopeSeniority.field
			return 1552
		end
		def initialize(data = nil)
			if( data == nil )
				super(1552)
			else
				super(1552, data)
			end
		end
	end

	class InstrumentScopePutOrCall < Quickfix::IntField
		def InstrumentScopePutOrCall.field
			return 1553
		end
		def initialize(data = nil)
			if( data == nil )
				super(1553)
			else
				super(1553, data)
			end
		end
	end

	class InstrumentScopeFlexibleIndicator < Quickfix::BoolField
		def InstrumentScopeFlexibleIndicator.field
			return 1554
		end
		def initialize(data = nil)
			if( data == nil )
				super(1554)
			else
				super(1554, data)
			end
		end
	end

	class InstrumentScopeCouponRate < Quickfix::DoubleField
		def InstrumentScopeCouponRate.field
			return 1555
		end
		def initialize(data = nil)
			if( data == nil )
				super(1555)
			else
				super(1555, data)
			end
		end
	end

	class InstrumentScopeSecurityDesc < Quickfix::StringField
		def InstrumentScopeSecurityDesc.field
			return 1556
		end
		def initialize(data = nil)
			if( data == nil )
				super(1556)
			else
				super(1556, data)
			end
		end
	end

	class InstrumentScopeSettlType < Quickfix::StringField
		def InstrumentScopeSettlType.field
			return 1557
		end
		def initialize(data = nil)
			if( data == nil )
				super(1557)
			else
				super(1557, data)
			end
		end
	end

	class RiskInstrumentMultiplier < Quickfix::DoubleField
		def RiskInstrumentMultiplier.field
			return 1558
		end
		def initialize(data = nil)
			if( data == nil )
				super(1558)
			else
				super(1558, data)
			end
		end
	end

	class NoRiskWarningLevels < Quickfix::IntField
		def NoRiskWarningLevels.field
			return 1559
		end
		def initialize(data = nil)
			if( data == nil )
				super(1559)
			else
				super(1559, data)
			end
		end
	end

	class RiskWarningLevelPercent < Quickfix::DoubleField
		def RiskWarningLevelPercent.field
			return 1560
		end
		def initialize(data = nil)
			if( data == nil )
				super(1560)
			else
				super(1560, data)
			end
		end
	end

	class RiskWarningLevelName < Quickfix::StringField
		def RiskWarningLevelName.field
			return 1561
		end
		def initialize(data = nil)
			if( data == nil )
				super(1561)
			else
				super(1561, data)
			end
		end
	end

	class NoRelatedPartyDetailID < Quickfix::IntField
		def NoRelatedPartyDetailID.field
			return 1562
		end
		def initialize(data = nil)
			if( data == nil )
				super(1562)
			else
				super(1562, data)
			end
		end
	end

	class RelatedPartyDetailID < Quickfix::StringField
		def RelatedPartyDetailID.field
			return 1563
		end
		def initialize(data = nil)
			if( data == nil )
				super(1563)
			else
				super(1563, data)
			end
		end
	end

	class RelatedPartyDetailIDSource < Quickfix::CharField
		def RelatedPartyDetailIDSource.field
			return 1564
		end
		def initialize(data = nil)
			if( data == nil )
				super(1564)
			else
				super(1564, data)
			end
		end
	end

	class RelatedPartyDetailRole < Quickfix::IntField
		def RelatedPartyDetailRole.field
			return 1565
		end
		def initialize(data = nil)
			if( data == nil )
				super(1565)
			else
				super(1565, data)
			end
		end
	end

	class NoRelatedPartyDetailSubIDs < Quickfix::IntField
		def NoRelatedPartyDetailSubIDs.field
			return 1566
		end
		def initialize(data = nil)
			if( data == nil )
				super(1566)
			else
				super(1566, data)
			end
		end
	end

	class RelatedPartyDetailSubID < Quickfix::StringField
		def RelatedPartyDetailSubID.field
			return 1567
		end
		def initialize(data = nil)
			if( data == nil )
				super(1567)
			else
				super(1567, data)
			end
		end
	end

	class RelatedPartyDetailSubIDType < Quickfix::IntField
		def RelatedPartyDetailSubIDType.field
			return 1568
		end
		def initialize(data = nil)
			if( data == nil )
				super(1568)
			else
				super(1568, data)
			end
		end
	end

	class NoRelatedPartyDetailAltID < Quickfix::IntField
		def NoRelatedPartyDetailAltID.field
			return 1569
		end
		def initialize(data = nil)
			if( data == nil )
				super(1569)
			else
				super(1569, data)
			end
		end
	end

	class RelatedPartyDetailAltID < Quickfix::StringField
		def RelatedPartyDetailAltID.field
			return 1570
		end
		def initialize(data = nil)
			if( data == nil )
				super(1570)
			else
				super(1570, data)
			end
		end
	end

	class RelatedPartyDetailAltIDSource < Quickfix::CharField
		def RelatedPartyDetailAltIDSource.field
			return 1571
		end
		def initialize(data = nil)
			if( data == nil )
				super(1571)
			else
				super(1571, data)
			end
		end
	end

	class NoRelatedPartyDetailAltSubIDs < Quickfix::IntField
		def NoRelatedPartyDetailAltSubIDs.field
			return 1572
		end
		def initialize(data = nil)
			if( data == nil )
				super(1572)
			else
				super(1572, data)
			end
		end
	end

	class RelatedPartyDetailAltSubID < Quickfix::StringField
		def RelatedPartyDetailAltSubID.field
			return 1573
		end
		def initialize(data = nil)
			if( data == nil )
				super(1573)
			else
				super(1573, data)
			end
		end
	end

	class RelatedPartyDetailAltSubIDType < Quickfix::IntField
		def RelatedPartyDetailAltSubIDType.field
			return 1574
		end
		def initialize(data = nil)
			if( data == nil )
				super(1574)
			else
				super(1574, data)
			end
		end
	end

	class InstrumentScopeSecurityExchange < Quickfix::StringField
		def InstrumentScopeSecurityExchange.field
			return 1616
		end
		def initialize(data = nil)
			if( data == nil )
				super(1616)
			else
				super(1616, data)
			end
		end
	end

	class InstrumentScopeEncodedSecurityDescLen < Quickfix::IntField
		def InstrumentScopeEncodedSecurityDescLen.field
			return 1620
		end
		def initialize(data = nil)
			if( data == nil )
				super(1620)
			else
				super(1620, data)
			end
		end
	end

	class InstrumentScopeEncodedSecurityDesc < Quickfix::StringField
		def InstrumentScopeEncodedSecurityDesc.field
			return 1621
		end
		def initialize(data = nil)
			if( data == nil )
				super(1621)
			else
				super(1621, data)
			end
		end
	end

	class NoInstrumentScopes < Quickfix::IntField
		def NoInstrumentScopes.field
			return 1656
		end
		def initialize(data = nil)
			if( data == nil )
				super(1656)
			else
				super(1656, data)
			end
		end
	end

	class NoRequestingPartyIDs < Quickfix::IntField
		def NoRequestingPartyIDs.field
			return 1657
		end
		def initialize(data = nil)
			if( data == nil )
				super(1657)
			else
				super(1657, data)
			end
		end
	end

	class RequestingPartyID < Quickfix::StringField
		def RequestingPartyID.field
			return 1658
		end
		def initialize(data = nil)
			if( data == nil )
				super(1658)
			else
				super(1658, data)
			end
		end
	end

	class RequestingPartyIDSource < Quickfix::CharField
		def RequestingPartyIDSource.field
			return 1659
		end
		def initialize(data = nil)
			if( data == nil )
				super(1659)
			else
				super(1659, data)
			end
		end
	end

	class RequestingPartyRole < Quickfix::IntField
		def RequestingPartyRole.field
			return 1660
		end
		def initialize(data = nil)
			if( data == nil )
				super(1660)
			else
				super(1660, data)
			end
		end
	end

	class NoRequestingPartySubIDs < Quickfix::IntField
		def NoRequestingPartySubIDs.field
			return 1661
		end
		def initialize(data = nil)
			if( data == nil )
				super(1661)
			else
				super(1661, data)
			end
		end
	end

	class RequestingPartySubID < Quickfix::StringField
		def RequestingPartySubID.field
			return 1662
		end
		def initialize(data = nil)
			if( data == nil )
				super(1662)
			else
				super(1662, data)
			end
		end
	end

	class RequestingPartySubIDType < Quickfix::IntField
		def RequestingPartySubIDType.field
			return 1663
		end
		def initialize(data = nil)
			if( data == nil )
				super(1663)
			else
				super(1663, data)
			end
		end
	end

	class EncodedRejectTextLen < Quickfix::IntField
		def EncodedRejectTextLen.field
			return 1664
		end
		def initialize(data = nil)
			if( data == nil )
				super(1664)
			else
				super(1664, data)
			end
		end
	end

	class EncodedRejectText < Quickfix::StringField
		def EncodedRejectText.field
			return 1665
		end
		def initialize(data = nil)
			if( data == nil )
				super(1665)
			else
				super(1665, data)
			end
		end
	end

	class RiskLimitRequestID < Quickfix::StringField
		def RiskLimitRequestID.field
			return 1666
		end
		def initialize(data = nil)
			if( data == nil )
				super(1666)
			else
				super(1666, data)
			end
		end
	end

	class RiskLimitReportID < Quickfix::StringField
		def RiskLimitReportID.field
			return 1667
		end
		def initialize(data = nil)
			if( data == nil )
				super(1667)
			else
				super(1667, data)
			end
		end
	end

	class NoRequestedRiskLimitType < Quickfix::IntField
		def NoRequestedRiskLimitType.field
			return 1668
		end
		def initialize(data = nil)
			if( data == nil )
				super(1668)
			else
				super(1668, data)
			end
		end
	end

	class NoRiskLimits < Quickfix::IntField
		def NoRiskLimits.field
			return 1669
		end
		def initialize(data = nil)
			if( data == nil )
				super(1669)
			else
				super(1669, data)
			end
		end
	end

	class RiskLimitID < Quickfix::StringField
		def RiskLimitID.field
			return 1670
		end
		def initialize(data = nil)
			if( data == nil )
				super(1670)
			else
				super(1670, data)
			end
		end
	end

	class NoPartyDetails < Quickfix::IntField
		def NoPartyDetails.field
			return 1671
		end
		def initialize(data = nil)
			if( data == nil )
				super(1671)
			else
				super(1671, data)
			end
		end
	end

	class PartyDetailStatus < Quickfix::IntField
		def PartyDetailStatus.field
			return 1672
		end
		def initialize(data = nil)
			if( data == nil )
				super(1672)
			else
				super(1672, data)
			end
		end
	end

	class PartyDetailRoleQualifier < Quickfix::IntField
		def PartyDetailRoleQualifier.field
			return 1674
		end
		def initialize(data = nil)
			if( data == nil )
				super(1674)
			else
				super(1674, data)
			end
		end
	end

	class RelatedPartyDetailRoleQualifier < Quickfix::IntField
		def RelatedPartyDetailRoleQualifier.field
			return 1675
		end
		def initialize(data = nil)
			if( data == nil )
				super(1675)
			else
				super(1675, data)
			end
		end
	end

	class NoPartyUpdates < Quickfix::IntField
		def NoPartyUpdates.field
			return 1676
		end
		def initialize(data = nil)
			if( data == nil )
				super(1676)
			else
				super(1676, data)
			end
		end
	end

	class NoPartyRiskLimits < Quickfix::IntField
		def NoPartyRiskLimits.field
			return 1677
		end
		def initialize(data = nil)
			if( data == nil )
				super(1677)
			else
				super(1677, data)
			end
		end
	end

	class PartyDetailID < Quickfix::StringField
		def PartyDetailID.field
			return 1691
		end
		def initialize(data = nil)
			if( data == nil )
				super(1691)
			else
				super(1691, data)
			end
		end
	end

	class PartyDetailIDSource < Quickfix::CharField
		def PartyDetailIDSource.field
			return 1692
		end
		def initialize(data = nil)
			if( data == nil )
				super(1692)
			else
				super(1692, data)
			end
		end
	end

	class PartyDetailRole < Quickfix::IntField
		def PartyDetailRole.field
			return 1693
		end
		def initialize(data = nil)
			if( data == nil )
				super(1693)
			else
				super(1693, data)
			end
		end
	end

	class NoPartyDetailSubIDs < Quickfix::IntField
		def NoPartyDetailSubIDs.field
			return 1694
		end
		def initialize(data = nil)
			if( data == nil )
				super(1694)
			else
				super(1694, data)
			end
		end
	end

	class PartyDetailSubID < Quickfix::StringField
		def PartyDetailSubID.field
			return 1695
		end
		def initialize(data = nil)
			if( data == nil )
				super(1695)
			else
				super(1695, data)
			end
		end
	end

	class PartyDetailSubIDType < Quickfix::IntField
		def PartyDetailSubIDType.field
			return 1696
		end
		def initialize(data = nil)
			if( data == nil )
				super(1696)
			else
				super(1696, data)
			end
		end
	end

	class SecurityMassTradingStatus < Quickfix::IntField
		def SecurityMassTradingStatus.field
			return 1679
		end
		def initialize(data = nil)
			if( data == nil )
				super(1679)
			else
				super(1679, data)
			end
		end
	end

	class SecurityMassTradingEvent < Quickfix::IntField
		def SecurityMassTradingEvent.field
			return 1680
		end
		def initialize(data = nil)
			if( data == nil )
				super(1680)
			else
				super(1680, data)
			end
		end
	end

	class MassHaltReason < Quickfix::IntField
		def MassHaltReason.field
			return 1681
		end
		def initialize(data = nil)
			if( data == nil )
				super(1681)
			else
				super(1681, data)
			end
		end
	end

	class MDSecurityTradingStatus < Quickfix::IntField
		def MDSecurityTradingStatus.field
			return 1682
		end
		def initialize(data = nil)
			if( data == nil )
				super(1682)
			else
				super(1682, data)
			end
		end
	end

	class MDSubFeedType < Quickfix::StringField
		def MDSubFeedType.field
			return 1683
		end
		def initialize(data = nil)
			if( data == nil )
				super(1683)
			else
				super(1683, data)
			end
		end
	end

	class MDHaltReason < Quickfix::IntField
		def MDHaltReason.field
			return 1684
		end
		def initialize(data = nil)
			if( data == nil )
				super(1684)
			else
				super(1684, data)
			end
		end
	end

	class SideTradeID < Quickfix::StringField
		def SideTradeID.field
			return 1506
		end
		def initialize(data = nil)
			if( data == nil )
				super(1506)
			else
				super(1506, data)
			end
		end
	end

	class SideOrigTradeID < Quickfix::StringField
		def SideOrigTradeID.field
			return 1507
		end
		def initialize(data = nil)
			if( data == nil )
				super(1507)
			else
				super(1507, data)
			end
		end
	end

	class DifferentialPrice < Quickfix::DoubleField
		def DifferentialPrice.field
			return 1522
		end
		def initialize(data = nil)
			if( data == nil )
				super(1522)
			else
				super(1522, data)
			end
		end
	end

	class TrdAckStatus < Quickfix::IntField
		def TrdAckStatus.field
			return 1523
		end
		def initialize(data = nil)
			if( data == nil )
				super(1523)
			else
				super(1523, data)
			end
		end
	end

	class PriceQuoteCurrency < Quickfix::StringField
		def PriceQuoteCurrency.field
			return 1524
		end
		def initialize(data = nil)
			if( data == nil )
				super(1524)
			else
				super(1524, data)
			end
		end
	end

	class UnderlyingPriceQuoteCurrency < Quickfix::StringField
		def UnderlyingPriceQuoteCurrency.field
			return 1526
		end
		def initialize(data = nil)
			if( data == nil )
				super(1526)
			else
				super(1526, data)
			end
		end
	end

	class LegPriceQuoteCurrency < Quickfix::StringField
		def LegPriceQuoteCurrency.field
			return 1528
		end
		def initialize(data = nil)
			if( data == nil )
				super(1528)
			else
				super(1528, data)
			end
		end
	end

	class DerivativePriceQuoteCurrency < Quickfix::StringField
		def DerivativePriceQuoteCurrency.field
			return 1576
		end
		def initialize(data = nil)
			if( data == nil )
				super(1576)
			else
				super(1576, data)
			end
		end
	end

	class NoSecurityClassifications < Quickfix::IntField
		def NoSecurityClassifications.field
			return 1582
		end
		def initialize(data = nil)
			if( data == nil )
				super(1582)
			else
				super(1582, data)
			end
		end
	end

	class SecurityClassificationReason < Quickfix::IntField
		def SecurityClassificationReason.field
			return 1583
		end
		def initialize(data = nil)
			if( data == nil )
				super(1583)
			else
				super(1583, data)
			end
		end
	end

	class SecurityClassificationValue < Quickfix::StringField
		def SecurityClassificationValue.field
			return 1584
		end
		def initialize(data = nil)
			if( data == nil )
				super(1584)
			else
				super(1584, data)
			end
		end
	end

	class PosAmtReason < Quickfix::IntField
		def PosAmtReason.field
			return 1585
		end
		def initialize(data = nil)
			if( data == nil )
				super(1585)
			else
				super(1585, data)
			end
		end
	end

	class NoLegPosAmt < Quickfix::IntField
		def NoLegPosAmt.field
			return 1586
		end
		def initialize(data = nil)
			if( data == nil )
				super(1586)
			else
				super(1586, data)
			end
		end
	end

	class LegPosAmt < Quickfix::DoubleField
		def LegPosAmt.field
			return 1587
		end
		def initialize(data = nil)
			if( data == nil )
				super(1587)
			else
				super(1587, data)
			end
		end
	end

	class LegPosAmtType < Quickfix::StringField
		def LegPosAmtType.field
			return 1588
		end
		def initialize(data = nil)
			if( data == nil )
				super(1588)
			else
				super(1588, data)
			end
		end
	end

	class LegPosCurrency < Quickfix::StringField
		def LegPosCurrency.field
			return 1589
		end
		def initialize(data = nil)
			if( data == nil )
				super(1589)
			else
				super(1589, data)
			end
		end
	end

	class LegPosAmtReason < Quickfix::IntField
		def LegPosAmtReason.field
			return 1590
		end
		def initialize(data = nil)
			if( data == nil )
				super(1590)
			else
				super(1590, data)
			end
		end
	end

	class LegQtyType < Quickfix::IntField
		def LegQtyType.field
			return 1591
		end
		def initialize(data = nil)
			if( data == nil )
				super(1591)
			else
				super(1591, data)
			end
		end
	end

	class DiscountFactor < Quickfix::DoubleField
		def DiscountFactor.field
			return 1592
		end
		def initialize(data = nil)
			if( data == nil )
				super(1592)
			else
				super(1592, data)
			end
		end
	end

	class ParentAllocID < Quickfix::StringField
		def ParentAllocID.field
			return 1593
		end
		def initialize(data = nil)
			if( data == nil )
				super(1593)
			else
				super(1593, data)
			end
		end
	end

	class LegSecurityGroup < Quickfix::StringField
		def LegSecurityGroup.field
			return 1594
		end
		def initialize(data = nil)
			if( data == nil )
				super(1594)
			else
				super(1594, data)
			end
		end
	end

	class PositionContingentPrice < Quickfix::DoubleField
		def PositionContingentPrice.field
			return 1595
		end
		def initialize(data = nil)
			if( data == nil )
				super(1595)
			else
				super(1595, data)
			end
		end
	end

	class ClearingTradePrice < Quickfix::DoubleField
		def ClearingTradePrice.field
			return 1596
		end
		def initialize(data = nil)
			if( data == nil )
				super(1596)
			else
				super(1596, data)
			end
		end
	end

	class SideClearingTradePrice < Quickfix::DoubleField
		def SideClearingTradePrice.field
			return 1597
		end
		def initialize(data = nil)
			if( data == nil )
				super(1597)
			else
				super(1597, data)
			end
		end
	end

	class SideClearingTradePriceType < Quickfix::IntField
		def SideClearingTradePriceType.field
			return 1598
		end
		def initialize(data = nil)
			if( data == nil )
				super(1598)
			else
				super(1598, data)
			end
		end
	end

	class SidePriceDifferential < Quickfix::DoubleField
		def SidePriceDifferential.field
			return 1599
		end
		def initialize(data = nil)
			if( data == nil )
				super(1599)
			else
				super(1599, data)
			end
		end
	end

	class FIXEngineName < Quickfix::StringField
		def FIXEngineName.field
			return 1600
		end
		def initialize(data = nil)
			if( data == nil )
				super(1600)
			else
				super(1600, data)
			end
		end
	end

	class FIXEngineVersion < Quickfix::StringField
		def FIXEngineVersion.field
			return 1601
		end
		def initialize(data = nil)
			if( data == nil )
				super(1601)
			else
				super(1601, data)
			end
		end
	end

	class FIXEngineVendor < Quickfix::StringField
		def FIXEngineVendor.field
			return 1602
		end
		def initialize(data = nil)
			if( data == nil )
				super(1602)
			else
				super(1602, data)
			end
		end
	end

	class ApplicationSystemName < Quickfix::StringField
		def ApplicationSystemName.field
			return 1603
		end
		def initialize(data = nil)
			if( data == nil )
				super(1603)
			else
				super(1603, data)
			end
		end
	end

	class ApplicationSystemVersion < Quickfix::StringField
		def ApplicationSystemVersion.field
			return 1604
		end
		def initialize(data = nil)
			if( data == nil )
				super(1604)
			else
				super(1604, data)
			end
		end
	end

	class ApplicationSystemVendor < Quickfix::StringField
		def ApplicationSystemVendor.field
			return 1605
		end
		def initialize(data = nil)
			if( data == nil )
				super(1605)
			else
				super(1605, data)
			end
		end
	end

	class NumOfSimpleInstruments < Quickfix::IntField
		def NumOfSimpleInstruments.field
			return 1606
		end
		def initialize(data = nil)
			if( data == nil )
				super(1606)
			else
				super(1606, data)
			end
		end
	end

	class SecurityRejectReason < Quickfix::IntField
		def SecurityRejectReason.field
			return 1607
		end
		def initialize(data = nil)
			if( data == nil )
				super(1607)
			else
				super(1607, data)
			end
		end
	end

	class InitialDisplayQty < Quickfix::DoubleField
		def InitialDisplayQty.field
			return 1608
		end
		def initialize(data = nil)
			if( data == nil )
				super(1608)
			else
				super(1608, data)
			end
		end
	end

	class ThrottleStatus < Quickfix::IntField
		def ThrottleStatus.field
			return 1609
		end
		def initialize(data = nil)
			if( data == nil )
				super(1609)
			else
				super(1609, data)
			end
		end
	end

	class NoThrottles < Quickfix::IntField
		def NoThrottles.field
			return 1610
		end
		def initialize(data = nil)
			if( data == nil )
				super(1610)
			else
				super(1610, data)
			end
		end
	end

	class ThrottleAction < Quickfix::IntField
		def ThrottleAction.field
			return 1611
		end
		def initialize(data = nil)
			if( data == nil )
				super(1611)
			else
				super(1611, data)
			end
		end
	end

	class ThrottleType < Quickfix::IntField
		def ThrottleType.field
			return 1612
		end
		def initialize(data = nil)
			if( data == nil )
				super(1612)
			else
				super(1612, data)
			end
		end
	end

	class ThrottleNoMsgs < Quickfix::IntField
		def ThrottleNoMsgs.field
			return 1613
		end
		def initialize(data = nil)
			if( data == nil )
				super(1613)
			else
				super(1613, data)
			end
		end
	end

	class ThrottleTimeInterval < Quickfix::IntField
		def ThrottleTimeInterval.field
			return 1614
		end
		def initialize(data = nil)
			if( data == nil )
				super(1614)
			else
				super(1614, data)
			end
		end
	end

	class ThrottleTimeUnit < Quickfix::IntField
		def ThrottleTimeUnit.field
			return 1615
		end
		def initialize(data = nil)
			if( data == nil )
				super(1615)
			else
				super(1615, data)
			end
		end
	end

	class NoThrottleMsgType < Quickfix::IntField
		def NoThrottleMsgType.field
			return 1618
		end
		def initialize(data = nil)
			if( data == nil )
				super(1618)
			else
				super(1618, data)
			end
		end
	end

	class ThrottleMsgType < Quickfix::StringField
		def ThrottleMsgType.field
			return 1619
		end
		def initialize(data = nil)
			if( data == nil )
				super(1619)
			else
				super(1619, data)
			end
		end
	end

	class ThrottleInst < Quickfix::IntField
		def ThrottleInst.field
			return 1685
		end
		def initialize(data = nil)
			if( data == nil )
				super(1685)
			else
				super(1685, data)
			end
		end
	end

	class ThrottleCountIndicator < Quickfix::IntField
		def ThrottleCountIndicator.field
			return 1686
		end
		def initialize(data = nil)
			if( data == nil )
				super(1686)
			else
				super(1686, data)
			end
		end
	end

	class AccountSummaryReportID < Quickfix::StringField
		def AccountSummaryReportID.field
			return 1699
		end
		def initialize(data = nil)
			if( data == nil )
				super(1699)
			else
				super(1699, data)
			end
		end
	end

	class NoSettlementAmounts < Quickfix::IntField
		def NoSettlementAmounts.field
			return 1700
		end
		def initialize(data = nil)
			if( data == nil )
				super(1700)
			else
				super(1700, data)
			end
		end
	end

	class SettlementAmount < Quickfix::DoubleField
		def SettlementAmount.field
			return 1701
		end
		def initialize(data = nil)
			if( data == nil )
				super(1701)
			else
				super(1701, data)
			end
		end
	end

	class SettlementAmountCurrency < Quickfix::StringField
		def SettlementAmountCurrency.field
			return 1702
		end
		def initialize(data = nil)
			if( data == nil )
				super(1702)
			else
				super(1702, data)
			end
		end
	end

	class NoCollateralAmounts < Quickfix::IntField
		def NoCollateralAmounts.field
			return 1703
		end
		def initialize(data = nil)
			if( data == nil )
				super(1703)
			else
				super(1703, data)
			end
		end
	end

	class CurrentCollateralAmount < Quickfix::DoubleField
		def CurrentCollateralAmount.field
			return 1704
		end
		def initialize(data = nil)
			if( data == nil )
				super(1704)
			else
				super(1704, data)
			end
		end
	end

	class CollateralCurrency < Quickfix::StringField
		def CollateralCurrency.field
			return 1705
		end
		def initialize(data = nil)
			if( data == nil )
				super(1705)
			else
				super(1705, data)
			end
		end
	end

	class CollateralType < Quickfix::StringField
		def CollateralType.field
			return 1706
		end
		def initialize(data = nil)
			if( data == nil )
				super(1706)
			else
				super(1706, data)
			end
		end
	end

	class NoPayCollects < Quickfix::IntField
		def NoPayCollects.field
			return 1707
		end
		def initialize(data = nil)
			if( data == nil )
				super(1707)
			else
				super(1707, data)
			end
		end
	end

	class PayAmount < Quickfix::DoubleField
		def PayAmount.field
			return 1710
		end
		def initialize(data = nil)
			if( data == nil )
				super(1710)
			else
				super(1710, data)
			end
		end
	end

	class CollectAmount < Quickfix::DoubleField
		def CollectAmount.field
			return 1711
		end
		def initialize(data = nil)
			if( data == nil )
				super(1711)
			else
				super(1711, data)
			end
		end
	end

	class PayCollectType < Quickfix::StringField
		def PayCollectType.field
			return 1708
		end
		def initialize(data = nil)
			if( data == nil )
				super(1708)
			else
				super(1708, data)
			end
		end
	end

	class PayCollectCurrency < Quickfix::StringField
		def PayCollectCurrency.field
			return 1709
		end
		def initialize(data = nil)
			if( data == nil )
				super(1709)
			else
				super(1709, data)
			end
		end
	end

	class PayCollectMarketSegmentID < Quickfix::StringField
		def PayCollectMarketSegmentID.field
			return 1712
		end
		def initialize(data = nil)
			if( data == nil )
				super(1712)
			else
				super(1712, data)
			end
		end
	end

	class PayCollectMarketID < Quickfix::StringField
		def PayCollectMarketID.field
			return 1713
		end
		def initialize(data = nil)
			if( data == nil )
				super(1713)
			else
				super(1713, data)
			end
		end
	end

	class MarginAmountMarketSegmentID < Quickfix::StringField
		def MarginAmountMarketSegmentID.field
			return 1714
		end
		def initialize(data = nil)
			if( data == nil )
				super(1714)
			else
				super(1714, data)
			end
		end
	end

	class MarginAmountMarketID < Quickfix::StringField
		def MarginAmountMarketID.field
			return 1715
		end
		def initialize(data = nil)
			if( data == nil )
				super(1715)
			else
				super(1715, data)
			end
		end
	end

	class FirmGroupID < Quickfix::StringField
		def FirmGroupID.field
			return 1728
		end
		def initialize(data = nil)
			if( data == nil )
				super(1728)
			else
				super(1728, data)
			end
		end
	end

	class FirmMnemonic < Quickfix::StringField
		def FirmMnemonic.field
			return 1729
		end
		def initialize(data = nil)
			if( data == nil )
				super(1729)
			else
				super(1729, data)
			end
		end
	end

	class AllocGroupID < Quickfix::StringField
		def AllocGroupID.field
			return 1730
		end
		def initialize(data = nil)
			if( data == nil )
				super(1730)
			else
				super(1730, data)
			end
		end
	end

	class AvgPxGroupID < Quickfix::StringField
		def AvgPxGroupID.field
			return 1731
		end
		def initialize(data = nil)
			if( data == nil )
				super(1731)
			else
				super(1731, data)
			end
		end
	end

	class FirmAllocText < Quickfix::StringField
		def FirmAllocText.field
			return 1732
		end
		def initialize(data = nil)
			if( data == nil )
				super(1732)
			else
				super(1732, data)
			end
		end
	end

	class EncodedFirmAllocTextLen < Quickfix::IntField
		def EncodedFirmAllocTextLen.field
			return 1733
		end
		def initialize(data = nil)
			if( data == nil )
				super(1733)
			else
				super(1733, data)
			end
		end
	end

	class EncodedFirmAllocText < Quickfix::StringField
		def EncodedFirmAllocText.field
			return 1734
		end
		def initialize(data = nil)
			if( data == nil )
				super(1734)
			else
				super(1734, data)
			end
		end
	end

	class AllocationRollupInstruction < Quickfix::IntField
		def AllocationRollupInstruction.field
			return 1735
		end
		def initialize(data = nil)
			if( data == nil )
				super(1735)
			else
				super(1735, data)
			end
		end
	end

	class AllocGroupQuantity < Quickfix::DoubleField
		def AllocGroupQuantity.field
			return 1736
		end
		def initialize(data = nil)
			if( data == nil )
				super(1736)
			else
				super(1736, data)
			end
		end
	end

	class AllocGroupRemainingQuantity < Quickfix::DoubleField
		def AllocGroupRemainingQuantity.field
			return 1737
		end
		def initialize(data = nil)
			if( data == nil )
				super(1737)
			else
				super(1737, data)
			end
		end
	end

	class AllocReversalStatus < Quickfix::IntField
		def AllocReversalStatus.field
			return 1738
		end
		def initialize(data = nil)
			if( data == nil )
				super(1738)
			else
				super(1738, data)
			end
		end
	end

	class ObligationType < Quickfix::StringField
		def ObligationType.field
			return 1739
		end
		def initialize(data = nil)
			if( data == nil )
				super(1739)
			else
				super(1739, data)
			end
		end
	end

	class TradePriceNegotiationMethod < Quickfix::IntField
		def TradePriceNegotiationMethod.field
			return 1740
		end
		def initialize(data = nil)
			if( data == nil )
				super(1740)
			else
				super(1740, data)
			end
		end
	end

	class UpfrontPriceType < Quickfix::IntField
		def UpfrontPriceType.field
			return 1741
		end
		def initialize(data = nil)
			if( data == nil )
				super(1741)
			else
				super(1741, data)
			end
		end
	end

	class UpfrontPrice < Quickfix::DoubleField
		def UpfrontPrice.field
			return 1742
		end
		def initialize(data = nil)
			if( data == nil )
				super(1742)
			else
				super(1742, data)
			end
		end
	end

	class LastUpfrontPrice < Quickfix::DoubleField
		def LastUpfrontPrice.field
			return 1743
		end
		def initialize(data = nil)
			if( data == nil )
				super(1743)
			else
				super(1743, data)
			end
		end
	end

	class ShortSaleRestriction < Quickfix::IntField
		def ShortSaleRestriction.field
			return 1687
		end
		def initialize(data = nil)
			if( data == nil )
				super(1687)
			else
				super(1687, data)
			end
		end
	end

	class ShortSaleExemptionReason < Quickfix::IntField
		def ShortSaleExemptionReason.field
			return 1688
		end
		def initialize(data = nil)
			if( data == nil )
				super(1688)
			else
				super(1688, data)
			end
		end
	end

	class LegShortSaleExemptionReason < Quickfix::IntField
		def LegShortSaleExemptionReason.field
			return 1689
		end
		def initialize(data = nil)
			if( data == nil )
				super(1689)
			else
				super(1689, data)
			end
		end
	end

	class SideShortSaleExemptionReason < Quickfix::IntField
		def SideShortSaleExemptionReason.field
			return 1690
		end
		def initialize(data = nil)
			if( data == nil )
				super(1690)
			else
				super(1690, data)
			end
		end
	end

	class UnitOfMeasureCurrency < Quickfix::StringField
		def UnitOfMeasureCurrency.field
			return 1716
		end
		def initialize(data = nil)
			if( data == nil )
				super(1716)
			else
				super(1716, data)
			end
		end
	end

	class PriceUnitOfMeasureCurrency < Quickfix::StringField
		def PriceUnitOfMeasureCurrency.field
			return 1717
		end
		def initialize(data = nil)
			if( data == nil )
				super(1717)
			else
				super(1717, data)
			end
		end
	end

	class UnderlyingUnitOfMeasureCurrency < Quickfix::StringField
		def UnderlyingUnitOfMeasureCurrency.field
			return 1718
		end
		def initialize(data = nil)
			if( data == nil )
				super(1718)
			else
				super(1718, data)
			end
		end
	end

	class UnderlyingPriceUnitOfMeasureCurrency < Quickfix::StringField
		def UnderlyingPriceUnitOfMeasureCurrency.field
			return 1719
		end
		def initialize(data = nil)
			if( data == nil )
				super(1719)
			else
				super(1719, data)
			end
		end
	end

	class LegUnitOfMeasureCurrency < Quickfix::StringField
		def LegUnitOfMeasureCurrency.field
			return 1720
		end
		def initialize(data = nil)
			if( data == nil )
				super(1720)
			else
				super(1720, data)
			end
		end
	end

	class LegPriceUnitOfMeasureCurrency < Quickfix::StringField
		def LegPriceUnitOfMeasureCurrency.field
			return 1721
		end
		def initialize(data = nil)
			if( data == nil )
				super(1721)
			else
				super(1721, data)
			end
		end
	end

	class DerivativeUnitOfMeasureCurrency < Quickfix::StringField
		def DerivativeUnitOfMeasureCurrency.field
			return 1722
		end
		def initialize(data = nil)
			if( data == nil )
				super(1722)
			else
				super(1722, data)
			end
		end
	end

	class DerivativePriceUnitOfMeasureCurrency < Quickfix::StringField
		def DerivativePriceUnitOfMeasureCurrency.field
			return 1723
		end
		def initialize(data = nil)
			if( data == nil )
				super(1723)
			else
				super(1723, data)
			end
		end
	end

	class ApplLevelRecoveryIndicator < Quickfix::IntField
		def ApplLevelRecoveryIndicator.field
			return 1744
		end
		def initialize(data = nil)
			if( data == nil )
				super(1744)
			else
				super(1744, data)
			end
		end
	end

	class BidMDEntryID < Quickfix::StringField
		def BidMDEntryID.field
			return 1745
		end
		def initialize(data = nil)
			if( data == nil )
				super(1745)
			else
				super(1745, data)
			end
		end
	end

	class OfferMDEntryID < Quickfix::StringField
		def OfferMDEntryID.field
			return 1746
		end
		def initialize(data = nil)
			if( data == nil )
				super(1746)
			else
				super(1746, data)
			end
		end
	end

	class BidQuoteID < Quickfix::StringField
		def BidQuoteID.field
			return 1747
		end
		def initialize(data = nil)
			if( data == nil )
				super(1747)
			else
				super(1747, data)
			end
		end
	end

	class OfferQuoteID < Quickfix::StringField
		def OfferQuoteID.field
			return 1748
		end
		def initialize(data = nil)
			if( data == nil )
				super(1748)
			else
				super(1748, data)
			end
		end
	end

	class TotalBidSize < Quickfix::DoubleField
		def TotalBidSize.field
			return 1749
		end
		def initialize(data = nil)
			if( data == nil )
				super(1749)
			else
				super(1749, data)
			end
		end
	end

	class TotalOfferSize < Quickfix::DoubleField
		def TotalOfferSize.field
			return 1750
		end
		def initialize(data = nil)
			if( data == nil )
				super(1750)
			else
				super(1750, data)
			end
		end
	end

	class SecondaryQuoteID < Quickfix::StringField
		def SecondaryQuoteID.field
			return 1751
		end
		def initialize(data = nil)
			if( data == nil )
				super(1751)
			else
				super(1751, data)
			end
		end
	end

	class CustodialLotID < Quickfix::StringField
		def CustodialLotID.field
			return 1752
		end
		def initialize(data = nil)
			if( data == nil )
				super(1752)
			else
				super(1752, data)
			end
		end
	end

	class VersusPurchaseDate < Quickfix::StringField
		def VersusPurchaseDate.field
			return 1753
		end
		def initialize(data = nil)
			if( data == nil )
				super(1753)
			else
				super(1753, data)
			end
		end
	end

	class VersusPurchasePrice < Quickfix::DoubleField
		def VersusPurchasePrice.field
			return 1754
		end
		def initialize(data = nil)
			if( data == nil )
				super(1754)
			else
				super(1754, data)
			end
		end
	end

	class CurrentCostBasis < Quickfix::DoubleField
		def CurrentCostBasis.field
			return 1755
		end
		def initialize(data = nil)
			if( data == nil )
				super(1755)
			else
				super(1755, data)
			end
		end
	end

	class LegCustodialLotID < Quickfix::StringField
		def LegCustodialLotID.field
			return 1756
		end
		def initialize(data = nil)
			if( data == nil )
				super(1756)
			else
				super(1756, data)
			end
		end
	end

	class LegVersusPurchaseDate < Quickfix::StringField
		def LegVersusPurchaseDate.field
			return 1757
		end
		def initialize(data = nil)
			if( data == nil )
				super(1757)
			else
				super(1757, data)
			end
		end
	end

	class LegVersusPurchasePrice < Quickfix::DoubleField
		def LegVersusPurchasePrice.field
			return 1758
		end
		def initialize(data = nil)
			if( data == nil )
				super(1758)
			else
				super(1758, data)
			end
		end
	end

	class LegCurrentCostBasis < Quickfix::DoubleField
		def LegCurrentCostBasis.field
			return 1759
		end
		def initialize(data = nil)
			if( data == nil )
				super(1759)
			else
				super(1759, data)
			end
		end
	end

	class RiskLimitRequestType < Quickfix::IntField
		def RiskLimitRequestType.field
			return 1760
		end
		def initialize(data = nil)
			if( data == nil )
				super(1760)
			else
				super(1760, data)
			end
		end
	end

	class RiskLimitRequestResult < Quickfix::IntField
		def RiskLimitRequestResult.field
			return 1761
		end
		def initialize(data = nil)
			if( data == nil )
				super(1761)
			else
				super(1761, data)
			end
		end
	end

	class RiskLimitRequestStatus < Quickfix::IntField
		def RiskLimitRequestStatus.field
			return 1762
		end
		def initialize(data = nil)
			if( data == nil )
				super(1762)
			else
				super(1762, data)
			end
		end
	end

	class RiskLimitStatus < Quickfix::IntField
		def RiskLimitStatus.field
			return 1763
		end
		def initialize(data = nil)
			if( data == nil )
				super(1763)
			else
				super(1763, data)
			end
		end
	end

	class RiskLimitResult < Quickfix::IntField
		def RiskLimitResult.field
			return 1764
		end
		def initialize(data = nil)
			if( data == nil )
				super(1764)
			else
				super(1764, data)
			end
		end
	end

	class RiskLimitUtilizationPercent < Quickfix::DoubleField
		def RiskLimitUtilizationPercent.field
			return 1765
		end
		def initialize(data = nil)
			if( data == nil )
				super(1765)
			else
				super(1765, data)
			end
		end
	end

	class RiskLimitUtilizationAmount < Quickfix::DoubleField
		def RiskLimitUtilizationAmount.field
			return 1766
		end
		def initialize(data = nil)
			if( data == nil )
				super(1766)
			else
				super(1766, data)
			end
		end
	end

	class RiskLimitAction < Quickfix::IntField
		def RiskLimitAction.field
			return 1767
		end
		def initialize(data = nil)
			if( data == nil )
				super(1767)
			else
				super(1767, data)
			end
		end
	end

	class RiskWarningLevelAmount < Quickfix::IntField
		def RiskWarningLevelAmount.field
			return 1768
		end
		def initialize(data = nil)
			if( data == nil )
				super(1768)
			else
				super(1768, data)
			end
		end
	end

	class RiskWarningLevelAction < Quickfix::IntField
		def RiskWarningLevelAction.field
			return 1769
		end
		def initialize(data = nil)
			if( data == nil )
				super(1769)
			else
				super(1769, data)
			end
		end
	end

	class EntitlementRequestID < Quickfix::StringField
		def EntitlementRequestID.field
			return 1770
		end
		def initialize(data = nil)
			if( data == nil )
				super(1770)
			else
				super(1770, data)
			end
		end
	end

	class EntitlementReportID < Quickfix::StringField
		def EntitlementReportID.field
			return 1771
		end
		def initialize(data = nil)
			if( data == nil )
				super(1771)
			else
				super(1771, data)
			end
		end
	end

	class NoPartyEntitlements < Quickfix::IntField
		def NoPartyEntitlements.field
			return 1772
		end
		def initialize(data = nil)
			if( data == nil )
				super(1772)
			else
				super(1772, data)
			end
		end
	end

	class NoEntitlements < Quickfix::IntField
		def NoEntitlements.field
			return 1773
		end
		def initialize(data = nil)
			if( data == nil )
				super(1773)
			else
				super(1773, data)
			end
		end
	end

	class EntitlementIndicator < Quickfix::BoolField
		def EntitlementIndicator.field
			return 1774
		end
		def initialize(data = nil)
			if( data == nil )
				super(1774)
			else
				super(1774, data)
			end
		end
	end

	class EntitlementType < Quickfix::IntField
		def EntitlementType.field
			return 1775
		end
		def initialize(data = nil)
			if( data == nil )
				super(1775)
			else
				super(1775, data)
			end
		end
	end

	class EntitlementID < Quickfix::StringField
		def EntitlementID.field
			return 1776
		end
		def initialize(data = nil)
			if( data == nil )
				super(1776)
			else
				super(1776, data)
			end
		end
	end

	class NoEntitlementAttrib < Quickfix::IntField
		def NoEntitlementAttrib.field
			return 1777
		end
		def initialize(data = nil)
			if( data == nil )
				super(1777)
			else
				super(1777, data)
			end
		end
	end

	class EntitlementAttribType < Quickfix::IntField
		def EntitlementAttribType.field
			return 1778
		end
		def initialize(data = nil)
			if( data == nil )
				super(1778)
			else
				super(1778, data)
			end
		end
	end

	class EntitlementAttribDatatype < Quickfix::IntField
		def EntitlementAttribDatatype.field
			return 1779
		end
		def initialize(data = nil)
			if( data == nil )
				super(1779)
			else
				super(1779, data)
			end
		end
	end

	class EntitlementAttribValue < Quickfix::StringField
		def EntitlementAttribValue.field
			return 1780
		end
		def initialize(data = nil)
			if( data == nil )
				super(1780)
			else
				super(1780, data)
			end
		end
	end

	class EntitlementAttribCurrency < Quickfix::StringField
		def EntitlementAttribCurrency.field
			return 1781
		end
		def initialize(data = nil)
			if( data == nil )
				super(1781)
			else
				super(1781, data)
			end
		end
	end

	class EntitlementStartDate < Quickfix::StringField
		def EntitlementStartDate.field
			return 1782
		end
		def initialize(data = nil)
			if( data == nil )
				super(1782)
			else
				super(1782, data)
			end
		end
	end

	class EntitlementEndDate < Quickfix::StringField
		def EntitlementEndDate.field
			return 1783
		end
		def initialize(data = nil)
			if( data == nil )
				super(1783)
			else
				super(1783, data)
			end
		end
	end

	class EntitlementPlatform < Quickfix::StringField
		def EntitlementPlatform.field
			return 1784
		end
		def initialize(data = nil)
			if( data == nil )
				super(1784)
			else
				super(1784, data)
			end
		end
	end

	class TradSesControl < Quickfix::IntField
		def TradSesControl.field
			return 1785
		end
		def initialize(data = nil)
			if( data == nil )
				super(1785)
			else
				super(1785, data)
			end
		end
	end

	class TradeVolType < Quickfix::IntField
		def TradeVolType.field
			return 1786
		end
		def initialize(data = nil)
			if( data == nil )
				super(1786)
			else
				super(1786, data)
			end
		end
	end

	class RefTickTableID < Quickfix::IntField
		def RefTickTableID.field
			return 1787
		end
		def initialize(data = nil)
			if( data == nil )
				super(1787)
			else
				super(1787, data)
			end
		end
	end

	class LegID < Quickfix::StringField
		def LegID.field
			return 1788
		end
		def initialize(data = nil)
			if( data == nil )
				super(1788)
			else
				super(1788, data)
			end
		end
	end

	class NoTargetMarketSegments < Quickfix::IntField
		def NoTargetMarketSegments.field
			return 1789
		end
		def initialize(data = nil)
			if( data == nil )
				super(1789)
			else
				super(1789, data)
			end
		end
	end

	class TargetMarketSegmentID < Quickfix::StringField
		def TargetMarketSegmentID.field
			return 1790
		end
		def initialize(data = nil)
			if( data == nil )
				super(1790)
			else
				super(1790, data)
			end
		end
	end

	class NoAffectedMarketSegments < Quickfix::IntField
		def NoAffectedMarketSegments.field
			return 1791
		end
		def initialize(data = nil)
			if( data == nil )
				super(1791)
			else
				super(1791, data)
			end
		end
	end

	class AffectedMarketSegmentID < Quickfix::StringField
		def AffectedMarketSegmentID.field
			return 1792
		end
		def initialize(data = nil)
			if( data == nil )
				super(1792)
			else
				super(1792, data)
			end
		end
	end

	class NoNotAffectedMarketSegments < Quickfix::IntField
		def NoNotAffectedMarketSegments.field
			return 1793
		end
		def initialize(data = nil)
			if( data == nil )
				super(1793)
			else
				super(1793, data)
			end
		end
	end

	class NotAffectedMarketSegmentID < Quickfix::StringField
		def NotAffectedMarketSegmentID.field
			return 1794
		end
		def initialize(data = nil)
			if( data == nil )
				super(1794)
			else
				super(1794, data)
			end
		end
	end

	class NoOrderEvents < Quickfix::IntField
		def NoOrderEvents.field
			return 1795
		end
		def initialize(data = nil)
			if( data == nil )
				super(1795)
			else
				super(1795, data)
			end
		end
	end

	class OrderEventType < Quickfix::IntField
		def OrderEventType.field
			return 1796
		end
		def initialize(data = nil)
			if( data == nil )
				super(1796)
			else
				super(1796, data)
			end
		end
	end

	class OrderEventExecID < Quickfix::StringField
		def OrderEventExecID.field
			return 1797
		end
		def initialize(data = nil)
			if( data == nil )
				super(1797)
			else
				super(1797, data)
			end
		end
	end

	class OrderEventReason < Quickfix::IntField
		def OrderEventReason.field
			return 1798
		end
		def initialize(data = nil)
			if( data == nil )
				super(1798)
			else
				super(1798, data)
			end
		end
	end

	class OrderEventPx < Quickfix::DoubleField
		def OrderEventPx.field
			return 1799
		end
		def initialize(data = nil)
			if( data == nil )
				super(1799)
			else
				super(1799, data)
			end
		end
	end

	class OrderEventQty < Quickfix::DoubleField
		def OrderEventQty.field
			return 1800
		end
		def initialize(data = nil)
			if( data == nil )
				super(1800)
			else
				super(1800, data)
			end
		end
	end

	class OrderEventLiquidityIndicator < Quickfix::IntField
		def OrderEventLiquidityIndicator.field
			return 1801
		end
		def initialize(data = nil)
			if( data == nil )
				super(1801)
			else
				super(1801, data)
			end
		end
	end

	class OrderEventText < Quickfix::StringField
		def OrderEventText.field
			return 1802
		end
		def initialize(data = nil)
			if( data == nil )
				super(1802)
			else
				super(1802, data)
			end
		end
	end

	class AuctionType < Quickfix::IntField
		def AuctionType.field
			return 1803
		end
		def initialize(data = nil)
			if( data == nil )
				super(1803)
			else
				super(1803, data)
			end
		end
	end

	class AuctionAllocationPct < Quickfix::DoubleField
		def AuctionAllocationPct.field
			return 1804
		end
		def initialize(data = nil)
			if( data == nil )
				super(1804)
			else
				super(1804, data)
			end
		end
	end

	class AuctionInstruction < Quickfix::IntField
		def AuctionInstruction.field
			return 1805
		end
		def initialize(data = nil)
			if( data == nil )
				super(1805)
			else
				super(1805, data)
			end
		end
	end

	class RefClOrdID < Quickfix::StringField
		def RefClOrdID.field
			return 1806
		end
		def initialize(data = nil)
			if( data == nil )
				super(1806)
			else
				super(1806, data)
			end
		end
	end

	class LockType < Quickfix::IntField
		def LockType.field
			return 1807
		end
		def initialize(data = nil)
			if( data == nil )
				super(1807)
			else
				super(1807, data)
			end
		end
	end

	class LockedQty < Quickfix::DoubleField
		def LockedQty.field
			return 1808
		end
		def initialize(data = nil)
			if( data == nil )
				super(1808)
			else
				super(1808, data)
			end
		end
	end

	class SecondaryLockedQty < Quickfix::DoubleField
		def SecondaryLockedQty.field
			return 1809
		end
		def initialize(data = nil)
			if( data == nil )
				super(1809)
			else
				super(1809, data)
			end
		end
	end

	class ReleaseInstruction < Quickfix::IntField
		def ReleaseInstruction.field
			return 1810
		end
		def initialize(data = nil)
			if( data == nil )
				super(1810)
			else
				super(1810, data)
			end
		end
	end

	class ReleaseQty < Quickfix::DoubleField
		def ReleaseQty.field
			return 1811
		end
		def initialize(data = nil)
			if( data == nil )
				super(1811)
			else
				super(1811, data)
			end
		end
	end

	class NoDisclosureInstructions < Quickfix::IntField
		def NoDisclosureInstructions.field
			return 1812
		end
		def initialize(data = nil)
			if( data == nil )
				super(1812)
			else
				super(1812, data)
			end
		end
	end

	class DisclosureType < Quickfix::IntField
		def DisclosureType.field
			return 1813
		end
		def initialize(data = nil)
			if( data == nil )
				super(1813)
			else
				super(1813, data)
			end
		end
	end

	class DisclosureInstruction < Quickfix::IntField
		def DisclosureInstruction.field
			return 1814
		end
		def initialize(data = nil)
			if( data == nil )
				super(1814)
			else
				super(1814, data)
			end
		end
	end

	class TradingCapacity < Quickfix::IntField
		def TradingCapacity.field
			return 1815
		end
		def initialize(data = nil)
			if( data == nil )
				super(1815)
			else
				super(1815, data)
			end
		end
	end

	class ClearingAccountType < Quickfix::IntField
		def ClearingAccountType.field
			return 1816
		end
		def initialize(data = nil)
			if( data == nil )
				super(1816)
			else
				super(1816, data)
			end
		end
	end

	class LegClearingAccountType < Quickfix::IntField
		def LegClearingAccountType.field
			return 1817
		end
		def initialize(data = nil)
			if( data == nil )
				super(1817)
			else
				super(1817, data)
			end
		end
	end

	class TargetPartyRoleQualifier < Quickfix::IntField
		def TargetPartyRoleQualifier.field
			return 1818
		end
		def initialize(data = nil)
			if( data == nil )
				super(1818)
			else
				super(1818, data)
			end
		end
	end

	class RelatedHighPrice < Quickfix::DoubleField
		def RelatedHighPrice.field
			return 1819
		end
		def initialize(data = nil)
			if( data == nil )
				super(1819)
			else
				super(1819, data)
			end
		end
	end

	class RelatedLowPrice < Quickfix::DoubleField
		def RelatedLowPrice.field
			return 1820
		end
		def initialize(data = nil)
			if( data == nil )
				super(1820)
			else
				super(1820, data)
			end
		end
	end

	class RelatedPriceSource < Quickfix::IntField
		def RelatedPriceSource.field
			return 1821
		end
		def initialize(data = nil)
			if( data == nil )
				super(1821)
			else
				super(1821, data)
			end
		end
	end

	class MinQtyMethod < Quickfix::IntField
		def MinQtyMethod.field
			return 1822
		end
		def initialize(data = nil)
			if( data == nil )
				super(1822)
			else
				super(1822, data)
			end
		end
	end

	class Triggered < Quickfix::IntField
		def Triggered.field
			return 1823
		end
		def initialize(data = nil)
			if( data == nil )
				super(1823)
			else
				super(1823, data)
			end
		end
	end

	class AffectedOrigClOrdID < Quickfix::StringField
		def AffectedOrigClOrdID.field
			return 1824
		end
		def initialize(data = nil)
			if( data == nil )
				super(1824)
			else
				super(1824, data)
			end
		end
	end

	class NotAffSecondaryOrderID < Quickfix::StringField
		def NotAffSecondaryOrderID.field
			return 1825
		end
		def initialize(data = nil)
			if( data == nil )
				super(1825)
			else
				super(1825, data)
			end
		end
	end

	class NoCrossLegs < Quickfix::IntField
		def NoCrossLegs.field
			return 1829
		end
		def initialize(data = nil)
			if( data == nil )
				super(1829)
			else
				super(1829, data)
			end
		end
	end

	class EventTimePeriod < Quickfix::IntField
		def EventTimePeriod.field
			return 1826
		end
		def initialize(data = nil)
			if( data == nil )
				super(1826)
			else
				super(1826, data)
			end
		end
	end

	class EventTimeUnit < Quickfix::StringField
		def EventTimeUnit.field
			return 1827
		end
		def initialize(data = nil)
			if( data == nil )
				super(1827)
			else
				super(1827, data)
			end
		end
	end

	class LastQtyVariance < Quickfix::DoubleField
		def LastQtyVariance.field
			return 1828
		end
		def initialize(data = nil)
			if( data == nil )
				super(1828)
			else
				super(1828, data)
			end
		end
	end

	class OrderOrigination < Quickfix::IntField
		def OrderOrigination.field
			return 1724
		end
		def initialize(data = nil)
			if( data == nil )
				super(1724)
			else
				super(1724, data)
			end
		end
	end

	class OriginatingDeptID < Quickfix::StringField
		def OriginatingDeptID.field
			return 1725
		end
		def initialize(data = nil)
			if( data == nil )
				super(1725)
			else
				super(1725, data)
			end
		end
	end

	class ReceivingDeptID < Quickfix::StringField
		def ReceivingDeptID.field
			return 1726
		end
		def initialize(data = nil)
			if( data == nil )
				super(1726)
			else
				super(1726, data)
			end
		end
	end

	class InformationBarrierID < Quickfix::StringField
		def InformationBarrierID.field
			return 1727
		end
		def initialize(data = nil)
			if( data == nil )
				super(1727)
			else
				super(1727, data)
			end
		end
	end

	class SettlPriceIncrement < Quickfix::DoubleField
		def SettlPriceIncrement.field
			return 1830
		end
		def initialize(data = nil)
			if( data == nil )
				super(1830)
			else
				super(1830, data)
			end
		end
	end

	class SettlPriceSecondaryIncrement < Quickfix::DoubleField
		def SettlPriceSecondaryIncrement.field
			return 1831
		end
		def initialize(data = nil)
			if( data == nil )
				super(1831)
			else
				super(1831, data)
			end
		end
	end

	class ClearedIndicator < Quickfix::IntField
		def ClearedIndicator.field
			return 1832
		end
		def initialize(data = nil)
			if( data == nil )
				super(1832)
			else
				super(1832, data)
			end
		end
	end

	class ContractRefPosType < Quickfix::IntField
		def ContractRefPosType.field
			return 1833
		end
		def initialize(data = nil)
			if( data == nil )
				super(1833)
			else
				super(1833, data)
			end
		end
	end

	class PositionCapacity < Quickfix::IntField
		def PositionCapacity.field
			return 1834
		end
		def initialize(data = nil)
			if( data == nil )
				super(1834)
			else
				super(1834, data)
			end
		end
	end

	class PosQtyUnitOfMeasureCurrency < Quickfix::StringField
		def PosQtyUnitOfMeasureCurrency.field
			return 1835
		end
		def initialize(data = nil)
			if( data == nil )
				super(1835)
			else
				super(1835, data)
			end
		end
	end

	class PosQtyUnitOfMeasure < Quickfix::StringField
		def PosQtyUnitOfMeasure.field
			return 1836
		end
		def initialize(data = nil)
			if( data == nil )
				super(1836)
			else
				super(1836, data)
			end
		end
	end

	class UnderlyingContractPriceRefMonth < Quickfix::StringField
		def UnderlyingContractPriceRefMonth.field
			return 1837
		end
		def initialize(data = nil)
			if( data == nil )
				super(1837)
			else
				super(1837, data)
			end
		end
	end

	class NoTradePriceConditions < Quickfix::IntField
		def NoTradePriceConditions.field
			return 1838
		end
		def initialize(data = nil)
			if( data == nil )
				super(1838)
			else
				super(1838, data)
			end
		end
	end

	class TradePriceCondition < Quickfix::IntField
		def TradePriceCondition.field
			return 1839
		end
		def initialize(data = nil)
			if( data == nil )
				super(1839)
			else
				super(1839, data)
			end
		end
	end

	class TradeAllocStatus < Quickfix::IntField
		def TradeAllocStatus.field
			return 1840
		end
		def initialize(data = nil)
			if( data == nil )
				super(1840)
			else
				super(1840, data)
			end
		end
	end

	class NoTradeQtys < Quickfix::IntField
		def NoTradeQtys.field
			return 1841
		end
		def initialize(data = nil)
			if( data == nil )
				super(1841)
			else
				super(1841, data)
			end
		end
	end

	class TradeQtyType < Quickfix::IntField
		def TradeQtyType.field
			return 1842
		end
		def initialize(data = nil)
			if( data == nil )
				super(1842)
			else
				super(1842, data)
			end
		end
	end

	class TradeQty < Quickfix::DoubleField
		def TradeQty.field
			return 1843
		end
		def initialize(data = nil)
			if( data == nil )
				super(1843)
			else
				super(1843, data)
			end
		end
	end

	class NoTradeAllocAmts < Quickfix::IntField
		def NoTradeAllocAmts.field
			return 1844
		end
		def initialize(data = nil)
			if( data == nil )
				super(1844)
			else
				super(1844, data)
			end
		end
	end

	class TradeAllocAmtType < Quickfix::StringField
		def TradeAllocAmtType.field
			return 1845
		end
		def initialize(data = nil)
			if( data == nil )
				super(1845)
			else
				super(1845, data)
			end
		end
	end

	class TradeAllocAmt < Quickfix::DoubleField
		def TradeAllocAmt.field
			return 1846
		end
		def initialize(data = nil)
			if( data == nil )
				super(1846)
			else
				super(1846, data)
			end
		end
	end

	class TradeAllocCurrency < Quickfix::StringField
		def TradeAllocCurrency.field
			return 1847
		end
		def initialize(data = nil)
			if( data == nil )
				super(1847)
			else
				super(1847, data)
			end
		end
	end

	class TradeAllocGroupInstruction < Quickfix::IntField
		def TradeAllocGroupInstruction.field
			return 1848
		end
		def initialize(data = nil)
			if( data == nil )
				super(1848)
			else
				super(1848, data)
			end
		end
	end

	class OffsetInstruction < Quickfix::IntField
		def OffsetInstruction.field
			return 1849
		end
		def initialize(data = nil)
			if( data == nil )
				super(1849)
			else
				super(1849, data)
			end
		end
	end

	class TradeAllocAmtReason < Quickfix::IntField
		def TradeAllocAmtReason.field
			return 1850
		end
		def initialize(data = nil)
			if( data == nil )
				super(1850)
			else
				super(1850, data)
			end
		end
	end

	class StrategyLinkID < Quickfix::StringField
		def StrategyLinkID.field
			return 1851
		end
		def initialize(data = nil)
			if( data == nil )
				super(1851)
			else
				super(1851, data)
			end
		end
	end

	class SideAvgPx < Quickfix::DoubleField
		def SideAvgPx.field
			return 1852
		end
		def initialize(data = nil)
			if( data == nil )
				super(1852)
			else
				super(1852, data)
			end
		end
	end

	class SideAvgPxIndicator < Quickfix::IntField
		def SideAvgPxIndicator.field
			return 1853
		end
		def initialize(data = nil)
			if( data == nil )
				super(1853)
			else
				super(1853, data)
			end
		end
	end

	class SideAvgPxGroupID < Quickfix::StringField
		def SideAvgPxGroupID.field
			return 1854
		end
		def initialize(data = nil)
			if( data == nil )
				super(1854)
			else
				super(1854, data)
			end
		end
	end

	class NoRelatedTrades < Quickfix::IntField
		def NoRelatedTrades.field
			return 1855
		end
		def initialize(data = nil)
			if( data == nil )
				super(1855)
			else
				super(1855, data)
			end
		end
	end

	class RelatedTradeID < Quickfix::StringField
		def RelatedTradeID.field
			return 1856
		end
		def initialize(data = nil)
			if( data == nil )
				super(1856)
			else
				super(1856, data)
			end
		end
	end

	class RelatedTradeIDSource < Quickfix::IntField
		def RelatedTradeIDSource.field
			return 1857
		end
		def initialize(data = nil)
			if( data == nil )
				super(1857)
			else
				super(1857, data)
			end
		end
	end

	class RelatedTradeDate < Quickfix::StringField
		def RelatedTradeDate.field
			return 1858
		end
		def initialize(data = nil)
			if( data == nil )
				super(1858)
			else
				super(1858, data)
			end
		end
	end

	class RelatedTradeMarketID < Quickfix::StringField
		def RelatedTradeMarketID.field
			return 1859
		end
		def initialize(data = nil)
			if( data == nil )
				super(1859)
			else
				super(1859, data)
			end
		end
	end

	class RelatedTradeQuantity < Quickfix::DoubleField
		def RelatedTradeQuantity.field
			return 1860
		end
		def initialize(data = nil)
			if( data == nil )
				super(1860)
			else
				super(1860, data)
			end
		end
	end

	class NoRelatedPositions < Quickfix::IntField
		def NoRelatedPositions.field
			return 1861
		end
		def initialize(data = nil)
			if( data == nil )
				super(1861)
			else
				super(1861, data)
			end
		end
	end

	class RelatedPositionID < Quickfix::StringField
		def RelatedPositionID.field
			return 1862
		end
		def initialize(data = nil)
			if( data == nil )
				super(1862)
			else
				super(1862, data)
			end
		end
	end

	class RelatedPositionIDSource < Quickfix::IntField
		def RelatedPositionIDSource.field
			return 1863
		end
		def initialize(data = nil)
			if( data == nil )
				super(1863)
			else
				super(1863, data)
			end
		end
	end

	class RelatedPositionDate < Quickfix::StringField
		def RelatedPositionDate.field
			return 1864
		end
		def initialize(data = nil)
			if( data == nil )
				super(1864)
			else
				super(1864, data)
			end
		end
	end

	class OfferID < Quickfix::StringField
		def OfferID.field
			return 1867
		end
		def initialize(data = nil)
			if( data == nil )
				super(1867)
			else
				super(1867, data)
			end
		end
	end

	class NoValueChecks < Quickfix::IntField
		def NoValueChecks.field
			return 1868
		end
		def initialize(data = nil)
			if( data == nil )
				super(1868)
			else
				super(1868, data)
			end
		end
	end

	class ValueCheckType < Quickfix::IntField
		def ValueCheckType.field
			return 1869
		end
		def initialize(data = nil)
			if( data == nil )
				super(1869)
			else
				super(1869, data)
			end
		end
	end

	class ValueCheckAction < Quickfix::IntField
		def ValueCheckAction.field
			return 1870
		end
		def initialize(data = nil)
			if( data == nil )
				super(1870)
			else
				super(1870, data)
			end
		end
	end

	class LegSecurityXMLLen < Quickfix::IntField
		def LegSecurityXMLLen.field
			return 1871
		end
		def initialize(data = nil)
			if( data == nil )
				super(1871)
			else
				super(1871, data)
			end
		end
	end

	class LegSecurityXML < Quickfix::StringField
		def LegSecurityXML.field
			return 1872
		end
		def initialize(data = nil)
			if( data == nil )
				super(1872)
			else
				super(1872, data)
			end
		end
	end

	class LegSecurityXMLSchema < Quickfix::StringField
		def LegSecurityXMLSchema.field
			return 1873
		end
		def initialize(data = nil)
			if( data == nil )
				super(1873)
			else
				super(1873, data)
			end
		end
	end

	class UnderlyingSecurityXMLLen < Quickfix::IntField
		def UnderlyingSecurityXMLLen.field
			return 1874
		end
		def initialize(data = nil)
			if( data == nil )
				super(1874)
			else
				super(1874, data)
			end
		end
	end

	class UnderlyingSecurityXML < Quickfix::StringField
		def UnderlyingSecurityXML.field
			return 1875
		end
		def initialize(data = nil)
			if( data == nil )
				super(1875)
			else
				super(1875, data)
			end
		end
	end

	class UnderlyingSecurityXMLSchema < Quickfix::StringField
		def UnderlyingSecurityXMLSchema.field
			return 1876
		end
		def initialize(data = nil)
			if( data == nil )
				super(1876)
			else
				super(1876, data)
			end
		end
	end

	class PartyDetailRequestResult < Quickfix::IntField
		def PartyDetailRequestResult.field
			return 1877
		end
		def initialize(data = nil)
			if( data == nil )
				super(1877)
			else
				super(1877, data)
			end
		end
	end

	class PartyDetailRequestStatus < Quickfix::IntField
		def PartyDetailRequestStatus.field
			return 1878
		end
		def initialize(data = nil)
			if( data == nil )
				super(1878)
			else
				super(1878, data)
			end
		end
	end

	class PartyDetailDefinitionStatus < Quickfix::IntField
		def PartyDetailDefinitionStatus.field
			return 1879
		end
		def initialize(data = nil)
			if( data == nil )
				super(1879)
			else
				super(1879, data)
			end
		end
	end

	class PartyDetailDefinitionResult < Quickfix::IntField
		def PartyDetailDefinitionResult.field
			return 1880
		end
		def initialize(data = nil)
			if( data == nil )
				super(1880)
			else
				super(1880, data)
			end
		end
	end

	class EntitlementRequestResult < Quickfix::IntField
		def EntitlementRequestResult.field
			return 1881
		end
		def initialize(data = nil)
			if( data == nil )
				super(1881)
			else
				super(1881, data)
			end
		end
	end

	class EntitlementRequestStatus < Quickfix::IntField
		def EntitlementRequestStatus.field
			return 1882
		end
		def initialize(data = nil)
			if( data == nil )
				super(1882)
			else
				super(1882, data)
			end
		end
	end

	class EntitlementStatus < Quickfix::IntField
		def EntitlementStatus.field
			return 1883
		end
		def initialize(data = nil)
			if( data == nil )
				super(1883)
			else
				super(1883, data)
			end
		end
	end

	class EntitlementResult < Quickfix::IntField
		def EntitlementResult.field
			return 1884
		end
		def initialize(data = nil)
			if( data == nil )
				super(1884)
			else
				super(1884, data)
			end
		end
	end

	class EntitlementRefID < Quickfix::StringField
		def EntitlementRefID.field
			return 1885
		end
		def initialize(data = nil)
			if( data == nil )
				super(1885)
			else
				super(1885, data)
			end
		end
	end

	class SettlPriceUnitOfMeasure < Quickfix::StringField
		def SettlPriceUnitOfMeasure.field
			return 1886
		end
		def initialize(data = nil)
			if( data == nil )
				super(1886)
			else
				super(1886, data)
			end
		end
	end

	class SettlPriceUnitOfMeasureCurrency < Quickfix::StringField
		def SettlPriceUnitOfMeasureCurrency.field
			return 1887
		end
		def initialize(data = nil)
			if( data == nil )
				super(1887)
			else
				super(1887, data)
			end
		end
	end

	class TradeMatchTimestamp < Quickfix::UtcTimeStampField
		def TradeMatchTimestamp.field
			return 1888
		end
		def initialize(data = nil)
			if( data == nil )
				super(1888)
			else
				super(1888, data)
			end
		end
	end

	class NoInstrmtMatchSides < Quickfix::IntField
		def NoInstrmtMatchSides.field
			return 1889
		end
		def initialize(data = nil)
			if( data == nil )
				super(1889)
			else
				super(1889, data)
			end
		end
	end

	class NoTrdMatchSides < Quickfix::IntField
		def NoTrdMatchSides.field
			return 1890
		end
		def initialize(data = nil)
			if( data == nil )
				super(1890)
			else
				super(1890, data)
			end
		end
	end

	class TrdMatchSubID < Quickfix::StringField
		def TrdMatchSubID.field
			return 1891
		end
		def initialize(data = nil)
			if( data == nil )
				super(1891)
			else
				super(1891, data)
			end
		end
	end

	class NoLegExecs < Quickfix::IntField
		def NoLegExecs.field
			return 1892
		end
		def initialize(data = nil)
			if( data == nil )
				super(1892)
			else
				super(1892, data)
			end
		end
	end

	class LegExecID < Quickfix::StringField
		def LegExecID.field
			return 1893
		end
		def initialize(data = nil)
			if( data == nil )
				super(1893)
			else
				super(1893, data)
			end
		end
	end

	class LegTradeID < Quickfix::StringField
		def LegTradeID.field
			return 1894
		end
		def initialize(data = nil)
			if( data == nil )
				super(1894)
			else
				super(1894, data)
			end
		end
	end

	class LegTradeReportID < Quickfix::StringField
		def LegTradeReportID.field
			return 1895
		end
		def initialize(data = nil)
			if( data == nil )
				super(1895)
			else
				super(1895, data)
			end
		end
	end

	class TradeMatchAckStatus < Quickfix::IntField
		def TradeMatchAckStatus.field
			return 1896
		end
		def initialize(data = nil)
			if( data == nil )
				super(1896)
			else
				super(1896, data)
			end
		end
	end

	class TradeMatchRejectReason < Quickfix::IntField
		def TradeMatchRejectReason.field
			return 1897
		end
		def initialize(data = nil)
			if( data == nil )
				super(1897)
			else
				super(1897, data)
			end
		end
	end

	class SideMarketSegmentID < Quickfix::StringField
		def SideMarketSegmentID.field
			return 1898
		end
		def initialize(data = nil)
			if( data == nil )
				super(1898)
			else
				super(1898, data)
			end
		end
	end

	class SideVenueType < Quickfix::CharField
		def SideVenueType.field
			return 1899
		end
		def initialize(data = nil)
			if( data == nil )
				super(1899)
			else
				super(1899, data)
			end
		end
	end

	class SideExecRefID < Quickfix::StringField
		def SideExecRefID.field
			return 1900
		end
		def initialize(data = nil)
			if( data == nil )
				super(1900)
			else
				super(1900, data)
			end
		end
	end

	class LegExecRefID < Quickfix::StringField
		def LegExecRefID.field
			return 1901
		end
		def initialize(data = nil)
			if( data == nil )
				super(1901)
			else
				super(1901, data)
			end
		end
	end

	class HaircutIndicator < Quickfix::BoolField
		def HaircutIndicator.field
			return 1902
		end
		def initialize(data = nil)
			if( data == nil )
				super(1902)
			else
				super(1902, data)
			end
		end
	end

	class NumOfCompetitors < Quickfix::IntField
		def NumOfCompetitors.field
			return 1913
		end
		def initialize(data = nil)
			if( data == nil )
				super(1913)
			else
				super(1913, data)
			end
		end
	end

	class ResponseTime < Quickfix::UtcTimeStampField
		def ResponseTime.field
			return 1914
		end
		def initialize(data = nil)
			if( data == nil )
				super(1914)
			else
				super(1914, data)
			end
		end
	end

	class QuoteDisplayTime < Quickfix::UtcTimeStampField
		def QuoteDisplayTime.field
			return 1915
		end
		def initialize(data = nil)
			if( data == nil )
				super(1915)
			else
				super(1915, data)
			end
		end
	end

	class ExposureDurationUnit < Quickfix::IntField
		def ExposureDurationUnit.field
			return 1916
		end
		def initialize(data = nil)
			if( data == nil )
				super(1916)
			else
				super(1916, data)
			end
		end
	end

	class CoverPrice < Quickfix::DoubleField
		def CoverPrice.field
			return 1917
		end
		def initialize(data = nil)
			if( data == nil )
				super(1917)
			else
				super(1917, data)
			end
		end
	end

	class NoClearingAccountTypes < Quickfix::IntField
		def NoClearingAccountTypes.field
			return 1918
		end
		def initialize(data = nil)
			if( data == nil )
				super(1918)
			else
				super(1918, data)
			end
		end
	end

	class NoPriceMovements < Quickfix::IntField
		def NoPriceMovements.field
			return 1919
		end
		def initialize(data = nil)
			if( data == nil )
				super(1919)
			else
				super(1919, data)
			end
		end
	end

	class NoPriceMovementValues < Quickfix::IntField
		def NoPriceMovementValues.field
			return 1920
		end
		def initialize(data = nil)
			if( data == nil )
				super(1920)
			else
				super(1920, data)
			end
		end
	end

	class PriceMovementValue < Quickfix::DoubleField
		def PriceMovementValue.field
			return 1921
		end
		def initialize(data = nil)
			if( data == nil )
				super(1921)
			else
				super(1921, data)
			end
		end
	end

	class PriceMovementPoint < Quickfix::IntField
		def PriceMovementPoint.field
			return 1922
		end
		def initialize(data = nil)
			if( data == nil )
				super(1922)
			else
				super(1922, data)
			end
		end
	end

	class PriceMovementType < Quickfix::IntField
		def PriceMovementType.field
			return 1923
		end
		def initialize(data = nil)
			if( data == nil )
				super(1923)
			else
				super(1923, data)
			end
		end
	end

	class EncodedEventTextLen < Quickfix::IntField
		def EncodedEventTextLen.field
			return 1578
		end
		def initialize(data = nil)
			if( data == nil )
				super(1578)
			else
				super(1578, data)
			end
		end
	end

	class EncodedEventText < Quickfix::StringField
		def EncodedEventText.field
			return 1579
		end
		def initialize(data = nil)
			if( data == nil )
				super(1579)
			else
				super(1579, data)
			end
		end
	end

	class RegulatoryTradeID < Quickfix::StringField
		def RegulatoryTradeID.field
			return 1903
		end
		def initialize(data = nil)
			if( data == nil )
				super(1903)
			else
				super(1903, data)
			end
		end
	end

	class RegulatoryTradeIDEvent < Quickfix::IntField
		def RegulatoryTradeIDEvent.field
			return 1904
		end
		def initialize(data = nil)
			if( data == nil )
				super(1904)
			else
				super(1904, data)
			end
		end
	end

	class RegulatoryTradeIDSource < Quickfix::StringField
		def RegulatoryTradeIDSource.field
			return 1905
		end
		def initialize(data = nil)
			if( data == nil )
				super(1905)
			else
				super(1905, data)
			end
		end
	end

	class RegulatoryTradeIDType < Quickfix::IntField
		def RegulatoryTradeIDType.field
			return 1906
		end
		def initialize(data = nil)
			if( data == nil )
				super(1906)
			else
				super(1906, data)
			end
		end
	end

	class NoRegulatoryTradeIDs < Quickfix::IntField
		def NoRegulatoryTradeIDs.field
			return 1907
		end
		def initialize(data = nil)
			if( data == nil )
				super(1907)
			else
				super(1907, data)
			end
		end
	end

	class NoAllocRegulatoryTradeIDs < Quickfix::IntField
		def NoAllocRegulatoryTradeIDs.field
			return 1908
		end
		def initialize(data = nil)
			if( data == nil )
				super(1908)
			else
				super(1908, data)
			end
		end
	end

	class AllocRegulatoryTradeID < Quickfix::StringField
		def AllocRegulatoryTradeID.field
			return 1909
		end
		def initialize(data = nil)
			if( data == nil )
				super(1909)
			else
				super(1909, data)
			end
		end
	end

	class AllocRegulatoryTradeIDSource < Quickfix::StringField
		def AllocRegulatoryTradeIDSource.field
			return 1910
		end
		def initialize(data = nil)
			if( data == nil )
				super(1910)
			else
				super(1910, data)
			end
		end
	end

	class AllocRegulatoryTradeIDEvent < Quickfix::IntField
		def AllocRegulatoryTradeIDEvent.field
			return 1911
		end
		def initialize(data = nil)
			if( data == nil )
				super(1911)
			else
				super(1911, data)
			end
		end
	end

	class AllocRegulatoryTradeIDType < Quickfix::IntField
		def AllocRegulatoryTradeIDType.field
			return 1912
		end
		def initialize(data = nil)
			if( data == nil )
				super(1912)
			else
				super(1912, data)
			end
		end
	end

	class ClearingIntention < Quickfix::IntField
		def ClearingIntention.field
			return 1924
		end
		def initialize(data = nil)
			if( data == nil )
				super(1924)
			else
				super(1924, data)
			end
		end
	end

	class TradeClearingInstruction < Quickfix::IntField
		def TradeClearingInstruction.field
			return 1925
		end
		def initialize(data = nil)
			if( data == nil )
				super(1925)
			else
				super(1925, data)
			end
		end
	end

	class BackloadedTradeIndicator < Quickfix::BoolField
		def BackloadedTradeIndicator.field
			return 1926
		end
		def initialize(data = nil)
			if( data == nil )
				super(1926)
			else
				super(1926, data)
			end
		end
	end

	class ConfirmationMethod < Quickfix::IntField
		def ConfirmationMethod.field
			return 1927
		end
		def initialize(data = nil)
			if( data == nil )
				super(1927)
			else
				super(1927, data)
			end
		end
	end

	class MandatoryClearingIndicator < Quickfix::BoolField
		def MandatoryClearingIndicator.field
			return 1928
		end
		def initialize(data = nil)
			if( data == nil )
				super(1928)
			else
				super(1928, data)
			end
		end
	end

	class MixedSwapIndicator < Quickfix::BoolField
		def MixedSwapIndicator.field
			return 1929
		end
		def initialize(data = nil)
			if( data == nil )
				super(1929)
			else
				super(1929, data)
			end
		end
	end

	class OffMarketPriceIndicator < Quickfix::BoolField
		def OffMarketPriceIndicator.field
			return 1930
		end
		def initialize(data = nil)
			if( data == nil )
				super(1930)
			else
				super(1930, data)
			end
		end
	end

	class VerificationMethod < Quickfix::IntField
		def VerificationMethod.field
			return 1931
		end
		def initialize(data = nil)
			if( data == nil )
				super(1931)
			else
				super(1931, data)
			end
		end
	end

	class ClearingRequirementException < Quickfix::IntField
		def ClearingRequirementException.field
			return 1932
		end
		def initialize(data = nil)
			if( data == nil )
				super(1932)
			else
				super(1932, data)
			end
		end
	end

	class IRSDirection < Quickfix::StringField
		def IRSDirection.field
			return 1933
		end
		def initialize(data = nil)
			if( data == nil )
				super(1933)
			else
				super(1933, data)
			end
		end
	end

	class RegulatoryReportType < Quickfix::IntField
		def RegulatoryReportType.field
			return 1934
		end
		def initialize(data = nil)
			if( data == nil )
				super(1934)
			else
				super(1934, data)
			end
		end
	end

	class VoluntaryRegulatoryReport < Quickfix::BoolField
		def VoluntaryRegulatoryReport.field
			return 1935
		end
		def initialize(data = nil)
			if( data == nil )
				super(1935)
			else
				super(1935, data)
			end
		end
	end

	class TradeCollateralization < Quickfix::IntField
		def TradeCollateralization.field
			return 1936
		end
		def initialize(data = nil)
			if( data == nil )
				super(1936)
			else
				super(1936, data)
			end
		end
	end

	class TradeContinuation < Quickfix::IntField
		def TradeContinuation.field
			return 1937
		end
		def initialize(data = nil)
			if( data == nil )
				super(1937)
			else
				super(1937, data)
			end
		end
	end

	class AssetClass < Quickfix::IntField
		def AssetClass.field
			return 1938
		end
		def initialize(data = nil)
			if( data == nil )
				super(1938)
			else
				super(1938, data)
			end
		end
	end

	class AssetSubClass < Quickfix::IntField
		def AssetSubClass.field
			return 1939
		end
		def initialize(data = nil)
			if( data == nil )
				super(1939)
			else
				super(1939, data)
			end
		end
	end

	class AssetType < Quickfix::StringField
		def AssetType.field
			return 1940
		end
		def initialize(data = nil)
			if( data == nil )
				super(1940)
			else
				super(1940, data)
			end
		end
	end

	class SwapClass < Quickfix::StringField
		def SwapClass.field
			return 1941
		end
		def initialize(data = nil)
			if( data == nil )
				super(1941)
			else
				super(1941, data)
			end
		end
	end

	class NthToDefault < Quickfix::IntField
		def NthToDefault.field
			return 1942
		end
		def initialize(data = nil)
			if( data == nil )
				super(1942)
			else
				super(1942, data)
			end
		end
	end

	class MthToDefault < Quickfix::IntField
		def MthToDefault.field
			return 1943
		end
		def initialize(data = nil)
			if( data == nil )
				super(1943)
			else
				super(1943, data)
			end
		end
	end

	class SettledEntityMatrixSource < Quickfix::StringField
		def SettledEntityMatrixSource.field
			return 1944
		end
		def initialize(data = nil)
			if( data == nil )
				super(1944)
			else
				super(1944, data)
			end
		end
	end

	class SettledEntityMatrixPublicationDate < Quickfix::StringField
		def SettledEntityMatrixPublicationDate.field
			return 1945
		end
		def initialize(data = nil)
			if( data == nil )
				super(1945)
			else
				super(1945, data)
			end
		end
	end

	class CouponType < Quickfix::IntField
		def CouponType.field
			return 1946
		end
		def initialize(data = nil)
			if( data == nil )
				super(1946)
			else
				super(1946, data)
			end
		end
	end

	class TotalIssuedAmount < Quickfix::DoubleField
		def TotalIssuedAmount.field
			return 1947
		end
		def initialize(data = nil)
			if( data == nil )
				super(1947)
			else
				super(1947, data)
			end
		end
	end

	class CouponFrequencyPeriod < Quickfix::IntField
		def CouponFrequencyPeriod.field
			return 1948
		end
		def initialize(data = nil)
			if( data == nil )
				super(1948)
			else
				super(1948, data)
			end
		end
	end

	class CouponFrequencyUnit < Quickfix::StringField
		def CouponFrequencyUnit.field
			return 1949
		end
		def initialize(data = nil)
			if( data == nil )
				super(1949)
			else
				super(1949, data)
			end
		end
	end

	class CouponDayCount < Quickfix::IntField
		def CouponDayCount.field
			return 1950
		end
		def initialize(data = nil)
			if( data == nil )
				super(1950)
			else
				super(1950, data)
			end
		end
	end

	class ConvertibleBondEquityID < Quickfix::StringField
		def ConvertibleBondEquityID.field
			return 1951
		end
		def initialize(data = nil)
			if( data == nil )
				super(1951)
			else
				super(1951, data)
			end
		end
	end

	class ConvertibleBondEquityIDSource < Quickfix::StringField
		def ConvertibleBondEquityIDSource.field
			return 1952
		end
		def initialize(data = nil)
			if( data == nil )
				super(1952)
			else
				super(1952, data)
			end
		end
	end

	class ContractPriceRefMonth < Quickfix::StringField
		def ContractPriceRefMonth.field
			return 1953
		end
		def initialize(data = nil)
			if( data == nil )
				super(1953)
			else
				super(1953, data)
			end
		end
	end

	class LienSeniority < Quickfix::IntField
		def LienSeniority.field
			return 1954
		end
		def initialize(data = nil)
			if( data == nil )
				super(1954)
			else
				super(1954, data)
			end
		end
	end

	class LoanFacility < Quickfix::IntField
		def LoanFacility.field
			return 1955
		end
		def initialize(data = nil)
			if( data == nil )
				super(1955)
			else
				super(1955, data)
			end
		end
	end

	class ReferenceEntityType < Quickfix::IntField
		def ReferenceEntityType.field
			return 1956
		end
		def initialize(data = nil)
			if( data == nil )
				super(1956)
			else
				super(1956, data)
			end
		end
	end

	class IndexSeries < Quickfix::IntField
		def IndexSeries.field
			return 1957
		end
		def initialize(data = nil)
			if( data == nil )
				super(1957)
			else
				super(1957, data)
			end
		end
	end

	class IndexAnnexVersion < Quickfix::IntField
		def IndexAnnexVersion.field
			return 1958
		end
		def initialize(data = nil)
			if( data == nil )
				super(1958)
			else
				super(1958, data)
			end
		end
	end

	class IndexAnnexDate < Quickfix::StringField
		def IndexAnnexDate.field
			return 1959
		end
		def initialize(data = nil)
			if( data == nil )
				super(1959)
			else
				super(1959, data)
			end
		end
	end

	class IndexAnnexSource < Quickfix::StringField
		def IndexAnnexSource.field
			return 1960
		end
		def initialize(data = nil)
			if( data == nil )
				super(1960)
			else
				super(1960, data)
			end
		end
	end

	class AgreementVersion < Quickfix::StringField
		def AgreementVersion.field
			return 1961
		end
		def initialize(data = nil)
			if( data == nil )
				super(1961)
			else
				super(1961, data)
			end
		end
	end

	class MasterConfirmationDesc < Quickfix::StringField
		def MasterConfirmationDesc.field
			return 1962
		end
		def initialize(data = nil)
			if( data == nil )
				super(1962)
			else
				super(1962, data)
			end
		end
	end

	class MasterConfirmationDate < Quickfix::StringField
		def MasterConfirmationDate.field
			return 1963
		end
		def initialize(data = nil)
			if( data == nil )
				super(1963)
			else
				super(1963, data)
			end
		end
	end

	class MasterConfirmationAnnexDesc < Quickfix::StringField
		def MasterConfirmationAnnexDesc.field
			return 1964
		end
		def initialize(data = nil)
			if( data == nil )
				super(1964)
			else
				super(1964, data)
			end
		end
	end

	class MasterConfirmationAnnexDate < Quickfix::StringField
		def MasterConfirmationAnnexDate.field
			return 1965
		end
		def initialize(data = nil)
			if( data == nil )
				super(1965)
			else
				super(1965, data)
			end
		end
	end

	class BrokerConfirmationDesc < Quickfix::StringField
		def BrokerConfirmationDesc.field
			return 1966
		end
		def initialize(data = nil)
			if( data == nil )
				super(1966)
			else
				super(1966, data)
			end
		end
	end

	class CreditSupportAgreementDesc < Quickfix::StringField
		def CreditSupportAgreementDesc.field
			return 1967
		end
		def initialize(data = nil)
			if( data == nil )
				super(1967)
			else
				super(1967, data)
			end
		end
	end

	class CreditSupportAgreementDate < Quickfix::StringField
		def CreditSupportAgreementDate.field
			return 1968
		end
		def initialize(data = nil)
			if( data == nil )
				super(1968)
			else
				super(1968, data)
			end
		end
	end

	class CreditSupportAgreementID < Quickfix::StringField
		def CreditSupportAgreementID.field
			return 1969
		end
		def initialize(data = nil)
			if( data == nil )
				super(1969)
			else
				super(1969, data)
			end
		end
	end

	class GoverningLaw < Quickfix::StringField
		def GoverningLaw.field
			return 1970
		end
		def initialize(data = nil)
			if( data == nil )
				super(1970)
			else
				super(1970, data)
			end
		end
	end

	class NoSideRegulatoryTradeIDs < Quickfix::IntField
		def NoSideRegulatoryTradeIDs.field
			return 1971
		end
		def initialize(data = nil)
			if( data == nil )
				super(1971)
			else
				super(1971, data)
			end
		end
	end

	class SideRegulatoryTradeID < Quickfix::StringField
		def SideRegulatoryTradeID.field
			return 1972
		end
		def initialize(data = nil)
			if( data == nil )
				super(1972)
			else
				super(1972, data)
			end
		end
	end

	class SideRegulatoryTradeIDSource < Quickfix::StringField
		def SideRegulatoryTradeIDSource.field
			return 1973
		end
		def initialize(data = nil)
			if( data == nil )
				super(1973)
			else
				super(1973, data)
			end
		end
	end

	class SideRegulatoryTradeIDEvent < Quickfix::IntField
		def SideRegulatoryTradeIDEvent.field
			return 1974
		end
		def initialize(data = nil)
			if( data == nil )
				super(1974)
			else
				super(1974, data)
			end
		end
	end

	class SideRegulatoryTradeIDType < Quickfix::IntField
		def SideRegulatoryTradeIDType.field
			return 1975
		end
		def initialize(data = nil)
			if( data == nil )
				super(1975)
			else
				super(1975, data)
			end
		end
	end

	class NoSecondaryAssetClasses < Quickfix::IntField
		def NoSecondaryAssetClasses.field
			return 1976
		end
		def initialize(data = nil)
			if( data == nil )
				super(1976)
			else
				super(1976, data)
			end
		end
	end

	class SecondaryAssetClass < Quickfix::IntField
		def SecondaryAssetClass.field
			return 1977
		end
		def initialize(data = nil)
			if( data == nil )
				super(1977)
			else
				super(1977, data)
			end
		end
	end

	class SecondaryAssetSubClass < Quickfix::IntField
		def SecondaryAssetSubClass.field
			return 1978
		end
		def initialize(data = nil)
			if( data == nil )
				super(1978)
			else
				super(1978, data)
			end
		end
	end

	class SecondaryAssetType < Quickfix::StringField
		def SecondaryAssetType.field
			return 1979
		end
		def initialize(data = nil)
			if( data == nil )
				super(1979)
			else
				super(1979, data)
			end
		end
	end

	class BlockTrdAllocIndicator < Quickfix::IntField
		def BlockTrdAllocIndicator.field
			return 1980
		end
		def initialize(data = nil)
			if( data == nil )
				super(1980)
			else
				super(1980, data)
			end
		end
	end

	class NoUnderlyingEvents < Quickfix::IntField
		def NoUnderlyingEvents.field
			return 1981
		end
		def initialize(data = nil)
			if( data == nil )
				super(1981)
			else
				super(1981, data)
			end
		end
	end

	class UnderlyingEventType < Quickfix::IntField
		def UnderlyingEventType.field
			return 1982
		end
		def initialize(data = nil)
			if( data == nil )
				super(1982)
			else
				super(1982, data)
			end
		end
	end

	class UnderlyingEventDate < Quickfix::StringField
		def UnderlyingEventDate.field
			return 1983
		end
		def initialize(data = nil)
			if( data == nil )
				super(1983)
			else
				super(1983, data)
			end
		end
	end

	class UnderlyingEventTime < Quickfix::UtcTimeStampField
		def UnderlyingEventTime.field
			return 1984
		end
		def initialize(data = nil)
			if( data == nil )
				super(1984)
			else
				super(1984, data)
			end
		end
	end

	class UnderlyingEventTimeUnit < Quickfix::StringField
		def UnderlyingEventTimeUnit.field
			return 1985
		end
		def initialize(data = nil)
			if( data == nil )
				super(1985)
			else
				super(1985, data)
			end
		end
	end

	class UnderlyingEventTimePeriod < Quickfix::IntField
		def UnderlyingEventTimePeriod.field
			return 1986
		end
		def initialize(data = nil)
			if( data == nil )
				super(1986)
			else
				super(1986, data)
			end
		end
	end

	class UnderlyingEventPx < Quickfix::DoubleField
		def UnderlyingEventPx.field
			return 1987
		end
		def initialize(data = nil)
			if( data == nil )
				super(1987)
			else
				super(1987, data)
			end
		end
	end

	class UnderlyingConstituentWeight < Quickfix::DoubleField
		def UnderlyingConstituentWeight.field
			return 1988
		end
		def initialize(data = nil)
			if( data == nil )
				super(1988)
			else
				super(1988, data)
			end
		end
	end

	class UnderlyingCouponType < Quickfix::IntField
		def UnderlyingCouponType.field
			return 1989
		end
		def initialize(data = nil)
			if( data == nil )
				super(1989)
			else
				super(1989, data)
			end
		end
	end

	class UnderlyingTotalIssuedAmount < Quickfix::DoubleField
		def UnderlyingTotalIssuedAmount.field
			return 1990
		end
		def initialize(data = nil)
			if( data == nil )
				super(1990)
			else
				super(1990, data)
			end
		end
	end

	class UnderlyingCouponFrequencyPeriod < Quickfix::IntField
		def UnderlyingCouponFrequencyPeriod.field
			return 1991
		end
		def initialize(data = nil)
			if( data == nil )
				super(1991)
			else
				super(1991, data)
			end
		end
	end

	class UnderlyingCouponFrequencyUnit < Quickfix::StringField
		def UnderlyingCouponFrequencyUnit.field
			return 1992
		end
		def initialize(data = nil)
			if( data == nil )
				super(1992)
			else
				super(1992, data)
			end
		end
	end

	class UnderlyingCouponDayCount < Quickfix::IntField
		def UnderlyingCouponDayCount.field
			return 1993
		end
		def initialize(data = nil)
			if( data == nil )
				super(1993)
			else
				super(1993, data)
			end
		end
	end

	class UnderlyingObligationID < Quickfix::StringField
		def UnderlyingObligationID.field
			return 1994
		end
		def initialize(data = nil)
			if( data == nil )
				super(1994)
			else
				super(1994, data)
			end
		end
	end

	class UnderlyingObligationIDSource < Quickfix::StringField
		def UnderlyingObligationIDSource.field
			return 1995
		end
		def initialize(data = nil)
			if( data == nil )
				super(1995)
			else
				super(1995, data)
			end
		end
	end

	class UnderlyingEquityID < Quickfix::StringField
		def UnderlyingEquityID.field
			return 1996
		end
		def initialize(data = nil)
			if( data == nil )
				super(1996)
			else
				super(1996, data)
			end
		end
	end

	class UnderlyingEquityIDSource < Quickfix::StringField
		def UnderlyingEquityIDSource.field
			return 1997
		end
		def initialize(data = nil)
			if( data == nil )
				super(1997)
			else
				super(1997, data)
			end
		end
	end

	class UnderlyingLienSeniority < Quickfix::IntField
		def UnderlyingLienSeniority.field
			return 1998
		end
		def initialize(data = nil)
			if( data == nil )
				super(1998)
			else
				super(1998, data)
			end
		end
	end

	class UnderlyingLoanFacility < Quickfix::IntField
		def UnderlyingLoanFacility.field
			return 1999
		end
		def initialize(data = nil)
			if( data == nil )
				super(1999)
			else
				super(1999, data)
			end
		end
	end

	class UnderlyingReferenceEntityType < Quickfix::IntField
		def UnderlyingReferenceEntityType.field
			return 2000
		end
		def initialize(data = nil)
			if( data == nil )
				super(2000)
			else
				super(2000, data)
			end
		end
	end

	class UnderlyingProtectionTermXIDRef < Quickfix::StringField
		def UnderlyingProtectionTermXIDRef.field
			return 41314
		end
		def initialize(data = nil)
			if( data == nil )
				super(41314)
			else
				super(41314, data)
			end
		end
	end

	class UnderlyingSettlTermXIDRef < Quickfix::StringField
		def UnderlyingSettlTermXIDRef.field
			return 41315
		end
		def initialize(data = nil)
			if( data == nil )
				super(41315)
			else
				super(41315, data)
			end
		end
	end

	class UnderlyingIndexSeries < Quickfix::IntField
		def UnderlyingIndexSeries.field
			return 2003
		end
		def initialize(data = nil)
			if( data == nil )
				super(2003)
			else
				super(2003, data)
			end
		end
	end

	class UnderlyingIndexAnnexVersion < Quickfix::IntField
		def UnderlyingIndexAnnexVersion.field
			return 2004
		end
		def initialize(data = nil)
			if( data == nil )
				super(2004)
			else
				super(2004, data)
			end
		end
	end

	class UnderlyingIndexAnnexDate < Quickfix::StringField
		def UnderlyingIndexAnnexDate.field
			return 2005
		end
		def initialize(data = nil)
			if( data == nil )
				super(2005)
			else
				super(2005, data)
			end
		end
	end

	class UnderlyingIndexAnnexSource < Quickfix::StringField
		def UnderlyingIndexAnnexSource.field
			return 2006
		end
		def initialize(data = nil)
			if( data == nil )
				super(2006)
			else
				super(2006, data)
			end
		end
	end

	class UnderlyingProductComplex < Quickfix::StringField
		def UnderlyingProductComplex.field
			return 2007
		end
		def initialize(data = nil)
			if( data == nil )
				super(2007)
			else
				super(2007, data)
			end
		end
	end

	class UnderlyingSecurityGroup < Quickfix::StringField
		def UnderlyingSecurityGroup.field
			return 2008
		end
		def initialize(data = nil)
			if( data == nil )
				super(2008)
			else
				super(2008, data)
			end
		end
	end

	class UnderlyingSettleOnOpenFlag < Quickfix::StringField
		def UnderlyingSettleOnOpenFlag.field
			return 2009
		end
		def initialize(data = nil)
			if( data == nil )
				super(2009)
			else
				super(2009, data)
			end
		end
	end

	class UnderlyingAssignmentMethod < Quickfix::CharField
		def UnderlyingAssignmentMethod.field
			return 2010
		end
		def initialize(data = nil)
			if( data == nil )
				super(2010)
			else
				super(2010, data)
			end
		end
	end

	class UnderlyingSecurityStatus < Quickfix::StringField
		def UnderlyingSecurityStatus.field
			return 2011
		end
		def initialize(data = nil)
			if( data == nil )
				super(2011)
			else
				super(2011, data)
			end
		end
	end

	class UnderlyingObligationType < Quickfix::StringField
		def UnderlyingObligationType.field
			return 2012
		end
		def initialize(data = nil)
			if( data == nil )
				super(2012)
			else
				super(2012, data)
			end
		end
	end

	class UnderlyingAssetClass < Quickfix::IntField
		def UnderlyingAssetClass.field
			return 2013
		end
		def initialize(data = nil)
			if( data == nil )
				super(2013)
			else
				super(2013, data)
			end
		end
	end

	class UnderlyingAssetSubClass < Quickfix::IntField
		def UnderlyingAssetSubClass.field
			return 2014
		end
		def initialize(data = nil)
			if( data == nil )
				super(2014)
			else
				super(2014, data)
			end
		end
	end

	class UnderlyingAssetType < Quickfix::StringField
		def UnderlyingAssetType.field
			return 2015
		end
		def initialize(data = nil)
			if( data == nil )
				super(2015)
			else
				super(2015, data)
			end
		end
	end

	class UnderlyingSwapClass < Quickfix::StringField
		def UnderlyingSwapClass.field
			return 2016
		end
		def initialize(data = nil)
			if( data == nil )
				super(2016)
			else
				super(2016, data)
			end
		end
	end

	class UnderlyingNthToDefault < Quickfix::IntField
		def UnderlyingNthToDefault.field
			return 2017
		end
		def initialize(data = nil)
			if( data == nil )
				super(2017)
			else
				super(2017, data)
			end
		end
	end

	class UnderlyingMthToDefault < Quickfix::IntField
		def UnderlyingMthToDefault.field
			return 2018
		end
		def initialize(data = nil)
			if( data == nil )
				super(2018)
			else
				super(2018, data)
			end
		end
	end

	class UnderlyingSettledEntityMatrixSource < Quickfix::StringField
		def UnderlyingSettledEntityMatrixSource.field
			return 2019
		end
		def initialize(data = nil)
			if( data == nil )
				super(2019)
			else
				super(2019, data)
			end
		end
	end

	class UnderlyingSettledEntityMatrixPublicationDate < Quickfix::StringField
		def UnderlyingSettledEntityMatrixPublicationDate.field
			return 2020
		end
		def initialize(data = nil)
			if( data == nil )
				super(2020)
			else
				super(2020, data)
			end
		end
	end

	class UnderlyingStrikeMultiplier < Quickfix::DoubleField
		def UnderlyingStrikeMultiplier.field
			return 2021
		end
		def initialize(data = nil)
			if( data == nil )
				super(2021)
			else
				super(2021, data)
			end
		end
	end

	class UnderlyingStrikeValue < Quickfix::DoubleField
		def UnderlyingStrikeValue.field
			return 2022
		end
		def initialize(data = nil)
			if( data == nil )
				super(2022)
			else
				super(2022, data)
			end
		end
	end

	class UnderlyingStrikePriceDeterminationMethod < Quickfix::IntField
		def UnderlyingStrikePriceDeterminationMethod.field
			return 2023
		end
		def initialize(data = nil)
			if( data == nil )
				super(2023)
			else
				super(2023, data)
			end
		end
	end

	class UnderlyingStrikePriceBoundaryMethod < Quickfix::IntField
		def UnderlyingStrikePriceBoundaryMethod.field
			return 2024
		end
		def initialize(data = nil)
			if( data == nil )
				super(2024)
			else
				super(2024, data)
			end
		end
	end

	class UnderlyingStrikePriceBoundaryPrecision < Quickfix::DoubleField
		def UnderlyingStrikePriceBoundaryPrecision.field
			return 2025
		end
		def initialize(data = nil)
			if( data == nil )
				super(2025)
			else
				super(2025, data)
			end
		end
	end

	class UnderlyingMinPriceIncrement < Quickfix::DoubleField
		def UnderlyingMinPriceIncrement.field
			return 2026
		end
		def initialize(data = nil)
			if( data == nil )
				super(2026)
			else
				super(2026, data)
			end
		end
	end

	class UnderlyingMinPriceIncrementAmount < Quickfix::DoubleField
		def UnderlyingMinPriceIncrementAmount.field
			return 2027
		end
		def initialize(data = nil)
			if( data == nil )
				super(2027)
			else
				super(2027, data)
			end
		end
	end

	class UnderlyingOptPayoutType < Quickfix::IntField
		def UnderlyingOptPayoutType.field
			return 2028
		end
		def initialize(data = nil)
			if( data == nil )
				super(2028)
			else
				super(2028, data)
			end
		end
	end

	class UnderlyingOptPayoutAmount < Quickfix::DoubleField
		def UnderlyingOptPayoutAmount.field
			return 2029
		end
		def initialize(data = nil)
			if( data == nil )
				super(2029)
			else
				super(2029, data)
			end
		end
	end

	class UnderlyingPriceQuoteMethod < Quickfix::StringField
		def UnderlyingPriceQuoteMethod.field
			return 2030
		end
		def initialize(data = nil)
			if( data == nil )
				super(2030)
			else
				super(2030, data)
			end
		end
	end

	class UnderlyingValuationMethod < Quickfix::StringField
		def UnderlyingValuationMethod.field
			return 2031
		end
		def initialize(data = nil)
			if( data == nil )
				super(2031)
			else
				super(2031, data)
			end
		end
	end

	class UnderlyingListMethod < Quickfix::IntField
		def UnderlyingListMethod.field
			return 2032
		end
		def initialize(data = nil)
			if( data == nil )
				super(2032)
			else
				super(2032, data)
			end
		end
	end

	class UnderlyingCapPrice < Quickfix::DoubleField
		def UnderlyingCapPrice.field
			return 2033
		end
		def initialize(data = nil)
			if( data == nil )
				super(2033)
			else
				super(2033, data)
			end
		end
	end

	class UnderlyingFloorPrice < Quickfix::DoubleField
		def UnderlyingFloorPrice.field
			return 2034
		end
		def initialize(data = nil)
			if( data == nil )
				super(2034)
			else
				super(2034, data)
			end
		end
	end

	class UnderlyingFlexibleIndicator < Quickfix::BoolField
		def UnderlyingFlexibleIndicator.field
			return 2035
		end
		def initialize(data = nil)
			if( data == nil )
				super(2035)
			else
				super(2035, data)
			end
		end
	end

	class UnderlyingFlexProductEligibilityIndicator < Quickfix::BoolField
		def UnderlyingFlexProductEligibilityIndicator.field
			return 2036
		end
		def initialize(data = nil)
			if( data == nil )
				super(2036)
			else
				super(2036, data)
			end
		end
	end

	class UnderlyingPositionLimit < Quickfix::IntField
		def UnderlyingPositionLimit.field
			return 2037
		end
		def initialize(data = nil)
			if( data == nil )
				super(2037)
			else
				super(2037, data)
			end
		end
	end

	class UnderlyingNTPositionLimit < Quickfix::IntField
		def UnderlyingNTPositionLimit.field
			return 2038
		end
		def initialize(data = nil)
			if( data == nil )
				super(2038)
			else
				super(2038, data)
			end
		end
	end

	class UnderlyingPool < Quickfix::StringField
		def UnderlyingPool.field
			return 2039
		end
		def initialize(data = nil)
			if( data == nil )
				super(2039)
			else
				super(2039, data)
			end
		end
	end

	class UnderlyingContractSettlMonth < Quickfix::StringField
		def UnderlyingContractSettlMonth.field
			return 2040
		end
		def initialize(data = nil)
			if( data == nil )
				super(2040)
			else
				super(2040, data)
			end
		end
	end

	class UnderlyingDatedDate < Quickfix::StringField
		def UnderlyingDatedDate.field
			return 2041
		end
		def initialize(data = nil)
			if( data == nil )
				super(2041)
			else
				super(2041, data)
			end
		end
	end

	class UnderlyingInterestAccrualDate < Quickfix::StringField
		def UnderlyingInterestAccrualDate.field
			return 2042
		end
		def initialize(data = nil)
			if( data == nil )
				super(2042)
			else
				super(2042, data)
			end
		end
	end

	class UnderlyingShortSaleRestriction < Quickfix::IntField
		def UnderlyingShortSaleRestriction.field
			return 2043
		end
		def initialize(data = nil)
			if( data == nil )
				super(2043)
			else
				super(2043, data)
			end
		end
	end

	class UnderlyingRefTickTableID < Quickfix::IntField
		def UnderlyingRefTickTableID.field
			return 2044
		end
		def initialize(data = nil)
			if( data == nil )
				super(2044)
			else
				super(2044, data)
			end
		end
	end

	class NoUnderlyingComplexEvents < Quickfix::IntField
		def NoUnderlyingComplexEvents.field
			return 2045
		end
		def initialize(data = nil)
			if( data == nil )
				super(2045)
			else
				super(2045, data)
			end
		end
	end

	class UnderlyingComplexEventType < Quickfix::IntField
		def UnderlyingComplexEventType.field
			return 2046
		end
		def initialize(data = nil)
			if( data == nil )
				super(2046)
			else
				super(2046, data)
			end
		end
	end

	class UnderlyingComplexOptPayoutAmount < Quickfix::DoubleField
		def UnderlyingComplexOptPayoutAmount.field
			return 2047
		end
		def initialize(data = nil)
			if( data == nil )
				super(2047)
			else
				super(2047, data)
			end
		end
	end

	class UnderlyingComplexEventPrice < Quickfix::DoubleField
		def UnderlyingComplexEventPrice.field
			return 2048
		end
		def initialize(data = nil)
			if( data == nil )
				super(2048)
			else
				super(2048, data)
			end
		end
	end

	class UnderlyingComplexEventPriceBoundaryMethod < Quickfix::IntField
		def UnderlyingComplexEventPriceBoundaryMethod.field
			return 2049
		end
		def initialize(data = nil)
			if( data == nil )
				super(2049)
			else
				super(2049, data)
			end
		end
	end

	class UnderlyingComplexEventPriceBoundaryPrecision < Quickfix::DoubleField
		def UnderlyingComplexEventPriceBoundaryPrecision.field
			return 2050
		end
		def initialize(data = nil)
			if( data == nil )
				super(2050)
			else
				super(2050, data)
			end
		end
	end

	class UnderlyingComplexEventPriceTimeType < Quickfix::IntField
		def UnderlyingComplexEventPriceTimeType.field
			return 2051
		end
		def initialize(data = nil)
			if( data == nil )
				super(2051)
			else
				super(2051, data)
			end
		end
	end

	class UnderlyingComplexEventCondition < Quickfix::IntField
		def UnderlyingComplexEventCondition.field
			return 2052
		end
		def initialize(data = nil)
			if( data == nil )
				super(2052)
			else
				super(2052, data)
			end
		end
	end

	class NoUnderlyingComplexEventDates < Quickfix::IntField
		def NoUnderlyingComplexEventDates.field
			return 2053
		end
		def initialize(data = nil)
			if( data == nil )
				super(2053)
			else
				super(2053, data)
			end
		end
	end

	class UnderlyingComplexEventStartDate < Quickfix::UtcDateField
		def UnderlyingComplexEventStartDate.field
			return 2054
		end
		def initialize(data = nil)
			if( data == nil )
				super(2054)
			else
				super(2054, data)
			end
		end
	end

	class UnderlyingComplexEventEndDate < Quickfix::UtcDateField
		def UnderlyingComplexEventEndDate.field
			return 2055
		end
		def initialize(data = nil)
			if( data == nil )
				super(2055)
			else
				super(2055, data)
			end
		end
	end

	class NoUnderlyingComplexEventTimes < Quickfix::IntField
		def NoUnderlyingComplexEventTimes.field
			return 2056
		end
		def initialize(data = nil)
			if( data == nil )
				super(2056)
			else
				super(2056, data)
			end
		end
	end

	class UnderlyingComplexEventStartTime < Quickfix::UtcTimeOnlyField
		def UnderlyingComplexEventStartTime.field
			return 2057
		end
		def initialize(data = nil)
			if( data == nil )
				super(2057)
			else
				super(2057, data)
			end
		end
	end

	class UnderlyingComplexEventEndTime < Quickfix::UtcTimeOnlyField
		def UnderlyingComplexEventEndTime.field
			return 2058
		end
		def initialize(data = nil)
			if( data == nil )
				super(2058)
			else
				super(2058, data)
			end
		end
	end

	class NoLegEvents < Quickfix::IntField
		def NoLegEvents.field
			return 2059
		end
		def initialize(data = nil)
			if( data == nil )
				super(2059)
			else
				super(2059, data)
			end
		end
	end

	class LegEventType < Quickfix::IntField
		def LegEventType.field
			return 2060
		end
		def initialize(data = nil)
			if( data == nil )
				super(2060)
			else
				super(2060, data)
			end
		end
	end

	class LegEventDate < Quickfix::StringField
		def LegEventDate.field
			return 2061
		end
		def initialize(data = nil)
			if( data == nil )
				super(2061)
			else
				super(2061, data)
			end
		end
	end

	class LegEventTime < Quickfix::UtcTimeStampField
		def LegEventTime.field
			return 2062
		end
		def initialize(data = nil)
			if( data == nil )
				super(2062)
			else
				super(2062, data)
			end
		end
	end

	class LegEventTimeUnit < Quickfix::StringField
		def LegEventTimeUnit.field
			return 2063
		end
		def initialize(data = nil)
			if( data == nil )
				super(2063)
			else
				super(2063, data)
			end
		end
	end

	class LegEventTimePeriod < Quickfix::IntField
		def LegEventTimePeriod.field
			return 2064
		end
		def initialize(data = nil)
			if( data == nil )
				super(2064)
			else
				super(2064, data)
			end
		end
	end

	class LegEventPx < Quickfix::DoubleField
		def LegEventPx.field
			return 2065
		end
		def initialize(data = nil)
			if( data == nil )
				super(2065)
			else
				super(2065, data)
			end
		end
	end

	class LegEventText < Quickfix::StringField
		def LegEventText.field
			return 2066
		end
		def initialize(data = nil)
			if( data == nil )
				super(2066)
			else
				super(2066, data)
			end
		end
	end

	class LegAssetClass < Quickfix::IntField
		def LegAssetClass.field
			return 2067
		end
		def initialize(data = nil)
			if( data == nil )
				super(2067)
			else
				super(2067, data)
			end
		end
	end

	class LegAssetSubClass < Quickfix::IntField
		def LegAssetSubClass.field
			return 2068
		end
		def initialize(data = nil)
			if( data == nil )
				super(2068)
			else
				super(2068, data)
			end
		end
	end

	class LegAssetType < Quickfix::StringField
		def LegAssetType.field
			return 2069
		end
		def initialize(data = nil)
			if( data == nil )
				super(2069)
			else
				super(2069, data)
			end
		end
	end

	class LegSwapClass < Quickfix::StringField
		def LegSwapClass.field
			return 2070
		end
		def initialize(data = nil)
			if( data == nil )
				super(2070)
			else
				super(2070, data)
			end
		end
	end

	class UnderlyingEventText < Quickfix::StringField
		def UnderlyingEventText.field
			return 2071
		end
		def initialize(data = nil)
			if( data == nil )
				super(2071)
			else
				super(2071, data)
			end
		end
	end

	class EncodedUnderlyingEventTextLen < Quickfix::IntField
		def EncodedUnderlyingEventTextLen.field
			return 2072
		end
		def initialize(data = nil)
			if( data == nil )
				super(2072)
			else
				super(2072, data)
			end
		end
	end

	class EncodedUnderlyingEventText < Quickfix::StringField
		def EncodedUnderlyingEventText.field
			return 2073
		end
		def initialize(data = nil)
			if( data == nil )
				super(2073)
			else
				super(2073, data)
			end
		end
	end

	class EncodedLegEventTextLen < Quickfix::IntField
		def EncodedLegEventTextLen.field
			return 2074
		end
		def initialize(data = nil)
			if( data == nil )
				super(2074)
			else
				super(2074, data)
			end
		end
	end

	class EncodedLegEventText < Quickfix::StringField
		def EncodedLegEventText.field
			return 2075
		end
		def initialize(data = nil)
			if( data == nil )
				super(2075)
			else
				super(2075, data)
			end
		end
	end

	class NoLegSecondaryAssetClasses < Quickfix::IntField
		def NoLegSecondaryAssetClasses.field
			return 2076
		end
		def initialize(data = nil)
			if( data == nil )
				super(2076)
			else
				super(2076, data)
			end
		end
	end

	class LegSecondaryAssetClass < Quickfix::IntField
		def LegSecondaryAssetClass.field
			return 2077
		end
		def initialize(data = nil)
			if( data == nil )
				super(2077)
			else
				super(2077, data)
			end
		end
	end

	class LegSecondaryAssetSubClass < Quickfix::IntField
		def LegSecondaryAssetSubClass.field
			return 2078
		end
		def initialize(data = nil)
			if( data == nil )
				super(2078)
			else
				super(2078, data)
			end
		end
	end

	class LegSecondaryAssetType < Quickfix::StringField
		def LegSecondaryAssetType.field
			return 2079
		end
		def initialize(data = nil)
			if( data == nil )
				super(2079)
			else
				super(2079, data)
			end
		end
	end

	class NoUnderlyingSecondaryAssetClasses < Quickfix::IntField
		def NoUnderlyingSecondaryAssetClasses.field
			return 2080
		end
		def initialize(data = nil)
			if( data == nil )
				super(2080)
			else
				super(2080, data)
			end
		end
	end

	class UnderlyingSecondaryAssetClass < Quickfix::IntField
		def UnderlyingSecondaryAssetClass.field
			return 2081
		end
		def initialize(data = nil)
			if( data == nil )
				super(2081)
			else
				super(2081, data)
			end
		end
	end

	class UnderlyingSecondaryAssetSubClass < Quickfix::IntField
		def UnderlyingSecondaryAssetSubClass.field
			return 2082
		end
		def initialize(data = nil)
			if( data == nil )
				super(2082)
			else
				super(2082, data)
			end
		end
	end

	class UnderlyingSecondaryAssetType < Quickfix::StringField
		def UnderlyingSecondaryAssetType.field
			return 2083
		end
		def initialize(data = nil)
			if( data == nil )
				super(2083)
			else
				super(2083, data)
			end
		end
	end

	class NoAdditionalTermBondRefs < Quickfix::IntField
		def NoAdditionalTermBondRefs.field
			return 40000
		end
		def initialize(data = nil)
			if( data == nil )
				super(40000)
			else
				super(40000, data)
			end
		end
	end

	class AdditionalTermBondSecurityID < Quickfix::StringField
		def AdditionalTermBondSecurityID.field
			return 40001
		end
		def initialize(data = nil)
			if( data == nil )
				super(40001)
			else
				super(40001, data)
			end
		end
	end

	class AdditionalTermBondSecurityIDSource < Quickfix::StringField
		def AdditionalTermBondSecurityIDSource.field
			return 40002
		end
		def initialize(data = nil)
			if( data == nil )
				super(40002)
			else
				super(40002, data)
			end
		end
	end

	class AdditionalTermBondDesc < Quickfix::StringField
		def AdditionalTermBondDesc.field
			return 40003
		end
		def initialize(data = nil)
			if( data == nil )
				super(40003)
			else
				super(40003, data)
			end
		end
	end

	class EncodedAdditionalTermBondDescLen < Quickfix::IntField
		def EncodedAdditionalTermBondDescLen.field
			return 40004
		end
		def initialize(data = nil)
			if( data == nil )
				super(40004)
			else
				super(40004, data)
			end
		end
	end

	class EncodedAdditionalTermBondDesc < Quickfix::StringField
		def EncodedAdditionalTermBondDesc.field
			return 40005
		end
		def initialize(data = nil)
			if( data == nil )
				super(40005)
			else
				super(40005, data)
			end
		end
	end

	class AdditionalTermBondCurrency < Quickfix::StringField
		def AdditionalTermBondCurrency.field
			return 40006
		end
		def initialize(data = nil)
			if( data == nil )
				super(40006)
			else
				super(40006, data)
			end
		end
	end

	class AdditionalTermBondIssuer < Quickfix::StringField
		def AdditionalTermBondIssuer.field
			return 40007
		end
		def initialize(data = nil)
			if( data == nil )
				super(40007)
			else
				super(40007, data)
			end
		end
	end

	class EncodedAdditionalTermBondIssuerLen < Quickfix::IntField
		def EncodedAdditionalTermBondIssuerLen.field
			return 40008
		end
		def initialize(data = nil)
			if( data == nil )
				super(40008)
			else
				super(40008, data)
			end
		end
	end

	class EncodedAdditionalTermBondIssuer < Quickfix::StringField
		def EncodedAdditionalTermBondIssuer.field
			return 40009
		end
		def initialize(data = nil)
			if( data == nil )
				super(40009)
			else
				super(40009, data)
			end
		end
	end

	class AdditionalTermBondSeniority < Quickfix::StringField
		def AdditionalTermBondSeniority.field
			return 40010
		end
		def initialize(data = nil)
			if( data == nil )
				super(40010)
			else
				super(40010, data)
			end
		end
	end

	class AdditionalTermBondCouponType < Quickfix::IntField
		def AdditionalTermBondCouponType.field
			return 40011
		end
		def initialize(data = nil)
			if( data == nil )
				super(40011)
			else
				super(40011, data)
			end
		end
	end

	class AdditionalTermBondCouponRate < Quickfix::DoubleField
		def AdditionalTermBondCouponRate.field
			return 40012
		end
		def initialize(data = nil)
			if( data == nil )
				super(40012)
			else
				super(40012, data)
			end
		end
	end

	class AdditionalTermBondMaturityDate < Quickfix::StringField
		def AdditionalTermBondMaturityDate.field
			return 40013
		end
		def initialize(data = nil)
			if( data == nil )
				super(40013)
			else
				super(40013, data)
			end
		end
	end

	class AdditionalTermBondParValue < Quickfix::DoubleField
		def AdditionalTermBondParValue.field
			return 40014
		end
		def initialize(data = nil)
			if( data == nil )
				super(40014)
			else
				super(40014, data)
			end
		end
	end

	class AdditionalTermBondCurrentTotalIssuedAmount < Quickfix::DoubleField
		def AdditionalTermBondCurrentTotalIssuedAmount.field
			return 40015
		end
		def initialize(data = nil)
			if( data == nil )
				super(40015)
			else
				super(40015, data)
			end
		end
	end

	class AdditionalTermBondCouponFrequencyPeriod < Quickfix::IntField
		def AdditionalTermBondCouponFrequencyPeriod.field
			return 40016
		end
		def initialize(data = nil)
			if( data == nil )
				super(40016)
			else
				super(40016, data)
			end
		end
	end

	class AdditionalTermBondCouponFrequencyUnit < Quickfix::StringField
		def AdditionalTermBondCouponFrequencyUnit.field
			return 40017
		end
		def initialize(data = nil)
			if( data == nil )
				super(40017)
			else
				super(40017, data)
			end
		end
	end

	class AdditionalTermBondDayCount < Quickfix::IntField
		def AdditionalTermBondDayCount.field
			return 40018
		end
		def initialize(data = nil)
			if( data == nil )
				super(40018)
			else
				super(40018, data)
			end
		end
	end

	class NoAdditionalTerms < Quickfix::IntField
		def NoAdditionalTerms.field
			return 40019
		end
		def initialize(data = nil)
			if( data == nil )
				super(40019)
			else
				super(40019, data)
			end
		end
	end

	class AdditionalTermConditionPrecedentBondIndicator < Quickfix::BoolField
		def AdditionalTermConditionPrecedentBondIndicator.field
			return 40020
		end
		def initialize(data = nil)
			if( data == nil )
				super(40020)
			else
				super(40020, data)
			end
		end
	end

	class AdditionalTermDiscrepancyClauseIndicator < Quickfix::BoolField
		def AdditionalTermDiscrepancyClauseIndicator.field
			return 40021
		end
		def initialize(data = nil)
			if( data == nil )
				super(40021)
			else
				super(40021, data)
			end
		end
	end

	class NoCashSettlTerms < Quickfix::IntField
		def NoCashSettlTerms.field
			return 40022
		end
		def initialize(data = nil)
			if( data == nil )
				super(40022)
			else
				super(40022, data)
			end
		end
	end

	class CashSettlCurrency < Quickfix::StringField
		def CashSettlCurrency.field
			return 40023
		end
		def initialize(data = nil)
			if( data == nil )
				super(40023)
			else
				super(40023, data)
			end
		end
	end

	class CashSettlValuationFirstBusinessDayOffset < Quickfix::IntField
		def CashSettlValuationFirstBusinessDayOffset.field
			return 40024
		end
		def initialize(data = nil)
			if( data == nil )
				super(40024)
			else
				super(40024, data)
			end
		end
	end

	class CashSettlValuationSubsequentBusinessDaysOffset < Quickfix::IntField
		def CashSettlValuationSubsequentBusinessDaysOffset.field
			return 40916
		end
		def initialize(data = nil)
			if( data == nil )
				super(40916)
			else
				super(40916, data)
			end
		end
	end

	class CashSettlNumOfValuationDates < Quickfix::IntField
		def CashSettlNumOfValuationDates.field
			return 40917
		end
		def initialize(data = nil)
			if( data == nil )
				super(40917)
			else
				super(40917, data)
			end
		end
	end

	class CashSettlValuationTime < Quickfix::StringField
		def CashSettlValuationTime.field
			return 40025
		end
		def initialize(data = nil)
			if( data == nil )
				super(40025)
			else
				super(40025, data)
			end
		end
	end

	class CashSettlBusinessCenter < Quickfix::StringField
		def CashSettlBusinessCenter.field
			return 40026
		end
		def initialize(data = nil)
			if( data == nil )
				super(40026)
			else
				super(40026, data)
			end
		end
	end

	class CashSettlQuoteMethod < Quickfix::IntField
		def CashSettlQuoteMethod.field
			return 40027
		end
		def initialize(data = nil)
			if( data == nil )
				super(40027)
			else
				super(40027, data)
			end
		end
	end

	class CashSettlQuoteAmount < Quickfix::DoubleField
		def CashSettlQuoteAmount.field
			return 40028
		end
		def initialize(data = nil)
			if( data == nil )
				super(40028)
			else
				super(40028, data)
			end
		end
	end

	class CashSettlQuoteCurrency < Quickfix::StringField
		def CashSettlQuoteCurrency.field
			return 40029
		end
		def initialize(data = nil)
			if( data == nil )
				super(40029)
			else
				super(40029, data)
			end
		end
	end

	class CashSettlMinimumQuoteAmount < Quickfix::DoubleField
		def CashSettlMinimumQuoteAmount.field
			return 40030
		end
		def initialize(data = nil)
			if( data == nil )
				super(40030)
			else
				super(40030, data)
			end
		end
	end

	class CashSettlMinimumQuoteCurrency < Quickfix::StringField
		def CashSettlMinimumQuoteCurrency.field
			return 40031
		end
		def initialize(data = nil)
			if( data == nil )
				super(40031)
			else
				super(40031, data)
			end
		end
	end

	class CashSettlDealer < Quickfix::StringField
		def CashSettlDealer.field
			return 40032
		end
		def initialize(data = nil)
			if( data == nil )
				super(40032)
			else
				super(40032, data)
			end
		end
	end

	class CashSettlBusinessDays < Quickfix::IntField
		def CashSettlBusinessDays.field
			return 40033
		end
		def initialize(data = nil)
			if( data == nil )
				super(40033)
			else
				super(40033, data)
			end
		end
	end

	class CashSettlAmount < Quickfix::DoubleField
		def CashSettlAmount.field
			return 40034
		end
		def initialize(data = nil)
			if( data == nil )
				super(40034)
			else
				super(40034, data)
			end
		end
	end

	class CashSettlRecoveryFactor < Quickfix::DoubleField
		def CashSettlRecoveryFactor.field
			return 40035
		end
		def initialize(data = nil)
			if( data == nil )
				super(40035)
			else
				super(40035, data)
			end
		end
	end

	class CashSettlFixedTermIndicator < Quickfix::BoolField
		def CashSettlFixedTermIndicator.field
			return 40036
		end
		def initialize(data = nil)
			if( data == nil )
				super(40036)
			else
				super(40036, data)
			end
		end
	end

	class CashSettlAccruedInterestIndicator < Quickfix::BoolField
		def CashSettlAccruedInterestIndicator.field
			return 40037
		end
		def initialize(data = nil)
			if( data == nil )
				super(40037)
			else
				super(40037, data)
			end
		end
	end

	class CashSettlValuationMethod < Quickfix::IntField
		def CashSettlValuationMethod.field
			return 40038
		end
		def initialize(data = nil)
			if( data == nil )
				super(40038)
			else
				super(40038, data)
			end
		end
	end

	class CashSettlTermXID < Quickfix::StringField
		def CashSettlTermXID.field
			return 40039
		end
		def initialize(data = nil)
			if( data == nil )
				super(40039)
			else
				super(40039, data)
			end
		end
	end

	class NoContractualDefinitions < Quickfix::IntField
		def NoContractualDefinitions.field
			return 40040
		end
		def initialize(data = nil)
			if( data == nil )
				super(40040)
			else
				super(40040, data)
			end
		end
	end

	class ContractualDefinition < Quickfix::StringField
		def ContractualDefinition.field
			return 40041
		end
		def initialize(data = nil)
			if( data == nil )
				super(40041)
			else
				super(40041, data)
			end
		end
	end

	class NoContractualMatrices < Quickfix::IntField
		def NoContractualMatrices.field
			return 40042
		end
		def initialize(data = nil)
			if( data == nil )
				super(40042)
			else
				super(40042, data)
			end
		end
	end

	class ContractualMatrixSource < Quickfix::StringField
		def ContractualMatrixSource.field
			return 40043
		end
		def initialize(data = nil)
			if( data == nil )
				super(40043)
			else
				super(40043, data)
			end
		end
	end

	class ContractualMatrixDate < Quickfix::StringField
		def ContractualMatrixDate.field
			return 40044
		end
		def initialize(data = nil)
			if( data == nil )
				super(40044)
			else
				super(40044, data)
			end
		end
	end

	class ContractualMatrixTerm < Quickfix::StringField
		def ContractualMatrixTerm.field
			return 40045
		end
		def initialize(data = nil)
			if( data == nil )
				super(40045)
			else
				super(40045, data)
			end
		end
	end

	class NoFinancingTermSupplements < Quickfix::IntField
		def NoFinancingTermSupplements.field
			return 40046
		end
		def initialize(data = nil)
			if( data == nil )
				super(40046)
			else
				super(40046, data)
			end
		end
	end

	class FinancingTermSupplementDesc < Quickfix::StringField
		def FinancingTermSupplementDesc.field
			return 40047
		end
		def initialize(data = nil)
			if( data == nil )
				super(40047)
			else
				super(40047, data)
			end
		end
	end

	class FinancingTermSupplementDate < Quickfix::StringField
		def FinancingTermSupplementDate.field
			return 40048
		end
		def initialize(data = nil)
			if( data == nil )
				super(40048)
			else
				super(40048, data)
			end
		end
	end

	class NoStreams < Quickfix::IntField
		def NoStreams.field
			return 40049
		end
		def initialize(data = nil)
			if( data == nil )
				super(40049)
			else
				super(40049, data)
			end
		end
	end

	class StreamType < Quickfix::IntField
		def StreamType.field
			return 40050
		end
		def initialize(data = nil)
			if( data == nil )
				super(40050)
			else
				super(40050, data)
			end
		end
	end

	class StreamDesc < Quickfix::StringField
		def StreamDesc.field
			return 40051
		end
		def initialize(data = nil)
			if( data == nil )
				super(40051)
			else
				super(40051, data)
			end
		end
	end

	class StreamPaySide < Quickfix::IntField
		def StreamPaySide.field
			return 40052
		end
		def initialize(data = nil)
			if( data == nil )
				super(40052)
			else
				super(40052, data)
			end
		end
	end

	class StreamReceiveSide < Quickfix::IntField
		def StreamReceiveSide.field
			return 40053
		end
		def initialize(data = nil)
			if( data == nil )
				super(40053)
			else
				super(40053, data)
			end
		end
	end

	class StreamNotional < Quickfix::DoubleField
		def StreamNotional.field
			return 40054
		end
		def initialize(data = nil)
			if( data == nil )
				super(40054)
			else
				super(40054, data)
			end
		end
	end

	class StreamCurrency < Quickfix::StringField
		def StreamCurrency.field
			return 40055
		end
		def initialize(data = nil)
			if( data == nil )
				super(40055)
			else
				super(40055, data)
			end
		end
	end

	class StreamText < Quickfix::StringField
		def StreamText.field
			return 40056
		end
		def initialize(data = nil)
			if( data == nil )
				super(40056)
			else
				super(40056, data)
			end
		end
	end

	class UnderlyingStreamEffectiveDateUnadjusted < Quickfix::StringField
		def UnderlyingStreamEffectiveDateUnadjusted.field
			return 40057
		end
		def initialize(data = nil)
			if( data == nil )
				super(40057)
			else
				super(40057, data)
			end
		end
	end

	class UnderlyingStreamEffectiveDateBusinessDayConvention < Quickfix::IntField
		def UnderlyingStreamEffectiveDateBusinessDayConvention.field
			return 40058
		end
		def initialize(data = nil)
			if( data == nil )
				super(40058)
			else
				super(40058, data)
			end
		end
	end

	class UnderlyingStreamEffectiveDateBusinessCenter < Quickfix::StringField
		def UnderlyingStreamEffectiveDateBusinessCenter.field
			return 40059
		end
		def initialize(data = nil)
			if( data == nil )
				super(40059)
			else
				super(40059, data)
			end
		end
	end

	class UnderlyingStreamEffectiveDateRelativeTo < Quickfix::IntField
		def UnderlyingStreamEffectiveDateRelativeTo.field
			return 40060
		end
		def initialize(data = nil)
			if( data == nil )
				super(40060)
			else
				super(40060, data)
			end
		end
	end

	class UnderlyingStreamEffectiveDateOffsetPeriod < Quickfix::IntField
		def UnderlyingStreamEffectiveDateOffsetPeriod.field
			return 40061
		end
		def initialize(data = nil)
			if( data == nil )
				super(40061)
			else
				super(40061, data)
			end
		end
	end

	class UnderlyingStreamEffectiveDateOffsetUnit < Quickfix::StringField
		def UnderlyingStreamEffectiveDateOffsetUnit.field
			return 40062
		end
		def initialize(data = nil)
			if( data == nil )
				super(40062)
			else
				super(40062, data)
			end
		end
	end

	class UnderlyingStreamEffectiveDateOffsetDayType < Quickfix::IntField
		def UnderlyingStreamEffectiveDateOffsetDayType.field
			return 40063
		end
		def initialize(data = nil)
			if( data == nil )
				super(40063)
			else
				super(40063, data)
			end
		end
	end

	class UnderlyingStreamEffectiveDateAdjusted < Quickfix::StringField
		def UnderlyingStreamEffectiveDateAdjusted.field
			return 40064
		end
		def initialize(data = nil)
			if( data == nil )
				super(40064)
			else
				super(40064, data)
			end
		end
	end

	class StreamTerminationDateUnadjusted < Quickfix::StringField
		def StreamTerminationDateUnadjusted.field
			return 40065
		end
		def initialize(data = nil)
			if( data == nil )
				super(40065)
			else
				super(40065, data)
			end
		end
	end

	class StreamTerminationDateBusinessDayConvention < Quickfix::IntField
		def StreamTerminationDateBusinessDayConvention.field
			return 40066
		end
		def initialize(data = nil)
			if( data == nil )
				super(40066)
			else
				super(40066, data)
			end
		end
	end

	class StreamTerminationDateBusinessCenter < Quickfix::StringField
		def StreamTerminationDateBusinessCenter.field
			return 40067
		end
		def initialize(data = nil)
			if( data == nil )
				super(40067)
			else
				super(40067, data)
			end
		end
	end

	class StreamTerminationDateRelativeTo < Quickfix::IntField
		def StreamTerminationDateRelativeTo.field
			return 40068
		end
		def initialize(data = nil)
			if( data == nil )
				super(40068)
			else
				super(40068, data)
			end
		end
	end

	class StreamTerminationDateOffsetPeriod < Quickfix::IntField
		def StreamTerminationDateOffsetPeriod.field
			return 40069
		end
		def initialize(data = nil)
			if( data == nil )
				super(40069)
			else
				super(40069, data)
			end
		end
	end

	class StreamTerminationDateOffsetUnit < Quickfix::StringField
		def StreamTerminationDateOffsetUnit.field
			return 40070
		end
		def initialize(data = nil)
			if( data == nil )
				super(40070)
			else
				super(40070, data)
			end
		end
	end

	class StreamTerminationDateOffsetDayType < Quickfix::IntField
		def StreamTerminationDateOffsetDayType.field
			return 40071
		end
		def initialize(data = nil)
			if( data == nil )
				super(40071)
			else
				super(40071, data)
			end
		end
	end

	class StreamTerminationDateAdjusted < Quickfix::StringField
		def StreamTerminationDateAdjusted.field
			return 40072
		end
		def initialize(data = nil)
			if( data == nil )
				super(40072)
			else
				super(40072, data)
			end
		end
	end

	class StreamCalculationPeriodBusinessDayConvention < Quickfix::IntField
		def StreamCalculationPeriodBusinessDayConvention.field
			return 40073
		end
		def initialize(data = nil)
			if( data == nil )
				super(40073)
			else
				super(40073, data)
			end
		end
	end

	class StreamCalculationPeriodBusinessCenter < Quickfix::StringField
		def StreamCalculationPeriodBusinessCenter.field
			return 40074
		end
		def initialize(data = nil)
			if( data == nil )
				super(40074)
			else
				super(40074, data)
			end
		end
	end

	class StreamFirstPeriodStartDateUnadjusted < Quickfix::StringField
		def StreamFirstPeriodStartDateUnadjusted.field
			return 40075
		end
		def initialize(data = nil)
			if( data == nil )
				super(40075)
			else
				super(40075, data)
			end
		end
	end

	class StreamFirstPeriodStartDateBusinessDayConvention < Quickfix::IntField
		def StreamFirstPeriodStartDateBusinessDayConvention.field
			return 40076
		end
		def initialize(data = nil)
			if( data == nil )
				super(40076)
			else
				super(40076, data)
			end
		end
	end

	class StreamFirstPeriodStartDateBusinessCenter < Quickfix::StringField
		def StreamFirstPeriodStartDateBusinessCenter.field
			return 40077
		end
		def initialize(data = nil)
			if( data == nil )
				super(40077)
			else
				super(40077, data)
			end
		end
	end

	class StreamFirstPeriodStartDateAdjusted < Quickfix::StringField
		def StreamFirstPeriodStartDateAdjusted.field
			return 40078
		end
		def initialize(data = nil)
			if( data == nil )
				super(40078)
			else
				super(40078, data)
			end
		end
	end

	class StreamFirstRegularPeriodStartDateUnadjusted < Quickfix::StringField
		def StreamFirstRegularPeriodStartDateUnadjusted.field
			return 40079
		end
		def initialize(data = nil)
			if( data == nil )
				super(40079)
			else
				super(40079, data)
			end
		end
	end

	class StreamFirstCompoundingPeriodEndDateUnadjusted < Quickfix::StringField
		def StreamFirstCompoundingPeriodEndDateUnadjusted.field
			return 40080
		end
		def initialize(data = nil)
			if( data == nil )
				super(40080)
			else
				super(40080, data)
			end
		end
	end

	class StreamLastRegularPeriodEndDateUnadjusted < Quickfix::StringField
		def StreamLastRegularPeriodEndDateUnadjusted.field
			return 40081
		end
		def initialize(data = nil)
			if( data == nil )
				super(40081)
			else
				super(40081, data)
			end
		end
	end

	class StreamCalculationFrequencyPeriod < Quickfix::IntField
		def StreamCalculationFrequencyPeriod.field
			return 40082
		end
		def initialize(data = nil)
			if( data == nil )
				super(40082)
			else
				super(40082, data)
			end
		end
	end

	class StreamCalculationFrequencyUnit < Quickfix::StringField
		def StreamCalculationFrequencyUnit.field
			return 40083
		end
		def initialize(data = nil)
			if( data == nil )
				super(40083)
			else
				super(40083, data)
			end
		end
	end

	class StreamCalculationRollConvention < Quickfix::StringField
		def StreamCalculationRollConvention.field
			return 40084
		end
		def initialize(data = nil)
			if( data == nil )
				super(40084)
			else
				super(40084, data)
			end
		end
	end

	class NoSettlRateFallbacks < Quickfix::IntField
		def NoSettlRateFallbacks.field
			return 40085
		end
		def initialize(data = nil)
			if( data == nil )
				super(40085)
			else
				super(40085, data)
			end
		end
	end

	class SettlRatePostponementMaximumDays < Quickfix::IntField
		def SettlRatePostponementMaximumDays.field
			return 40086
		end
		def initialize(data = nil)
			if( data == nil )
				super(40086)
			else
				super(40086, data)
			end
		end
	end

	class LegPaymentStreamNonDeliverableSettlRateSource < Quickfix::IntField
		def LegPaymentStreamNonDeliverableSettlRateSource.field
			return 40087
		end
		def initialize(data = nil)
			if( data == nil )
				super(40087)
			else
				super(40087, data)
			end
		end
	end

	class SettlRatePostponementSurvey < Quickfix::BoolField
		def SettlRatePostponementSurvey.field
			return 40088
		end
		def initialize(data = nil)
			if( data == nil )
				super(40088)
			else
				super(40088, data)
			end
		end
	end

	class SettlRatePostponementCalculationAgent < Quickfix::IntField
		def SettlRatePostponementCalculationAgent.field
			return 40089
		end
		def initialize(data = nil)
			if( data == nil )
				super(40089)
			else
				super(40089, data)
			end
		end
	end

	class NoProvisions < Quickfix::IntField
		def NoProvisions.field
			return 40090
		end
		def initialize(data = nil)
			if( data == nil )
				super(40090)
			else
				super(40090, data)
			end
		end
	end

	class ProvisionType < Quickfix::IntField
		def ProvisionType.field
			return 40091
		end
		def initialize(data = nil)
			if( data == nil )
				super(40091)
			else
				super(40091, data)
			end
		end
	end

	class ProvisionDateUnadjusted < Quickfix::StringField
		def ProvisionDateUnadjusted.field
			return 40092
		end
		def initialize(data = nil)
			if( data == nil )
				super(40092)
			else
				super(40092, data)
			end
		end
	end

	class ProvisionDateBusinessDayConvention < Quickfix::IntField
		def ProvisionDateBusinessDayConvention.field
			return 40093
		end
		def initialize(data = nil)
			if( data == nil )
				super(40093)
			else
				super(40093, data)
			end
		end
	end

	class ProvisionDateBusinessCenter < Quickfix::StringField
		def ProvisionDateBusinessCenter.field
			return 40094
		end
		def initialize(data = nil)
			if( data == nil )
				super(40094)
			else
				super(40094, data)
			end
		end
	end

	class ProvisionDateAdjusted < Quickfix::StringField
		def ProvisionDateAdjusted.field
			return 40095
		end
		def initialize(data = nil)
			if( data == nil )
				super(40095)
			else
				super(40095, data)
			end
		end
	end

	class ProvisionDateTenorPeriod < Quickfix::IntField
		def ProvisionDateTenorPeriod.field
			return 40096
		end
		def initialize(data = nil)
			if( data == nil )
				super(40096)
			else
				super(40096, data)
			end
		end
	end

	class ProvisionDateTenorUnit < Quickfix::StringField
		def ProvisionDateTenorUnit.field
			return 40097
		end
		def initialize(data = nil)
			if( data == nil )
				super(40097)
			else
				super(40097, data)
			end
		end
	end

	class ProvisionCalculationAgent < Quickfix::IntField
		def ProvisionCalculationAgent.field
			return 40098
		end
		def initialize(data = nil)
			if( data == nil )
				super(40098)
			else
				super(40098, data)
			end
		end
	end

	class ProvisionOptionSinglePartyBuyerSide < Quickfix::IntField
		def ProvisionOptionSinglePartyBuyerSide.field
			return 40099
		end
		def initialize(data = nil)
			if( data == nil )
				super(40099)
			else
				super(40099, data)
			end
		end
	end

	class ProvisionOptionSinglePartySellerSide < Quickfix::IntField
		def ProvisionOptionSinglePartySellerSide.field
			return 40100
		end
		def initialize(data = nil)
			if( data == nil )
				super(40100)
			else
				super(40100, data)
			end
		end
	end

	class ProvisionOptionExerciseStyle < Quickfix::IntField
		def ProvisionOptionExerciseStyle.field
			return 40101
		end
		def initialize(data = nil)
			if( data == nil )
				super(40101)
			else
				super(40101, data)
			end
		end
	end

	class ProvisionOptionExerciseMultipleNotional < Quickfix::DoubleField
		def ProvisionOptionExerciseMultipleNotional.field
			return 40102
		end
		def initialize(data = nil)
			if( data == nil )
				super(40102)
			else
				super(40102, data)
			end
		end
	end

	class ProvisionOptionExerciseMinimumNotional < Quickfix::DoubleField
		def ProvisionOptionExerciseMinimumNotional.field
			return 40103
		end
		def initialize(data = nil)
			if( data == nil )
				super(40103)
			else
				super(40103, data)
			end
		end
	end

	class ProvisionOptionExerciseMaximumNotional < Quickfix::DoubleField
		def ProvisionOptionExerciseMaximumNotional.field
			return 40104
		end
		def initialize(data = nil)
			if( data == nil )
				super(40104)
			else
				super(40104, data)
			end
		end
	end

	class ProvisionOptionMinimumNumber < Quickfix::IntField
		def ProvisionOptionMinimumNumber.field
			return 40105
		end
		def initialize(data = nil)
			if( data == nil )
				super(40105)
			else
				super(40105, data)
			end
		end
	end

	class ProvisionOptionMaximumNumber < Quickfix::IntField
		def ProvisionOptionMaximumNumber.field
			return 40106
		end
		def initialize(data = nil)
			if( data == nil )
				super(40106)
			else
				super(40106, data)
			end
		end
	end

	class ProvisionOptionExerciseConfirmation < Quickfix::BoolField
		def ProvisionOptionExerciseConfirmation.field
			return 40107
		end
		def initialize(data = nil)
			if( data == nil )
				super(40107)
			else
				super(40107, data)
			end
		end
	end

	class ProvisionCashSettlMethod < Quickfix::IntField
		def ProvisionCashSettlMethod.field
			return 40108
		end
		def initialize(data = nil)
			if( data == nil )
				super(40108)
			else
				super(40108, data)
			end
		end
	end

	class ProvisionCashSettlCurrency < Quickfix::StringField
		def ProvisionCashSettlCurrency.field
			return 40109
		end
		def initialize(data = nil)
			if( data == nil )
				super(40109)
			else
				super(40109, data)
			end
		end
	end

	class ProvisionCashSettlCurrency2 < Quickfix::StringField
		def ProvisionCashSettlCurrency2.field
			return 40110
		end
		def initialize(data = nil)
			if( data == nil )
				super(40110)
			else
				super(40110, data)
			end
		end
	end

	class ProvisionCashSettlQuoteType < Quickfix::IntField
		def ProvisionCashSettlQuoteType.field
			return 40111
		end
		def initialize(data = nil)
			if( data == nil )
				super(40111)
			else
				super(40111, data)
			end
		end
	end

	class ProvisionCashSettlQuoteSource < Quickfix::IntField
		def ProvisionCashSettlQuoteSource.field
			return 40112
		end
		def initialize(data = nil)
			if( data == nil )
				super(40112)
			else
				super(40112, data)
			end
		end
	end

	class ProvisionText < Quickfix::StringField
		def ProvisionText.field
			return 40113
		end
		def initialize(data = nil)
			if( data == nil )
				super(40113)
			else
				super(40113, data)
			end
		end
	end

	class ProvisionCashSettlValueTime < Quickfix::StringField
		def ProvisionCashSettlValueTime.field
			return 40114
		end
		def initialize(data = nil)
			if( data == nil )
				super(40114)
			else
				super(40114, data)
			end
		end
	end

	class ProvisionCashSettlValueTimeBusinessCenter < Quickfix::StringField
		def ProvisionCashSettlValueTimeBusinessCenter.field
			return 40115
		end
		def initialize(data = nil)
			if( data == nil )
				super(40115)
			else
				super(40115, data)
			end
		end
	end

	class ProvisionCashSettlValueDateBusinessDayConvention < Quickfix::IntField
		def ProvisionCashSettlValueDateBusinessDayConvention.field
			return 40116
		end
		def initialize(data = nil)
			if( data == nil )
				super(40116)
			else
				super(40116, data)
			end
		end
	end

	class ProvisionCashSettlValueDateBusinessCenter < Quickfix::StringField
		def ProvisionCashSettlValueDateBusinessCenter.field
			return 40117
		end
		def initialize(data = nil)
			if( data == nil )
				super(40117)
			else
				super(40117, data)
			end
		end
	end

	class ProvisionCashSettlValueDateRelativeTo < Quickfix::IntField
		def ProvisionCashSettlValueDateRelativeTo.field
			return 40118
		end
		def initialize(data = nil)
			if( data == nil )
				super(40118)
			else
				super(40118, data)
			end
		end
	end

	class ProvisionCashSettlValueDateOffsetPeriod < Quickfix::IntField
		def ProvisionCashSettlValueDateOffsetPeriod.field
			return 40119
		end
		def initialize(data = nil)
			if( data == nil )
				super(40119)
			else
				super(40119, data)
			end
		end
	end

	class ProvisionCashSettlValueDateOffsetUnit < Quickfix::StringField
		def ProvisionCashSettlValueDateOffsetUnit.field
			return 40120
		end
		def initialize(data = nil)
			if( data == nil )
				super(40120)
			else
				super(40120, data)
			end
		end
	end

	class ProvisionCashSettlValueDateOffsetDayType < Quickfix::IntField
		def ProvisionCashSettlValueDateOffsetDayType.field
			return 40121
		end
		def initialize(data = nil)
			if( data == nil )
				super(40121)
			else
				super(40121, data)
			end
		end
	end

	class ProvisionCashSettlValueDateAdjusted < Quickfix::StringField
		def ProvisionCashSettlValueDateAdjusted.field
			return 40122
		end
		def initialize(data = nil)
			if( data == nil )
				super(40122)
			else
				super(40122, data)
			end
		end
	end

	class ProvisionOptionExerciseBusinessDayConvention < Quickfix::IntField
		def ProvisionOptionExerciseBusinessDayConvention.field
			return 40123
		end
		def initialize(data = nil)
			if( data == nil )
				super(40123)
			else
				super(40123, data)
			end
		end
	end

	class ProvisionOptionExerciseBusinessCenter < Quickfix::StringField
		def ProvisionOptionExerciseBusinessCenter.field
			return 40124
		end
		def initialize(data = nil)
			if( data == nil )
				super(40124)
			else
				super(40124, data)
			end
		end
	end

	class ProvisionOptionExerciseEarliestDateOffsetPeriod < Quickfix::IntField
		def ProvisionOptionExerciseEarliestDateOffsetPeriod.field
			return 40125
		end
		def initialize(data = nil)
			if( data == nil )
				super(40125)
			else
				super(40125, data)
			end
		end
	end

	class ProvisionOptionExerciseEarliestDateOffsetUnit < Quickfix::StringField
		def ProvisionOptionExerciseEarliestDateOffsetUnit.field
			return 40126
		end
		def initialize(data = nil)
			if( data == nil )
				super(40126)
			else
				super(40126, data)
			end
		end
	end

	class ProvisionOptionExerciseFrequencyPeriod < Quickfix::IntField
		def ProvisionOptionExerciseFrequencyPeriod.field
			return 40127
		end
		def initialize(data = nil)
			if( data == nil )
				super(40127)
			else
				super(40127, data)
			end
		end
	end

	class ProvisionOptionExerciseFrequencyUnit < Quickfix::StringField
		def ProvisionOptionExerciseFrequencyUnit.field
			return 40128
		end
		def initialize(data = nil)
			if( data == nil )
				super(40128)
			else
				super(40128, data)
			end
		end
	end

	class ProvisionOptionExerciseStartDateUnadjusted < Quickfix::StringField
		def ProvisionOptionExerciseStartDateUnadjusted.field
			return 40129
		end
		def initialize(data = nil)
			if( data == nil )
				super(40129)
			else
				super(40129, data)
			end
		end
	end

	class ProvisionOptionExerciseStartDateRelativeTo < Quickfix::IntField
		def ProvisionOptionExerciseStartDateRelativeTo.field
			return 40130
		end
		def initialize(data = nil)
			if( data == nil )
				super(40130)
			else
				super(40130, data)
			end
		end
	end

	class ProvisionOptionExerciseStartDateOffsetPeriod < Quickfix::IntField
		def ProvisionOptionExerciseStartDateOffsetPeriod.field
			return 40131
		end
		def initialize(data = nil)
			if( data == nil )
				super(40131)
			else
				super(40131, data)
			end
		end
	end

	class ProvisionOptionExerciseStartDateOffsetUnit < Quickfix::StringField
		def ProvisionOptionExerciseStartDateOffsetUnit.field
			return 40132
		end
		def initialize(data = nil)
			if( data == nil )
				super(40132)
			else
				super(40132, data)
			end
		end
	end

	class ProvisionOptionExerciseStartDateOffsetDayType < Quickfix::IntField
		def ProvisionOptionExerciseStartDateOffsetDayType.field
			return 40133
		end
		def initialize(data = nil)
			if( data == nil )
				super(40133)
			else
				super(40133, data)
			end
		end
	end

	class ProvisionOptionExerciseStartDateAdjusted < Quickfix::StringField
		def ProvisionOptionExerciseStartDateAdjusted.field
			return 40134
		end
		def initialize(data = nil)
			if( data == nil )
				super(40134)
			else
				super(40134, data)
			end
		end
	end

	class ProvisionOptionExercisePeriodSkip < Quickfix::IntField
		def ProvisionOptionExercisePeriodSkip.field
			return 40135
		end
		def initialize(data = nil)
			if( data == nil )
				super(40135)
			else
				super(40135, data)
			end
		end
	end

	class ProvisionOptionExerciseBoundsFirstDateUnadjusted < Quickfix::StringField
		def ProvisionOptionExerciseBoundsFirstDateUnadjusted.field
			return 40136
		end
		def initialize(data = nil)
			if( data == nil )
				super(40136)
			else
				super(40136, data)
			end
		end
	end

	class ProvisionOptionExerciseBoundsLastDateUnadjusted < Quickfix::StringField
		def ProvisionOptionExerciseBoundsLastDateUnadjusted.field
			return 40137
		end
		def initialize(data = nil)
			if( data == nil )
				super(40137)
			else
				super(40137, data)
			end
		end
	end

	class ProvisionOptionExerciseEarliestTime < Quickfix::StringField
		def ProvisionOptionExerciseEarliestTime.field
			return 40138
		end
		def initialize(data = nil)
			if( data == nil )
				super(40138)
			else
				super(40138, data)
			end
		end
	end

	class ProvisionOptionExerciseEarliestTimeBusinessCenter < Quickfix::StringField
		def ProvisionOptionExerciseEarliestTimeBusinessCenter.field
			return 40139
		end
		def initialize(data = nil)
			if( data == nil )
				super(40139)
			else
				super(40139, data)
			end
		end
	end

	class ProvisionOptionExerciseLatestTime < Quickfix::StringField
		def ProvisionOptionExerciseLatestTime.field
			return 40140
		end
		def initialize(data = nil)
			if( data == nil )
				super(40140)
			else
				super(40140, data)
			end
		end
	end

	class ProvisionOptionExerciseLatestTimeBusinessCenter < Quickfix::StringField
		def ProvisionOptionExerciseLatestTimeBusinessCenter.field
			return 40141
		end
		def initialize(data = nil)
			if( data == nil )
				super(40141)
			else
				super(40141, data)
			end
		end
	end

	class NoProvisionOptionExerciseFixedDates < Quickfix::IntField
		def NoProvisionOptionExerciseFixedDates.field
			return 40142
		end
		def initialize(data = nil)
			if( data == nil )
				super(40142)
			else
				super(40142, data)
			end
		end
	end

	class ProvisionOptionExerciseFixedDate < Quickfix::StringField
		def ProvisionOptionExerciseFixedDate.field
			return 40143
		end
		def initialize(data = nil)
			if( data == nil )
				super(40143)
			else
				super(40143, data)
			end
		end
	end

	class ProvisionOptionExerciseFixedDateType < Quickfix::IntField
		def ProvisionOptionExerciseFixedDateType.field
			return 40144
		end
		def initialize(data = nil)
			if( data == nil )
				super(40144)
			else
				super(40144, data)
			end
		end
	end

	class ProvisionOptionExpirationDateUnadjusted < Quickfix::StringField
		def ProvisionOptionExpirationDateUnadjusted.field
			return 40145
		end
		def initialize(data = nil)
			if( data == nil )
				super(40145)
			else
				super(40145, data)
			end
		end
	end

	class ProvisionOptionExpirationDateBusinessDayConvention < Quickfix::IntField
		def ProvisionOptionExpirationDateBusinessDayConvention.field
			return 40146
		end
		def initialize(data = nil)
			if( data == nil )
				super(40146)
			else
				super(40146, data)
			end
		end
	end

	class ProvisionOptionExpirationDateBusinessCenter < Quickfix::StringField
		def ProvisionOptionExpirationDateBusinessCenter.field
			return 40147
		end
		def initialize(data = nil)
			if( data == nil )
				super(40147)
			else
				super(40147, data)
			end
		end
	end

	class ProvisionOptionExpirationDateRelativeTo < Quickfix::IntField
		def ProvisionOptionExpirationDateRelativeTo.field
			return 40148
		end
		def initialize(data = nil)
			if( data == nil )
				super(40148)
			else
				super(40148, data)
			end
		end
	end

	class ProvisionOptionExpirationDateOffsetPeriod < Quickfix::IntField
		def ProvisionOptionExpirationDateOffsetPeriod.field
			return 40149
		end
		def initialize(data = nil)
			if( data == nil )
				super(40149)
			else
				super(40149, data)
			end
		end
	end

	class ProvisionOptionExpirationDateOffsetUnit < Quickfix::StringField
		def ProvisionOptionExpirationDateOffsetUnit.field
			return 40150
		end
		def initialize(data = nil)
			if( data == nil )
				super(40150)
			else
				super(40150, data)
			end
		end
	end

	class ProvisionOptionExpirationDateOffsetDayType < Quickfix::IntField
		def ProvisionOptionExpirationDateOffsetDayType.field
			return 40151
		end
		def initialize(data = nil)
			if( data == nil )
				super(40151)
			else
				super(40151, data)
			end
		end
	end

	class ProvisionOptionExpirationDateAdjusted < Quickfix::StringField
		def ProvisionOptionExpirationDateAdjusted.field
			return 40152
		end
		def initialize(data = nil)
			if( data == nil )
				super(40152)
			else
				super(40152, data)
			end
		end
	end

	class ProvisionOptionExpirationTime < Quickfix::StringField
		def ProvisionOptionExpirationTime.field
			return 40153
		end
		def initialize(data = nil)
			if( data == nil )
				super(40153)
			else
				super(40153, data)
			end
		end
	end

	class ProvisionOptionExpirationTimeBusinessCenter < Quickfix::StringField
		def ProvisionOptionExpirationTimeBusinessCenter.field
			return 40154
		end
		def initialize(data = nil)
			if( data == nil )
				super(40154)
			else
				super(40154, data)
			end
		end
	end

	class ProvisionOptionRelevantUnderlyingDateUnadjusted < Quickfix::StringField
		def ProvisionOptionRelevantUnderlyingDateUnadjusted.field
			return 40155
		end
		def initialize(data = nil)
			if( data == nil )
				super(40155)
			else
				super(40155, data)
			end
		end
	end

	class ProvisionOptionRelevantUnderlyingDateBusinessDayConvention < Quickfix::IntField
		def ProvisionOptionRelevantUnderlyingDateBusinessDayConvention.field
			return 40156
		end
		def initialize(data = nil)
			if( data == nil )
				super(40156)
			else
				super(40156, data)
			end
		end
	end

	class ProvisionOptionRelevantUnderlyingDateBusinessCenter < Quickfix::StringField
		def ProvisionOptionRelevantUnderlyingDateBusinessCenter.field
			return 40157
		end
		def initialize(data = nil)
			if( data == nil )
				super(40157)
			else
				super(40157, data)
			end
		end
	end

	class ProvisionOptionRelevantUnderlyingDateRelativeTo < Quickfix::IntField
		def ProvisionOptionRelevantUnderlyingDateRelativeTo.field
			return 40158
		end
		def initialize(data = nil)
			if( data == nil )
				super(40158)
			else
				super(40158, data)
			end
		end
	end

	class ProvisionOptionRelevantUnderlyingDateOffsetPeriod < Quickfix::IntField
		def ProvisionOptionRelevantUnderlyingDateOffsetPeriod.field
			return 40159
		end
		def initialize(data = nil)
			if( data == nil )
				super(40159)
			else
				super(40159, data)
			end
		end
	end

	class ProvisionOptionRelevantUnderlyingDateOffsetUnit < Quickfix::StringField
		def ProvisionOptionRelevantUnderlyingDateOffsetUnit.field
			return 40160
		end
		def initialize(data = nil)
			if( data == nil )
				super(40160)
			else
				super(40160, data)
			end
		end
	end

	class ProvisionOptionRelevantUnderlyingDateOffsetDayType < Quickfix::IntField
		def ProvisionOptionRelevantUnderlyingDateOffsetDayType.field
			return 40161
		end
		def initialize(data = nil)
			if( data == nil )
				super(40161)
			else
				super(40161, data)
			end
		end
	end

	class ProvisionOptionRelevantUnderlyingDateAdjusted < Quickfix::StringField
		def ProvisionOptionRelevantUnderlyingDateAdjusted.field
			return 40162
		end
		def initialize(data = nil)
			if( data == nil )
				super(40162)
			else
				super(40162, data)
			end
		end
	end

	class ProvisionCashSettlPaymentDateBusinessDayConvention < Quickfix::IntField
		def ProvisionCashSettlPaymentDateBusinessDayConvention.field
			return 40163
		end
		def initialize(data = nil)
			if( data == nil )
				super(40163)
			else
				super(40163, data)
			end
		end
	end

	class ProvisionCashSettlPaymentDateBusinessCenter < Quickfix::StringField
		def ProvisionCashSettlPaymentDateBusinessCenter.field
			return 40164
		end
		def initialize(data = nil)
			if( data == nil )
				super(40164)
			else
				super(40164, data)
			end
		end
	end

	class ProvisionCashSettlPaymentDateRelativeTo < Quickfix::IntField
		def ProvisionCashSettlPaymentDateRelativeTo.field
			return 40165
		end
		def initialize(data = nil)
			if( data == nil )
				super(40165)
			else
				super(40165, data)
			end
		end
	end

	class ProvisionCashSettlPaymentDateOffsetPeriod < Quickfix::IntField
		def ProvisionCashSettlPaymentDateOffsetPeriod.field
			return 40166
		end
		def initialize(data = nil)
			if( data == nil )
				super(40166)
			else
				super(40166, data)
			end
		end
	end

	class ProvisionCashSettlPaymentDateOffsetUnit < Quickfix::StringField
		def ProvisionCashSettlPaymentDateOffsetUnit.field
			return 40167
		end
		def initialize(data = nil)
			if( data == nil )
				super(40167)
			else
				super(40167, data)
			end
		end
	end

	class ProvisionCashSettlPaymentDateOffsetDayType < Quickfix::IntField
		def ProvisionCashSettlPaymentDateOffsetDayType.field
			return 40168
		end
		def initialize(data = nil)
			if( data == nil )
				super(40168)
			else
				super(40168, data)
			end
		end
	end

	class ProvisionCashSettlPaymentDateRangeFirst < Quickfix::StringField
		def ProvisionCashSettlPaymentDateRangeFirst.field
			return 40169
		end
		def initialize(data = nil)
			if( data == nil )
				super(40169)
			else
				super(40169, data)
			end
		end
	end

	class ProvisionCashSettlPaymentDateRangeLast < Quickfix::StringField
		def ProvisionCashSettlPaymentDateRangeLast.field
			return 40170
		end
		def initialize(data = nil)
			if( data == nil )
				super(40170)
			else
				super(40170, data)
			end
		end
	end

	class NoProvisionCashSettlPaymentDates < Quickfix::IntField
		def NoProvisionCashSettlPaymentDates.field
			return 40171
		end
		def initialize(data = nil)
			if( data == nil )
				super(40171)
			else
				super(40171, data)
			end
		end
	end

	class ProvisionCashSettlPaymentDate < Quickfix::StringField
		def ProvisionCashSettlPaymentDate.field
			return 40172
		end
		def initialize(data = nil)
			if( data == nil )
				super(40172)
			else
				super(40172, data)
			end
		end
	end

	class ProvisionCashSettlPaymentDateType < Quickfix::IntField
		def ProvisionCashSettlPaymentDateType.field
			return 40173
		end
		def initialize(data = nil)
			if( data == nil )
				super(40173)
			else
				super(40173, data)
			end
		end
	end

	class NoProvisionPartyIDs < Quickfix::IntField
		def NoProvisionPartyIDs.field
			return 40174
		end
		def initialize(data = nil)
			if( data == nil )
				super(40174)
			else
				super(40174, data)
			end
		end
	end

	class ProvisionPartyID < Quickfix::StringField
		def ProvisionPartyID.field
			return 40175
		end
		def initialize(data = nil)
			if( data == nil )
				super(40175)
			else
				super(40175, data)
			end
		end
	end

	class ProvisionPartyIDSource < Quickfix::CharField
		def ProvisionPartyIDSource.field
			return 40176
		end
		def initialize(data = nil)
			if( data == nil )
				super(40176)
			else
				super(40176, data)
			end
		end
	end

	class ProvisionPartyRole < Quickfix::IntField
		def ProvisionPartyRole.field
			return 40177
		end
		def initialize(data = nil)
			if( data == nil )
				super(40177)
			else
				super(40177, data)
			end
		end
	end

	class NoProvisionPartySubIDs < Quickfix::IntField
		def NoProvisionPartySubIDs.field
			return 40178
		end
		def initialize(data = nil)
			if( data == nil )
				super(40178)
			else
				super(40178, data)
			end
		end
	end

	class ProvisionPartySubID < Quickfix::StringField
		def ProvisionPartySubID.field
			return 40179
		end
		def initialize(data = nil)
			if( data == nil )
				super(40179)
			else
				super(40179, data)
			end
		end
	end

	class ProvisionPartySubIDType < Quickfix::IntField
		def ProvisionPartySubIDType.field
			return 40180
		end
		def initialize(data = nil)
			if( data == nil )
				super(40180)
			else
				super(40180, data)
			end
		end
	end

	class NoProtectionTerms < Quickfix::IntField
		def NoProtectionTerms.field
			return 40181
		end
		def initialize(data = nil)
			if( data == nil )
				super(40181)
			else
				super(40181, data)
			end
		end
	end

	class ProtectionTermNotional < Quickfix::DoubleField
		def ProtectionTermNotional.field
			return 40182
		end
		def initialize(data = nil)
			if( data == nil )
				super(40182)
			else
				super(40182, data)
			end
		end
	end

	class ProtectionTermCurrency < Quickfix::StringField
		def ProtectionTermCurrency.field
			return 40183
		end
		def initialize(data = nil)
			if( data == nil )
				super(40183)
			else
				super(40183, data)
			end
		end
	end

	class ProtectionTermSellerNotifies < Quickfix::BoolField
		def ProtectionTermSellerNotifies.field
			return 40184
		end
		def initialize(data = nil)
			if( data == nil )
				super(40184)
			else
				super(40184, data)
			end
		end
	end

	class ProtectionTermBuyerNotifies < Quickfix::BoolField
		def ProtectionTermBuyerNotifies.field
			return 40185
		end
		def initialize(data = nil)
			if( data == nil )
				super(40185)
			else
				super(40185, data)
			end
		end
	end

	class ProtectionTermEventBusinessCenter < Quickfix::StringField
		def ProtectionTermEventBusinessCenter.field
			return 40186
		end
		def initialize(data = nil)
			if( data == nil )
				super(40186)
			else
				super(40186, data)
			end
		end
	end

	class ProtectionTermStandardSources < Quickfix::BoolField
		def ProtectionTermStandardSources.field
			return 40187
		end
		def initialize(data = nil)
			if( data == nil )
				super(40187)
			else
				super(40187, data)
			end
		end
	end

	class ProtectionTermEventMinimumSources < Quickfix::IntField
		def ProtectionTermEventMinimumSources.field
			return 40188
		end
		def initialize(data = nil)
			if( data == nil )
				super(40188)
			else
				super(40188, data)
			end
		end
	end

	class ProtectionTermEventNewsSource < Quickfix::StringField
		def ProtectionTermEventNewsSource.field
			return 40189
		end
		def initialize(data = nil)
			if( data == nil )
				super(40189)
			else
				super(40189, data)
			end
		end
	end

	class ProtectionTermXID < Quickfix::StringField
		def ProtectionTermXID.field
			return 40190
		end
		def initialize(data = nil)
			if( data == nil )
				super(40190)
			else
				super(40190, data)
			end
		end
	end

	class NoProtectionTermEvents < Quickfix::IntField
		def NoProtectionTermEvents.field
			return 40191
		end
		def initialize(data = nil)
			if( data == nil )
				super(40191)
			else
				super(40191, data)
			end
		end
	end

	class ProtectionTermEventType < Quickfix::StringField
		def ProtectionTermEventType.field
			return 40192
		end
		def initialize(data = nil)
			if( data == nil )
				super(40192)
			else
				super(40192, data)
			end
		end
	end

	class ProtectionTermEventValue < Quickfix::StringField
		def ProtectionTermEventValue.field
			return 40193
		end
		def initialize(data = nil)
			if( data == nil )
				super(40193)
			else
				super(40193, data)
			end
		end
	end

	class ProtectionTermEventCurrency < Quickfix::StringField
		def ProtectionTermEventCurrency.field
			return 40194
		end
		def initialize(data = nil)
			if( data == nil )
				super(40194)
			else
				super(40194, data)
			end
		end
	end

	class ProtectionTermEventPeriod < Quickfix::IntField
		def ProtectionTermEventPeriod.field
			return 40195
		end
		def initialize(data = nil)
			if( data == nil )
				super(40195)
			else
				super(40195, data)
			end
		end
	end

	class ProtectionTermEventUnit < Quickfix::StringField
		def ProtectionTermEventUnit.field
			return 40196
		end
		def initialize(data = nil)
			if( data == nil )
				super(40196)
			else
				super(40196, data)
			end
		end
	end

	class ProtectionTermEventDayType < Quickfix::IntField
		def ProtectionTermEventDayType.field
			return 40197
		end
		def initialize(data = nil)
			if( data == nil )
				super(40197)
			else
				super(40197, data)
			end
		end
	end

	class ProtectionTermEventRateSource < Quickfix::StringField
		def ProtectionTermEventRateSource.field
			return 40198
		end
		def initialize(data = nil)
			if( data == nil )
				super(40198)
			else
				super(40198, data)
			end
		end
	end

	class NoProtectionTermEventQualifiers < Quickfix::IntField
		def NoProtectionTermEventQualifiers.field
			return 40199
		end
		def initialize(data = nil)
			if( data == nil )
				super(40199)
			else
				super(40199, data)
			end
		end
	end

	class ProtectionTermEventQualifier < Quickfix::CharField
		def ProtectionTermEventQualifier.field
			return 40200
		end
		def initialize(data = nil)
			if( data == nil )
				super(40200)
			else
				super(40200, data)
			end
		end
	end

	class NoProtectionTermObligations < Quickfix::IntField
		def NoProtectionTermObligations.field
			return 40201
		end
		def initialize(data = nil)
			if( data == nil )
				super(40201)
			else
				super(40201, data)
			end
		end
	end

	class ProtectionTermObligationType < Quickfix::StringField
		def ProtectionTermObligationType.field
			return 40202
		end
		def initialize(data = nil)
			if( data == nil )
				super(40202)
			else
				super(40202, data)
			end
		end
	end

	class ProtectionTermObligationValue < Quickfix::StringField
		def ProtectionTermObligationValue.field
			return 40203
		end
		def initialize(data = nil)
			if( data == nil )
				super(40203)
			else
				super(40203, data)
			end
		end
	end

	class NoPhysicalSettlTerms < Quickfix::IntField
		def NoPhysicalSettlTerms.field
			return 40204
		end
		def initialize(data = nil)
			if( data == nil )
				super(40204)
			else
				super(40204, data)
			end
		end
	end

	class PhysicalSettlCurrency < Quickfix::StringField
		def PhysicalSettlCurrency.field
			return 40205
		end
		def initialize(data = nil)
			if( data == nil )
				super(40205)
			else
				super(40205, data)
			end
		end
	end

	class PhysicalSettlBusinessDays < Quickfix::IntField
		def PhysicalSettlBusinessDays.field
			return 40206
		end
		def initialize(data = nil)
			if( data == nil )
				super(40206)
			else
				super(40206, data)
			end
		end
	end

	class PhysicalSettlMaximumBusinessDays < Quickfix::IntField
		def PhysicalSettlMaximumBusinessDays.field
			return 40207
		end
		def initialize(data = nil)
			if( data == nil )
				super(40207)
			else
				super(40207, data)
			end
		end
	end

	class PhysicalSettlTermXID < Quickfix::StringField
		def PhysicalSettlTermXID.field
			return 40208
		end
		def initialize(data = nil)
			if( data == nil )
				super(40208)
			else
				super(40208, data)
			end
		end
	end

	class NoPhysicalSettlDeliverableObligations < Quickfix::IntField
		def NoPhysicalSettlDeliverableObligations.field
			return 40209
		end
		def initialize(data = nil)
			if( data == nil )
				super(40209)
			else
				super(40209, data)
			end
		end
	end

	class PhysicalSettlDeliverableObligationType < Quickfix::StringField
		def PhysicalSettlDeliverableObligationType.field
			return 40210
		end
		def initialize(data = nil)
			if( data == nil )
				super(40210)
			else
				super(40210, data)
			end
		end
	end

	class PhysicalSettlDeliverableObligationValue < Quickfix::StringField
		def PhysicalSettlDeliverableObligationValue.field
			return 40211
		end
		def initialize(data = nil)
			if( data == nil )
				super(40211)
			else
				super(40211, data)
			end
		end
	end

	class NoPayments < Quickfix::IntField
		def NoPayments.field
			return 40212
		end
		def initialize(data = nil)
			if( data == nil )
				super(40212)
			else
				super(40212, data)
			end
		end
	end

	class PaymentType < Quickfix::IntField
		def PaymentType.field
			return 40213
		end
		def initialize(data = nil)
			if( data == nil )
				super(40213)
			else
				super(40213, data)
			end
		end
	end

	class PaymentPaySide < Quickfix::IntField
		def PaymentPaySide.field
			return 40214
		end
		def initialize(data = nil)
			if( data == nil )
				super(40214)
			else
				super(40214, data)
			end
		end
	end

	class PaymentReceiveSide < Quickfix::IntField
		def PaymentReceiveSide.field
			return 40215
		end
		def initialize(data = nil)
			if( data == nil )
				super(40215)
			else
				super(40215, data)
			end
		end
	end

	class PaymentCurrency < Quickfix::StringField
		def PaymentCurrency.field
			return 40216
		end
		def initialize(data = nil)
			if( data == nil )
				super(40216)
			else
				super(40216, data)
			end
		end
	end

	class PaymentAmount < Quickfix::DoubleField
		def PaymentAmount.field
			return 40217
		end
		def initialize(data = nil)
			if( data == nil )
				super(40217)
			else
				super(40217, data)
			end
		end
	end

	class PaymentPrice < Quickfix::DoubleField
		def PaymentPrice.field
			return 40218
		end
		def initialize(data = nil)
			if( data == nil )
				super(40218)
			else
				super(40218, data)
			end
		end
	end

	class PaymentDateUnadjusted < Quickfix::StringField
		def PaymentDateUnadjusted.field
			return 40219
		end
		def initialize(data = nil)
			if( data == nil )
				super(40219)
			else
				super(40219, data)
			end
		end
	end

	class PaymentBusinessDayConvention < Quickfix::IntField
		def PaymentBusinessDayConvention.field
			return 40220
		end
		def initialize(data = nil)
			if( data == nil )
				super(40220)
			else
				super(40220, data)
			end
		end
	end

	class PaymentBusinessCenter < Quickfix::StringField
		def PaymentBusinessCenter.field
			return 40221
		end
		def initialize(data = nil)
			if( data == nil )
				super(40221)
			else
				super(40221, data)
			end
		end
	end

	class PaymentDateAdjusted < Quickfix::StringField
		def PaymentDateAdjusted.field
			return 40222
		end
		def initialize(data = nil)
			if( data == nil )
				super(40222)
			else
				super(40222, data)
			end
		end
	end

	class PaymentDiscountFactor < Quickfix::DoubleField
		def PaymentDiscountFactor.field
			return 40224
		end
		def initialize(data = nil)
			if( data == nil )
				super(40224)
			else
				super(40224, data)
			end
		end
	end

	class PaymentPresentValueAmount < Quickfix::DoubleField
		def PaymentPresentValueAmount.field
			return 40225
		end
		def initialize(data = nil)
			if( data == nil )
				super(40225)
			else
				super(40225, data)
			end
		end
	end

	class PaymentPresentValueCurrency < Quickfix::StringField
		def PaymentPresentValueCurrency.field
			return 40226
		end
		def initialize(data = nil)
			if( data == nil )
				super(40226)
			else
				super(40226, data)
			end
		end
	end

	class PaymentSettlStyle < Quickfix::IntField
		def PaymentSettlStyle.field
			return 40227
		end
		def initialize(data = nil)
			if( data == nil )
				super(40227)
			else
				super(40227, data)
			end
		end
	end

	class LegPaymentStreamNonDeliverableSettlReferencePage < Quickfix::StringField
		def LegPaymentStreamNonDeliverableSettlReferencePage.field
			return 40228
		end
		def initialize(data = nil)
			if( data == nil )
				super(40228)
			else
				super(40228, data)
			end
		end
	end

	class PaymentText < Quickfix::StringField
		def PaymentText.field
			return 40229
		end
		def initialize(data = nil)
			if( data == nil )
				super(40229)
			else
				super(40229, data)
			end
		end
	end

	class NoPaymentSettls < Quickfix::IntField
		def NoPaymentSettls.field
			return 40230
		end
		def initialize(data = nil)
			if( data == nil )
				super(40230)
			else
				super(40230, data)
			end
		end
	end

	class PaymentSettlAmount < Quickfix::DoubleField
		def PaymentSettlAmount.field
			return 40231
		end
		def initialize(data = nil)
			if( data == nil )
				super(40231)
			else
				super(40231, data)
			end
		end
	end

	class PaymentSettlCurrency < Quickfix::StringField
		def PaymentSettlCurrency.field
			return 40232
		end
		def initialize(data = nil)
			if( data == nil )
				super(40232)
			else
				super(40232, data)
			end
		end
	end

	class NoPaymentSettlPartyIDs < Quickfix::IntField
		def NoPaymentSettlPartyIDs.field
			return 40233
		end
		def initialize(data = nil)
			if( data == nil )
				super(40233)
			else
				super(40233, data)
			end
		end
	end

	class PaymentSettlPartyID < Quickfix::StringField
		def PaymentSettlPartyID.field
			return 40234
		end
		def initialize(data = nil)
			if( data == nil )
				super(40234)
			else
				super(40234, data)
			end
		end
	end

	class PaymentSettlPartyIDSource < Quickfix::CharField
		def PaymentSettlPartyIDSource.field
			return 40235
		end
		def initialize(data = nil)
			if( data == nil )
				super(40235)
			else
				super(40235, data)
			end
		end
	end

	class PaymentSettlPartyRole < Quickfix::IntField
		def PaymentSettlPartyRole.field
			return 40236
		end
		def initialize(data = nil)
			if( data == nil )
				super(40236)
			else
				super(40236, data)
			end
		end
	end

	class PaymentSettlPartyRoleQualifier < Quickfix::IntField
		def PaymentSettlPartyRoleQualifier.field
			return 40237
		end
		def initialize(data = nil)
			if( data == nil )
				super(40237)
			else
				super(40237, data)
			end
		end
	end

	class NoPaymentSettlPartySubIDs < Quickfix::IntField
		def NoPaymentSettlPartySubIDs.field
			return 40238
		end
		def initialize(data = nil)
			if( data == nil )
				super(40238)
			else
				super(40238, data)
			end
		end
	end

	class PaymentSettlPartySubID < Quickfix::StringField
		def PaymentSettlPartySubID.field
			return 40239
		end
		def initialize(data = nil)
			if( data == nil )
				super(40239)
			else
				super(40239, data)
			end
		end
	end

	class PaymentSettlPartySubIDType < Quickfix::IntField
		def PaymentSettlPartySubIDType.field
			return 40240
		end
		def initialize(data = nil)
			if( data == nil )
				super(40240)
			else
				super(40240, data)
			end
		end
	end

	class NoLegStreams < Quickfix::IntField
		def NoLegStreams.field
			return 40241
		end
		def initialize(data = nil)
			if( data == nil )
				super(40241)
			else
				super(40241, data)
			end
		end
	end

	class LegStreamType < Quickfix::IntField
		def LegStreamType.field
			return 40242
		end
		def initialize(data = nil)
			if( data == nil )
				super(40242)
			else
				super(40242, data)
			end
		end
	end

	class LegStreamDesc < Quickfix::StringField
		def LegStreamDesc.field
			return 40243
		end
		def initialize(data = nil)
			if( data == nil )
				super(40243)
			else
				super(40243, data)
			end
		end
	end

	class LegStreamPaySide < Quickfix::IntField
		def LegStreamPaySide.field
			return 40244
		end
		def initialize(data = nil)
			if( data == nil )
				super(40244)
			else
				super(40244, data)
			end
		end
	end

	class LegStreamReceiveSide < Quickfix::IntField
		def LegStreamReceiveSide.field
			return 40245
		end
		def initialize(data = nil)
			if( data == nil )
				super(40245)
			else
				super(40245, data)
			end
		end
	end

	class LegStreamNotional < Quickfix::DoubleField
		def LegStreamNotional.field
			return 40246
		end
		def initialize(data = nil)
			if( data == nil )
				super(40246)
			else
				super(40246, data)
			end
		end
	end

	class LegStreamCurrency < Quickfix::StringField
		def LegStreamCurrency.field
			return 40247
		end
		def initialize(data = nil)
			if( data == nil )
				super(40247)
			else
				super(40247, data)
			end
		end
	end

	class LegStreamText < Quickfix::StringField
		def LegStreamText.field
			return 40248
		end
		def initialize(data = nil)
			if( data == nil )
				super(40248)
			else
				super(40248, data)
			end
		end
	end

	class LegStreamEffectiveDateUnadjusted < Quickfix::StringField
		def LegStreamEffectiveDateUnadjusted.field
			return 40249
		end
		def initialize(data = nil)
			if( data == nil )
				super(40249)
			else
				super(40249, data)
			end
		end
	end

	class LegStreamEffectiveDateBusinessDayConvention < Quickfix::IntField
		def LegStreamEffectiveDateBusinessDayConvention.field
			return 40250
		end
		def initialize(data = nil)
			if( data == nil )
				super(40250)
			else
				super(40250, data)
			end
		end
	end

	class LegStreamEffectiveDateBusinessCenter < Quickfix::StringField
		def LegStreamEffectiveDateBusinessCenter.field
			return 40251
		end
		def initialize(data = nil)
			if( data == nil )
				super(40251)
			else
				super(40251, data)
			end
		end
	end

	class LegStreamEffectiveDateRelativeTo < Quickfix::IntField
		def LegStreamEffectiveDateRelativeTo.field
			return 40252
		end
		def initialize(data = nil)
			if( data == nil )
				super(40252)
			else
				super(40252, data)
			end
		end
	end

	class LegStreamEffectiveDateOffsetPeriod < Quickfix::IntField
		def LegStreamEffectiveDateOffsetPeriod.field
			return 40253
		end
		def initialize(data = nil)
			if( data == nil )
				super(40253)
			else
				super(40253, data)
			end
		end
	end

	class LegStreamEffectiveDateOffsetUnit < Quickfix::StringField
		def LegStreamEffectiveDateOffsetUnit.field
			return 40254
		end
		def initialize(data = nil)
			if( data == nil )
				super(40254)
			else
				super(40254, data)
			end
		end
	end

	class LegStreamEffectiveDateOffsetDayType < Quickfix::IntField
		def LegStreamEffectiveDateOffsetDayType.field
			return 40255
		end
		def initialize(data = nil)
			if( data == nil )
				super(40255)
			else
				super(40255, data)
			end
		end
	end

	class LegStreamEffectiveDateAdjusted < Quickfix::StringField
		def LegStreamEffectiveDateAdjusted.field
			return 40256
		end
		def initialize(data = nil)
			if( data == nil )
				super(40256)
			else
				super(40256, data)
			end
		end
	end

	class LegStreamTerminationDateUnadjusted < Quickfix::StringField
		def LegStreamTerminationDateUnadjusted.field
			return 40257
		end
		def initialize(data = nil)
			if( data == nil )
				super(40257)
			else
				super(40257, data)
			end
		end
	end

	class LegStreamTerminationDateBusinessDayConvention < Quickfix::IntField
		def LegStreamTerminationDateBusinessDayConvention.field
			return 40258
		end
		def initialize(data = nil)
			if( data == nil )
				super(40258)
			else
				super(40258, data)
			end
		end
	end

	class LegStreamTerminationDateBusinessCenter < Quickfix::StringField
		def LegStreamTerminationDateBusinessCenter.field
			return 40259
		end
		def initialize(data = nil)
			if( data == nil )
				super(40259)
			else
				super(40259, data)
			end
		end
	end

	class LegStreamTerminationDateRelativeTo < Quickfix::IntField
		def LegStreamTerminationDateRelativeTo.field
			return 40260
		end
		def initialize(data = nil)
			if( data == nil )
				super(40260)
			else
				super(40260, data)
			end
		end
	end

	class LegStreamTerminationDateOffsetPeriod < Quickfix::IntField
		def LegStreamTerminationDateOffsetPeriod.field
			return 40261
		end
		def initialize(data = nil)
			if( data == nil )
				super(40261)
			else
				super(40261, data)
			end
		end
	end

	class LegStreamTerminationDateOffsetUnit < Quickfix::StringField
		def LegStreamTerminationDateOffsetUnit.field
			return 40262
		end
		def initialize(data = nil)
			if( data == nil )
				super(40262)
			else
				super(40262, data)
			end
		end
	end

	class LegStreamTerminationDateOffsetDayType < Quickfix::IntField
		def LegStreamTerminationDateOffsetDayType.field
			return 40263
		end
		def initialize(data = nil)
			if( data == nil )
				super(40263)
			else
				super(40263, data)
			end
		end
	end

	class LegStreamTerminationDateAdjusted < Quickfix::StringField
		def LegStreamTerminationDateAdjusted.field
			return 40264
		end
		def initialize(data = nil)
			if( data == nil )
				super(40264)
			else
				super(40264, data)
			end
		end
	end

	class LegStreamCalculationPeriodBusinessDayConvention < Quickfix::IntField
		def LegStreamCalculationPeriodBusinessDayConvention.field
			return 40265
		end
		def initialize(data = nil)
			if( data == nil )
				super(40265)
			else
				super(40265, data)
			end
		end
	end

	class LegStreamCalculationPeriodBusinessCenter < Quickfix::StringField
		def LegStreamCalculationPeriodBusinessCenter.field
			return 40266
		end
		def initialize(data = nil)
			if( data == nil )
				super(40266)
			else
				super(40266, data)
			end
		end
	end

	class LegStreamFirstPeriodStartDateUnadjusted < Quickfix::StringField
		def LegStreamFirstPeriodStartDateUnadjusted.field
			return 40267
		end
		def initialize(data = nil)
			if( data == nil )
				super(40267)
			else
				super(40267, data)
			end
		end
	end

	class LegStreamFirstPeriodStartDateBusinessDayConvention < Quickfix::IntField
		def LegStreamFirstPeriodStartDateBusinessDayConvention.field
			return 40268
		end
		def initialize(data = nil)
			if( data == nil )
				super(40268)
			else
				super(40268, data)
			end
		end
	end

	class LegStreamFirstPeriodStartDateBusinessCenter < Quickfix::StringField
		def LegStreamFirstPeriodStartDateBusinessCenter.field
			return 40269
		end
		def initialize(data = nil)
			if( data == nil )
				super(40269)
			else
				super(40269, data)
			end
		end
	end

	class LegStreamFirstPeriodStartDateAdjusted < Quickfix::StringField
		def LegStreamFirstPeriodStartDateAdjusted.field
			return 40270
		end
		def initialize(data = nil)
			if( data == nil )
				super(40270)
			else
				super(40270, data)
			end
		end
	end

	class LegStreamFirstRegularPeriodStartDateUnadjusted < Quickfix::StringField
		def LegStreamFirstRegularPeriodStartDateUnadjusted.field
			return 40271
		end
		def initialize(data = nil)
			if( data == nil )
				super(40271)
			else
				super(40271, data)
			end
		end
	end

	class LegStreamFirstCompoundingPeriodEndDateUnadjusted < Quickfix::StringField
		def LegStreamFirstCompoundingPeriodEndDateUnadjusted.field
			return 40272
		end
		def initialize(data = nil)
			if( data == nil )
				super(40272)
			else
				super(40272, data)
			end
		end
	end

	class LegStreamLastRegularPeriodEndDateUnadjusted < Quickfix::StringField
		def LegStreamLastRegularPeriodEndDateUnadjusted.field
			return 40273
		end
		def initialize(data = nil)
			if( data == nil )
				super(40273)
			else
				super(40273, data)
			end
		end
	end

	class LegStreamCalculationFrequencyPeriod < Quickfix::IntField
		def LegStreamCalculationFrequencyPeriod.field
			return 40274
		end
		def initialize(data = nil)
			if( data == nil )
				super(40274)
			else
				super(40274, data)
			end
		end
	end

	class LegStreamCalculationFrequencyUnit < Quickfix::StringField
		def LegStreamCalculationFrequencyUnit.field
			return 40275
		end
		def initialize(data = nil)
			if( data == nil )
				super(40275)
			else
				super(40275, data)
			end
		end
	end

	class LegStreamCalculationRollConvention < Quickfix::StringField
		def LegStreamCalculationRollConvention.field
			return 40276
		end
		def initialize(data = nil)
			if( data == nil )
				super(40276)
			else
				super(40276, data)
			end
		end
	end

	class NoCashSettlDealers < Quickfix::IntField
		def NoCashSettlDealers.field
			return 40277
		end
		def initialize(data = nil)
			if( data == nil )
				super(40277)
			else
				super(40277, data)
			end
		end
	end

	class NoBusinessCenters < Quickfix::IntField
		def NoBusinessCenters.field
			return 40278
		end
		def initialize(data = nil)
			if( data == nil )
				super(40278)
			else
				super(40278, data)
			end
		end
	end

	class LegPaymentStreamType < Quickfix::IntField
		def LegPaymentStreamType.field
			return 40279
		end
		def initialize(data = nil)
			if( data == nil )
				super(40279)
			else
				super(40279, data)
			end
		end
	end

	class LegPaymentStreamMarketRate < Quickfix::IntField
		def LegPaymentStreamMarketRate.field
			return 40280
		end
		def initialize(data = nil)
			if( data == nil )
				super(40280)
			else
				super(40280, data)
			end
		end
	end

	class LegPaymentStreamDelayIndicator < Quickfix::BoolField
		def LegPaymentStreamDelayIndicator.field
			return 40281
		end
		def initialize(data = nil)
			if( data == nil )
				super(40281)
			else
				super(40281, data)
			end
		end
	end

	class LegPaymentStreamSettlCurrency < Quickfix::StringField
		def LegPaymentStreamSettlCurrency.field
			return 40282
		end
		def initialize(data = nil)
			if( data == nil )
				super(40282)
			else
				super(40282, data)
			end
		end
	end

	class LegPaymentStreamDayCount < Quickfix::IntField
		def LegPaymentStreamDayCount.field
			return 40283
		end
		def initialize(data = nil)
			if( data == nil )
				super(40283)
			else
				super(40283, data)
			end
		end
	end

	class LegPaymentStreamAccrualDays < Quickfix::IntField
		def LegPaymentStreamAccrualDays.field
			return 40284
		end
		def initialize(data = nil)
			if( data == nil )
				super(40284)
			else
				super(40284, data)
			end
		end
	end

	class LegPaymentStreamDiscountType < Quickfix::IntField
		def LegPaymentStreamDiscountType.field
			return 40285
		end
		def initialize(data = nil)
			if( data == nil )
				super(40285)
			else
				super(40285, data)
			end
		end
	end

	class LegPaymentStreamDiscountRate < Quickfix::DoubleField
		def LegPaymentStreamDiscountRate.field
			return 40286
		end
		def initialize(data = nil)
			if( data == nil )
				super(40286)
			else
				super(40286, data)
			end
		end
	end

	class LegPaymentStreamDiscountRateDayCount < Quickfix::IntField
		def LegPaymentStreamDiscountRateDayCount.field
			return 40287
		end
		def initialize(data = nil)
			if( data == nil )
				super(40287)
			else
				super(40287, data)
			end
		end
	end

	class LegPaymentStreamCompoundingMethod < Quickfix::IntField
		def LegPaymentStreamCompoundingMethod.field
			return 40288
		end
		def initialize(data = nil)
			if( data == nil )
				super(40288)
			else
				super(40288, data)
			end
		end
	end

	class LegPaymentStreamInitialPrincipalExchangeIndicator < Quickfix::BoolField
		def LegPaymentStreamInitialPrincipalExchangeIndicator.field
			return 40289
		end
		def initialize(data = nil)
			if( data == nil )
				super(40289)
			else
				super(40289, data)
			end
		end
	end

	class LegPaymentStreamInterimPrincipalExchangeIndicator < Quickfix::BoolField
		def LegPaymentStreamInterimPrincipalExchangeIndicator.field
			return 40290
		end
		def initialize(data = nil)
			if( data == nil )
				super(40290)
			else
				super(40290, data)
			end
		end
	end

	class LegPaymentStreamFinalPrincipalExchangeIndicator < Quickfix::BoolField
		def LegPaymentStreamFinalPrincipalExchangeIndicator.field
			return 40291
		end
		def initialize(data = nil)
			if( data == nil )
				super(40291)
			else
				super(40291, data)
			end
		end
	end

	class LegPaymentStreamPaymentDateBusinessDayConvention < Quickfix::IntField
		def LegPaymentStreamPaymentDateBusinessDayConvention.field
			return 40292
		end
		def initialize(data = nil)
			if( data == nil )
				super(40292)
			else
				super(40292, data)
			end
		end
	end

	class LegPaymentStreamPaymentDateBusinessCenter < Quickfix::StringField
		def LegPaymentStreamPaymentDateBusinessCenter.field
			return 40293
		end
		def initialize(data = nil)
			if( data == nil )
				super(40293)
			else
				super(40293, data)
			end
		end
	end

	class LegPaymentStreamPaymentFrequencyPeriod < Quickfix::IntField
		def LegPaymentStreamPaymentFrequencyPeriod.field
			return 40294
		end
		def initialize(data = nil)
			if( data == nil )
				super(40294)
			else
				super(40294, data)
			end
		end
	end

	class LegPaymentStreamPaymentFrequencyUnit < Quickfix::StringField
		def LegPaymentStreamPaymentFrequencyUnit.field
			return 40295
		end
		def initialize(data = nil)
			if( data == nil )
				super(40295)
			else
				super(40295, data)
			end
		end
	end

	class LegPaymentStreamPaymentRollConvention < Quickfix::StringField
		def LegPaymentStreamPaymentRollConvention.field
			return 40296
		end
		def initialize(data = nil)
			if( data == nil )
				super(40296)
			else
				super(40296, data)
			end
		end
	end

	class LegPaymentStreamFirstPaymentDateUnadjusted < Quickfix::StringField
		def LegPaymentStreamFirstPaymentDateUnadjusted.field
			return 40297
		end
		def initialize(data = nil)
			if( data == nil )
				super(40297)
			else
				super(40297, data)
			end
		end
	end

	class LegPaymentStreamLastRegularPaymentDateUnadjusted < Quickfix::StringField
		def LegPaymentStreamLastRegularPaymentDateUnadjusted.field
			return 40298
		end
		def initialize(data = nil)
			if( data == nil )
				super(40298)
			else
				super(40298, data)
			end
		end
	end

	class LegPaymentStreamPaymentDateRelativeTo < Quickfix::IntField
		def LegPaymentStreamPaymentDateRelativeTo.field
			return 40299
		end
		def initialize(data = nil)
			if( data == nil )
				super(40299)
			else
				super(40299, data)
			end
		end
	end

	class LegPaymentStreamPaymentDateOffsetPeriod < Quickfix::IntField
		def LegPaymentStreamPaymentDateOffsetPeriod.field
			return 40300
		end
		def initialize(data = nil)
			if( data == nil )
				super(40300)
			else
				super(40300, data)
			end
		end
	end

	class LegPaymentStreamPaymentDateOffsetUnit < Quickfix::StringField
		def LegPaymentStreamPaymentDateOffsetUnit.field
			return 40301
		end
		def initialize(data = nil)
			if( data == nil )
				super(40301)
			else
				super(40301, data)
			end
		end
	end

	class LegPaymentStreamPaymentDateOffsetDayType < Quickfix::IntField
		def LegPaymentStreamPaymentDateOffsetDayType.field
			return 40302
		end
		def initialize(data = nil)
			if( data == nil )
				super(40302)
			else
				super(40302, data)
			end
		end
	end

	class LegPaymentStreamResetDateRelativeTo < Quickfix::IntField
		def LegPaymentStreamResetDateRelativeTo.field
			return 40303
		end
		def initialize(data = nil)
			if( data == nil )
				super(40303)
			else
				super(40303, data)
			end
		end
	end

	class LegPaymentStreamResetDateBusinessDayConvention < Quickfix::IntField
		def LegPaymentStreamResetDateBusinessDayConvention.field
			return 40304
		end
		def initialize(data = nil)
			if( data == nil )
				super(40304)
			else
				super(40304, data)
			end
		end
	end

	class LegPaymentStreamResetDateBusinessCenter < Quickfix::StringField
		def LegPaymentStreamResetDateBusinessCenter.field
			return 40305
		end
		def initialize(data = nil)
			if( data == nil )
				super(40305)
			else
				super(40305, data)
			end
		end
	end

	class LegPaymentStreamResetFrequencyPeriod < Quickfix::IntField
		def LegPaymentStreamResetFrequencyPeriod.field
			return 40306
		end
		def initialize(data = nil)
			if( data == nil )
				super(40306)
			else
				super(40306, data)
			end
		end
	end

	class LegPaymentStreamResetFrequencyUnit < Quickfix::StringField
		def LegPaymentStreamResetFrequencyUnit.field
			return 40307
		end
		def initialize(data = nil)
			if( data == nil )
				super(40307)
			else
				super(40307, data)
			end
		end
	end

	class LegPaymentStreamResetWeeklyRollConvention < Quickfix::StringField
		def LegPaymentStreamResetWeeklyRollConvention.field
			return 40308
		end
		def initialize(data = nil)
			if( data == nil )
				super(40308)
			else
				super(40308, data)
			end
		end
	end

	class LegPaymentStreamInitialFixingDateRelativeTo < Quickfix::IntField
		def LegPaymentStreamInitialFixingDateRelativeTo.field
			return 40309
		end
		def initialize(data = nil)
			if( data == nil )
				super(40309)
			else
				super(40309, data)
			end
		end
	end

	class LegPaymentStreamInitialFixingDateBusinessDayConvention < Quickfix::IntField
		def LegPaymentStreamInitialFixingDateBusinessDayConvention.field
			return 40310
		end
		def initialize(data = nil)
			if( data == nil )
				super(40310)
			else
				super(40310, data)
			end
		end
	end

	class LegPaymentStreamInitialFixingDateBusinessCenter < Quickfix::StringField
		def LegPaymentStreamInitialFixingDateBusinessCenter.field
			return 40311
		end
		def initialize(data = nil)
			if( data == nil )
				super(40311)
			else
				super(40311, data)
			end
		end
	end

	class LegPaymentStreamInitialFixingDateOffsetPeriod < Quickfix::IntField
		def LegPaymentStreamInitialFixingDateOffsetPeriod.field
			return 40312
		end
		def initialize(data = nil)
			if( data == nil )
				super(40312)
			else
				super(40312, data)
			end
		end
	end

	class LegPaymentStreamInitialFixingDateOffsetUnit < Quickfix::StringField
		def LegPaymentStreamInitialFixingDateOffsetUnit.field
			return 40313
		end
		def initialize(data = nil)
			if( data == nil )
				super(40313)
			else
				super(40313, data)
			end
		end
	end

	class LegPaymentStreamInitialFixingDateOffsetDayType < Quickfix::IntField
		def LegPaymentStreamInitialFixingDateOffsetDayType.field
			return 40314
		end
		def initialize(data = nil)
			if( data == nil )
				super(40314)
			else
				super(40314, data)
			end
		end
	end

	class LegPaymentStreamInitialFixingDateAdjusted < Quickfix::StringField
		def LegPaymentStreamInitialFixingDateAdjusted.field
			return 40315
		end
		def initialize(data = nil)
			if( data == nil )
				super(40315)
			else
				super(40315, data)
			end
		end
	end

	class LegPaymentStreamFixingDateRelativeTo < Quickfix::IntField
		def LegPaymentStreamFixingDateRelativeTo.field
			return 40316
		end
		def initialize(data = nil)
			if( data == nil )
				super(40316)
			else
				super(40316, data)
			end
		end
	end

	class LegPaymentStreamFixingDateBusinessDayConvention < Quickfix::IntField
		def LegPaymentStreamFixingDateBusinessDayConvention.field
			return 40317
		end
		def initialize(data = nil)
			if( data == nil )
				super(40317)
			else
				super(40317, data)
			end
		end
	end

	class LegPaymentStreamFixingDateBusinessCenter < Quickfix::StringField
		def LegPaymentStreamFixingDateBusinessCenter.field
			return 40318
		end
		def initialize(data = nil)
			if( data == nil )
				super(40318)
			else
				super(40318, data)
			end
		end
	end

	class LegPaymentStreamFixingDateOffsetPeriod < Quickfix::IntField
		def LegPaymentStreamFixingDateOffsetPeriod.field
			return 40319
		end
		def initialize(data = nil)
			if( data == nil )
				super(40319)
			else
				super(40319, data)
			end
		end
	end

	class LegPaymentStreamFixingDateOffsetUnit < Quickfix::StringField
		def LegPaymentStreamFixingDateOffsetUnit.field
			return 40320
		end
		def initialize(data = nil)
			if( data == nil )
				super(40320)
			else
				super(40320, data)
			end
		end
	end

	class LegPaymentStreamFixingDateOffsetDayType < Quickfix::IntField
		def LegPaymentStreamFixingDateOffsetDayType.field
			return 40321
		end
		def initialize(data = nil)
			if( data == nil )
				super(40321)
			else
				super(40321, data)
			end
		end
	end

	class LegPaymentStreamFixingDateAdjusted < Quickfix::StringField
		def LegPaymentStreamFixingDateAdjusted.field
			return 40322
		end
		def initialize(data = nil)
			if( data == nil )
				super(40322)
			else
				super(40322, data)
			end
		end
	end

	class LegPaymentStreamRateCutoffDateOffsetPeriod < Quickfix::IntField
		def LegPaymentStreamRateCutoffDateOffsetPeriod.field
			return 40323
		end
		def initialize(data = nil)
			if( data == nil )
				super(40323)
			else
				super(40323, data)
			end
		end
	end

	class LegPaymentStreamRateCutoffDateOffsetUnit < Quickfix::StringField
		def LegPaymentStreamRateCutoffDateOffsetUnit.field
			return 40324
		end
		def initialize(data = nil)
			if( data == nil )
				super(40324)
			else
				super(40324, data)
			end
		end
	end

	class LegPaymentStreamRateCutoffDateOffsetDayType < Quickfix::IntField
		def LegPaymentStreamRateCutoffDateOffsetDayType.field
			return 40325
		end
		def initialize(data = nil)
			if( data == nil )
				super(40325)
			else
				super(40325, data)
			end
		end
	end

	class LegPaymentStreamRate < Quickfix::DoubleField
		def LegPaymentStreamRate.field
			return 40326
		end
		def initialize(data = nil)
			if( data == nil )
				super(40326)
			else
				super(40326, data)
			end
		end
	end

	class LegPaymentStreamFixedAmount < Quickfix::DoubleField
		def LegPaymentStreamFixedAmount.field
			return 40327
		end
		def initialize(data = nil)
			if( data == nil )
				super(40327)
			else
				super(40327, data)
			end
		end
	end

	class LegPaymentStreamRateOrAmountCurrency < Quickfix::StringField
		def LegPaymentStreamRateOrAmountCurrency.field
			return 40328
		end
		def initialize(data = nil)
			if( data == nil )
				super(40328)
			else
				super(40328, data)
			end
		end
	end

	class LegPaymentStreamFutureValueNotional < Quickfix::DoubleField
		def LegPaymentStreamFutureValueNotional.field
			return 40329
		end
		def initialize(data = nil)
			if( data == nil )
				super(40329)
			else
				super(40329, data)
			end
		end
	end

	class LegPaymentStreamFutureValueDateAdjusted < Quickfix::StringField
		def LegPaymentStreamFutureValueDateAdjusted.field
			return 40330
		end
		def initialize(data = nil)
			if( data == nil )
				super(40330)
			else
				super(40330, data)
			end
		end
	end

	class LegPaymentStreamRateIndex < Quickfix::StringField
		def LegPaymentStreamRateIndex.field
			return 40331
		end
		def initialize(data = nil)
			if( data == nil )
				super(40331)
			else
				super(40331, data)
			end
		end
	end

	class LegPaymentStreamRateIndexSource < Quickfix::IntField
		def LegPaymentStreamRateIndexSource.field
			return 40332
		end
		def initialize(data = nil)
			if( data == nil )
				super(40332)
			else
				super(40332, data)
			end
		end
	end

	class LegPaymentStreamRateIndexCurveUnit < Quickfix::StringField
		def LegPaymentStreamRateIndexCurveUnit.field
			return 40333
		end
		def initialize(data = nil)
			if( data == nil )
				super(40333)
			else
				super(40333, data)
			end
		end
	end

	class LegPaymentStreamRateIndexCurvePeriod < Quickfix::IntField
		def LegPaymentStreamRateIndexCurvePeriod.field
			return 40334
		end
		def initialize(data = nil)
			if( data == nil )
				super(40334)
			else
				super(40334, data)
			end
		end
	end

	class LegPaymentStreamRateMultiplier < Quickfix::DoubleField
		def LegPaymentStreamRateMultiplier.field
			return 40335
		end
		def initialize(data = nil)
			if( data == nil )
				super(40335)
			else
				super(40335, data)
			end
		end
	end

	class LegPaymentStreamRateSpread < Quickfix::DoubleField
		def LegPaymentStreamRateSpread.field
			return 40336
		end
		def initialize(data = nil)
			if( data == nil )
				super(40336)
			else
				super(40336, data)
			end
		end
	end

	class LegPaymentStreamRateSpreadPositionType < Quickfix::IntField
		def LegPaymentStreamRateSpreadPositionType.field
			return 40337
		end
		def initialize(data = nil)
			if( data == nil )
				super(40337)
			else
				super(40337, data)
			end
		end
	end

	class LegPaymentStreamRateTreatment < Quickfix::IntField
		def LegPaymentStreamRateTreatment.field
			return 40338
		end
		def initialize(data = nil)
			if( data == nil )
				super(40338)
			else
				super(40338, data)
			end
		end
	end

	class LegPaymentStreamCapRate < Quickfix::DoubleField
		def LegPaymentStreamCapRate.field
			return 40339
		end
		def initialize(data = nil)
			if( data == nil )
				super(40339)
			else
				super(40339, data)
			end
		end
	end

	class LegPaymentStreamCapRateBuySide < Quickfix::IntField
		def LegPaymentStreamCapRateBuySide.field
			return 40340
		end
		def initialize(data = nil)
			if( data == nil )
				super(40340)
			else
				super(40340, data)
			end
		end
	end

	class LegPaymentStreamCapRateSellSide < Quickfix::IntField
		def LegPaymentStreamCapRateSellSide.field
			return 40341
		end
		def initialize(data = nil)
			if( data == nil )
				super(40341)
			else
				super(40341, data)
			end
		end
	end

	class LegPaymentStreamFloorRate < Quickfix::DoubleField
		def LegPaymentStreamFloorRate.field
			return 40342
		end
		def initialize(data = nil)
			if( data == nil )
				super(40342)
			else
				super(40342, data)
			end
		end
	end

	class LegPaymentStreamFloorRateBuySide < Quickfix::IntField
		def LegPaymentStreamFloorRateBuySide.field
			return 40343
		end
		def initialize(data = nil)
			if( data == nil )
				super(40343)
			else
				super(40343, data)
			end
		end
	end

	class LegPaymentStreamFloorRateSellSide < Quickfix::IntField
		def LegPaymentStreamFloorRateSellSide.field
			return 40344
		end
		def initialize(data = nil)
			if( data == nil )
				super(40344)
			else
				super(40344, data)
			end
		end
	end

	class LegPaymentStreamInitialRate < Quickfix::DoubleField
		def LegPaymentStreamInitialRate.field
			return 40345
		end
		def initialize(data = nil)
			if( data == nil )
				super(40345)
			else
				super(40345, data)
			end
		end
	end

	class LegPaymentStreamFinalRateRoundingDirection < Quickfix::CharField
		def LegPaymentStreamFinalRateRoundingDirection.field
			return 40346
		end
		def initialize(data = nil)
			if( data == nil )
				super(40346)
			else
				super(40346, data)
			end
		end
	end

	class LegPaymentStreamFinalRatePrecision < Quickfix::IntField
		def LegPaymentStreamFinalRatePrecision.field
			return 40347
		end
		def initialize(data = nil)
			if( data == nil )
				super(40347)
			else
				super(40347, data)
			end
		end
	end

	class LegPaymentStreamAveragingMethod < Quickfix::IntField
		def LegPaymentStreamAveragingMethod.field
			return 40348
		end
		def initialize(data = nil)
			if( data == nil )
				super(40348)
			else
				super(40348, data)
			end
		end
	end

	class LegPaymentStreamNegativeRateTreatment < Quickfix::IntField
		def LegPaymentStreamNegativeRateTreatment.field
			return 40349
		end
		def initialize(data = nil)
			if( data == nil )
				super(40349)
			else
				super(40349, data)
			end
		end
	end

	class LegPaymentStreamInflationLagPeriod < Quickfix::IntField
		def LegPaymentStreamInflationLagPeriod.field
			return 40350
		end
		def initialize(data = nil)
			if( data == nil )
				super(40350)
			else
				super(40350, data)
			end
		end
	end

	class LegPaymentStreamInflationLagUnit < Quickfix::StringField
		def LegPaymentStreamInflationLagUnit.field
			return 40351
		end
		def initialize(data = nil)
			if( data == nil )
				super(40351)
			else
				super(40351, data)
			end
		end
	end

	class LegPaymentStreamInflationLagDayType < Quickfix::IntField
		def LegPaymentStreamInflationLagDayType.field
			return 40352
		end
		def initialize(data = nil)
			if( data == nil )
				super(40352)
			else
				super(40352, data)
			end
		end
	end

	class LegPaymentStreamInflationInterpolationMethod < Quickfix::IntField
		def LegPaymentStreamInflationInterpolationMethod.field
			return 40353
		end
		def initialize(data = nil)
			if( data == nil )
				super(40353)
			else
				super(40353, data)
			end
		end
	end

	class LegPaymentStreamInflationIndexSource < Quickfix::IntField
		def LegPaymentStreamInflationIndexSource.field
			return 40354
		end
		def initialize(data = nil)
			if( data == nil )
				super(40354)
			else
				super(40354, data)
			end
		end
	end

	class LegPaymentStreamInflationPublicationSource < Quickfix::StringField
		def LegPaymentStreamInflationPublicationSource.field
			return 40355
		end
		def initialize(data = nil)
			if( data == nil )
				super(40355)
			else
				super(40355, data)
			end
		end
	end

	class LegPaymentStreamInflationInitialIndexLevel < Quickfix::DoubleField
		def LegPaymentStreamInflationInitialIndexLevel.field
			return 40356
		end
		def initialize(data = nil)
			if( data == nil )
				super(40356)
			else
				super(40356, data)
			end
		end
	end

	class LegPaymentStreamInflationFallbackBondApplicable < Quickfix::BoolField
		def LegPaymentStreamInflationFallbackBondApplicable.field
			return 40357
		end
		def initialize(data = nil)
			if( data == nil )
				super(40357)
			else
				super(40357, data)
			end
		end
	end

	class LegPaymentStreamFRADiscounting < Quickfix::IntField
		def LegPaymentStreamFRADiscounting.field
			return 40358
		end
		def initialize(data = nil)
			if( data == nil )
				super(40358)
			else
				super(40358, data)
			end
		end
	end

	class LegPaymentStreamNonDeliverableRefCurrency < Quickfix::StringField
		def LegPaymentStreamNonDeliverableRefCurrency.field
			return 40359
		end
		def initialize(data = nil)
			if( data == nil )
				super(40359)
			else
				super(40359, data)
			end
		end
	end

	class LegPaymentStreamNonDeliverableFixingDatesBusinessDayConvention < Quickfix::IntField
		def LegPaymentStreamNonDeliverableFixingDatesBusinessDayConvention.field
			return 40360
		end
		def initialize(data = nil)
			if( data == nil )
				super(40360)
			else
				super(40360, data)
			end
		end
	end

	class LegPaymentStreamNonDeliverableFixingDatesBusinessCenter < Quickfix::StringField
		def LegPaymentStreamNonDeliverableFixingDatesBusinessCenter.field
			return 40361
		end
		def initialize(data = nil)
			if( data == nil )
				super(40361)
			else
				super(40361, data)
			end
		end
	end

	class LegPaymentStreamNonDeliverableFixingDatesRelativeTo < Quickfix::IntField
		def LegPaymentStreamNonDeliverableFixingDatesRelativeTo.field
			return 40362
		end
		def initialize(data = nil)
			if( data == nil )
				super(40362)
			else
				super(40362, data)
			end
		end
	end

	class LegPaymentStreamNonDeliverableFixingDatesOffsetPeriod < Quickfix::IntField
		def LegPaymentStreamNonDeliverableFixingDatesOffsetPeriod.field
			return 40363
		end
		def initialize(data = nil)
			if( data == nil )
				super(40363)
			else
				super(40363, data)
			end
		end
	end

	class LegPaymentStreamNonDeliverableFixingDatesOffsetUnit < Quickfix::StringField
		def LegPaymentStreamNonDeliverableFixingDatesOffsetUnit.field
			return 40364
		end
		def initialize(data = nil)
			if( data == nil )
				super(40364)
			else
				super(40364, data)
			end
		end
	end

	class LegPaymentStreamNonDeliverableFixingDatesOffsetDayType < Quickfix::IntField
		def LegPaymentStreamNonDeliverableFixingDatesOffsetDayType.field
			return 40365
		end
		def initialize(data = nil)
			if( data == nil )
				super(40365)
			else
				super(40365, data)
			end
		end
	end

	class LegSettlRateFallbackRateSource < Quickfix::IntField
		def LegSettlRateFallbackRateSource.field
			return 40366
		end
		def initialize(data = nil)
			if( data == nil )
				super(40366)
			else
				super(40366, data)
			end
		end
	end

	class NoLegNonDeliverableFixingDates < Quickfix::IntField
		def NoLegNonDeliverableFixingDates.field
			return 40367
		end
		def initialize(data = nil)
			if( data == nil )
				super(40367)
			else
				super(40367, data)
			end
		end
	end

	class LegNonDeliverableFixingDate < Quickfix::StringField
		def LegNonDeliverableFixingDate.field
			return 40368
		end
		def initialize(data = nil)
			if( data == nil )
				super(40368)
			else
				super(40368, data)
			end
		end
	end

	class LegNonDeliverableFixingDateType < Quickfix::IntField
		def LegNonDeliverableFixingDateType.field
			return 40369
		end
		def initialize(data = nil)
			if( data == nil )
				super(40369)
			else
				super(40369, data)
			end
		end
	end

	class LegSettlRateFallbackReferencePage < Quickfix::StringField
		def LegSettlRateFallbackReferencePage.field
			return 40370
		end
		def initialize(data = nil)
			if( data == nil )
				super(40370)
			else
				super(40370, data)
			end
		end
	end

	class PaymentStreamNonDeliverableSettlRateSource < Quickfix::IntField
		def PaymentStreamNonDeliverableSettlRateSource.field
			return 40371
		end
		def initialize(data = nil)
			if( data == nil )
				super(40371)
			else
				super(40371, data)
			end
		end
	end

	class PaymentStreamNonDeliverableSettlReferencePage < Quickfix::StringField
		def PaymentStreamNonDeliverableSettlReferencePage.field
			return 40372
		end
		def initialize(data = nil)
			if( data == nil )
				super(40372)
			else
				super(40372, data)
			end
		end
	end

	class SettlRateFallbackRateSource < Quickfix::IntField
		def SettlRateFallbackRateSource.field
			return 40373
		end
		def initialize(data = nil)
			if( data == nil )
				super(40373)
			else
				super(40373, data)
			end
		end
	end

	class NoLegPaymentSchedules < Quickfix::IntField
		def NoLegPaymentSchedules.field
			return 40374
		end
		def initialize(data = nil)
			if( data == nil )
				super(40374)
			else
				super(40374, data)
			end
		end
	end

	class LegPaymentScheduleType < Quickfix::IntField
		def LegPaymentScheduleType.field
			return 40375
		end
		def initialize(data = nil)
			if( data == nil )
				super(40375)
			else
				super(40375, data)
			end
		end
	end

	class LegPaymentScheduleStubType < Quickfix::IntField
		def LegPaymentScheduleStubType.field
			return 40376
		end
		def initialize(data = nil)
			if( data == nil )
				super(40376)
			else
				super(40376, data)
			end
		end
	end

	class LegPaymentScheduleStartDateUnadjusted < Quickfix::StringField
		def LegPaymentScheduleStartDateUnadjusted.field
			return 40377
		end
		def initialize(data = nil)
			if( data == nil )
				super(40377)
			else
				super(40377, data)
			end
		end
	end

	class LegPaymentScheduleEndDateUnadjusted < Quickfix::StringField
		def LegPaymentScheduleEndDateUnadjusted.field
			return 40378
		end
		def initialize(data = nil)
			if( data == nil )
				super(40378)
			else
				super(40378, data)
			end
		end
	end

	class LegPaymentSchedulePaySide < Quickfix::IntField
		def LegPaymentSchedulePaySide.field
			return 40379
		end
		def initialize(data = nil)
			if( data == nil )
				super(40379)
			else
				super(40379, data)
			end
		end
	end

	class LegPaymentScheduleReceiveSide < Quickfix::IntField
		def LegPaymentScheduleReceiveSide.field
			return 40380
		end
		def initialize(data = nil)
			if( data == nil )
				super(40380)
			else
				super(40380, data)
			end
		end
	end

	class LegPaymentScheduleNotional < Quickfix::DoubleField
		def LegPaymentScheduleNotional.field
			return 40381
		end
		def initialize(data = nil)
			if( data == nil )
				super(40381)
			else
				super(40381, data)
			end
		end
	end

	class LegPaymentScheduleCurrency < Quickfix::StringField
		def LegPaymentScheduleCurrency.field
			return 40382
		end
		def initialize(data = nil)
			if( data == nil )
				super(40382)
			else
				super(40382, data)
			end
		end
	end

	class LegPaymentScheduleRate < Quickfix::DoubleField
		def LegPaymentScheduleRate.field
			return 40383
		end
		def initialize(data = nil)
			if( data == nil )
				super(40383)
			else
				super(40383, data)
			end
		end
	end

	class LegPaymentScheduleRateMultiplier < Quickfix::DoubleField
		def LegPaymentScheduleRateMultiplier.field
			return 40384
		end
		def initialize(data = nil)
			if( data == nil )
				super(40384)
			else
				super(40384, data)
			end
		end
	end

	class LegPaymentScheduleRateSpread < Quickfix::DoubleField
		def LegPaymentScheduleRateSpread.field
			return 40385
		end
		def initialize(data = nil)
			if( data == nil )
				super(40385)
			else
				super(40385, data)
			end
		end
	end

	class LegPaymentScheduleRateSpreadPositionType < Quickfix::IntField
		def LegPaymentScheduleRateSpreadPositionType.field
			return 40386
		end
		def initialize(data = nil)
			if( data == nil )
				super(40386)
			else
				super(40386, data)
			end
		end
	end

	class LegPaymentScheduleRateTreatment < Quickfix::IntField
		def LegPaymentScheduleRateTreatment.field
			return 40387
		end
		def initialize(data = nil)
			if( data == nil )
				super(40387)
			else
				super(40387, data)
			end
		end
	end

	class LegPaymentScheduleFixedAmount < Quickfix::DoubleField
		def LegPaymentScheduleFixedAmount.field
			return 40388
		end
		def initialize(data = nil)
			if( data == nil )
				super(40388)
			else
				super(40388, data)
			end
		end
	end

	class LegPaymentScheduleFixedCurrency < Quickfix::StringField
		def LegPaymentScheduleFixedCurrency.field
			return 40389
		end
		def initialize(data = nil)
			if( data == nil )
				super(40389)
			else
				super(40389, data)
			end
		end
	end

	class LegPaymentScheduleStepFrequencyPeriod < Quickfix::IntField
		def LegPaymentScheduleStepFrequencyPeriod.field
			return 40390
		end
		def initialize(data = nil)
			if( data == nil )
				super(40390)
			else
				super(40390, data)
			end
		end
	end

	class LegPaymentScheduleStepFrequencyUnit < Quickfix::StringField
		def LegPaymentScheduleStepFrequencyUnit.field
			return 40391
		end
		def initialize(data = nil)
			if( data == nil )
				super(40391)
			else
				super(40391, data)
			end
		end
	end

	class LegPaymentScheduleStepOffsetValue < Quickfix::DoubleField
		def LegPaymentScheduleStepOffsetValue.field
			return 40392
		end
		def initialize(data = nil)
			if( data == nil )
				super(40392)
			else
				super(40392, data)
			end
		end
	end

	class LegPaymentScheduleStepRate < Quickfix::DoubleField
		def LegPaymentScheduleStepRate.field
			return 40393
		end
		def initialize(data = nil)
			if( data == nil )
				super(40393)
			else
				super(40393, data)
			end
		end
	end

	class LegPaymentScheduleStepOffsetRate < Quickfix::DoubleField
		def LegPaymentScheduleStepOffsetRate.field
			return 40394
		end
		def initialize(data = nil)
			if( data == nil )
				super(40394)
			else
				super(40394, data)
			end
		end
	end

	class LegPaymentScheduleStepRelativeTo < Quickfix::IntField
		def LegPaymentScheduleStepRelativeTo.field
			return 40395
		end
		def initialize(data = nil)
			if( data == nil )
				super(40395)
			else
				super(40395, data)
			end
		end
	end

	class LegPaymentScheduleFixingDateUnadjusted < Quickfix::StringField
		def LegPaymentScheduleFixingDateUnadjusted.field
			return 40396
		end
		def initialize(data = nil)
			if( data == nil )
				super(40396)
			else
				super(40396, data)
			end
		end
	end

	class LegPaymentScheduleWeight < Quickfix::DoubleField
		def LegPaymentScheduleWeight.field
			return 40397
		end
		def initialize(data = nil)
			if( data == nil )
				super(40397)
			else
				super(40397, data)
			end
		end
	end

	class LegPaymentScheduleFixingDateRelativeTo < Quickfix::IntField
		def LegPaymentScheduleFixingDateRelativeTo.field
			return 40398
		end
		def initialize(data = nil)
			if( data == nil )
				super(40398)
			else
				super(40398, data)
			end
		end
	end

	class LegPaymentScheduleFixingDateBusinessDayConvention < Quickfix::IntField
		def LegPaymentScheduleFixingDateBusinessDayConvention.field
			return 40399
		end
		def initialize(data = nil)
			if( data == nil )
				super(40399)
			else
				super(40399, data)
			end
		end
	end

	class LegPaymentScheduleFixingDateBusinessCenter < Quickfix::StringField
		def LegPaymentScheduleFixingDateBusinessCenter.field
			return 40400
		end
		def initialize(data = nil)
			if( data == nil )
				super(40400)
			else
				super(40400, data)
			end
		end
	end

	class LegPaymentScheduleFixingDateOffsetPeriod < Quickfix::IntField
		def LegPaymentScheduleFixingDateOffsetPeriod.field
			return 40401
		end
		def initialize(data = nil)
			if( data == nil )
				super(40401)
			else
				super(40401, data)
			end
		end
	end

	class LegPaymentScheduleFixingDateOffsetUnit < Quickfix::StringField
		def LegPaymentScheduleFixingDateOffsetUnit.field
			return 40402
		end
		def initialize(data = nil)
			if( data == nil )
				super(40402)
			else
				super(40402, data)
			end
		end
	end

	class LegPaymentScheduleFixingDateOffsetDayType < Quickfix::IntField
		def LegPaymentScheduleFixingDateOffsetDayType.field
			return 40403
		end
		def initialize(data = nil)
			if( data == nil )
				super(40403)
			else
				super(40403, data)
			end
		end
	end

	class LegPaymentScheduleFixingDateAdjusted < Quickfix::StringField
		def LegPaymentScheduleFixingDateAdjusted.field
			return 40404
		end
		def initialize(data = nil)
			if( data == nil )
				super(40404)
			else
				super(40404, data)
			end
		end
	end

	class LegPaymentScheduleFixingTime < Quickfix::StringField
		def LegPaymentScheduleFixingTime.field
			return 40405
		end
		def initialize(data = nil)
			if( data == nil )
				super(40405)
			else
				super(40405, data)
			end
		end
	end

	class LegPaymentScheduleFixingTimeBusinessCenter < Quickfix::StringField
		def LegPaymentScheduleFixingTimeBusinessCenter.field
			return 40406
		end
		def initialize(data = nil)
			if( data == nil )
				super(40406)
			else
				super(40406, data)
			end
		end
	end

	class LegPaymentScheduleInterimExchangePaymentDateRelativeTo < Quickfix::IntField
		def LegPaymentScheduleInterimExchangePaymentDateRelativeTo.field
			return 40407
		end
		def initialize(data = nil)
			if( data == nil )
				super(40407)
			else
				super(40407, data)
			end
		end
	end

	class LegPaymentScheduleInterimExchangeDatesBusinessDayConvention < Quickfix::IntField
		def LegPaymentScheduleInterimExchangeDatesBusinessDayConvention.field
			return 40408
		end
		def initialize(data = nil)
			if( data == nil )
				super(40408)
			else
				super(40408, data)
			end
		end
	end

	class LegPaymentScheduleInterimExchangeDatesBusinessCenter < Quickfix::StringField
		def LegPaymentScheduleInterimExchangeDatesBusinessCenter.field
			return 40409
		end
		def initialize(data = nil)
			if( data == nil )
				super(40409)
			else
				super(40409, data)
			end
		end
	end

	class LegPaymentScheduleInterimExchangeDatesOffsetPeriod < Quickfix::IntField
		def LegPaymentScheduleInterimExchangeDatesOffsetPeriod.field
			return 40410
		end
		def initialize(data = nil)
			if( data == nil )
				super(40410)
			else
				super(40410, data)
			end
		end
	end

	class LegPaymentScheduleInterimExchangeDatesOffsetUnit < Quickfix::StringField
		def LegPaymentScheduleInterimExchangeDatesOffsetUnit.field
			return 40411
		end
		def initialize(data = nil)
			if( data == nil )
				super(40411)
			else
				super(40411, data)
			end
		end
	end

	class LegPaymentScheduleInterimExchangeDatesOffsetDayType < Quickfix::IntField
		def LegPaymentScheduleInterimExchangeDatesOffsetDayType.field
			return 40412
		end
		def initialize(data = nil)
			if( data == nil )
				super(40412)
			else
				super(40412, data)
			end
		end
	end

	class LegPaymentScheduleInterimExchangeDateAdjusted < Quickfix::StringField
		def LegPaymentScheduleInterimExchangeDateAdjusted.field
			return 40413
		end
		def initialize(data = nil)
			if( data == nil )
				super(40413)
			else
				super(40413, data)
			end
		end
	end

	class NoLegPaymentScheduleRateSources < Quickfix::IntField
		def NoLegPaymentScheduleRateSources.field
			return 40414
		end
		def initialize(data = nil)
			if( data == nil )
				super(40414)
			else
				super(40414, data)
			end
		end
	end

	class LegPaymentScheduleRateSource < Quickfix::IntField
		def LegPaymentScheduleRateSource.field
			return 40415
		end
		def initialize(data = nil)
			if( data == nil )
				super(40415)
			else
				super(40415, data)
			end
		end
	end

	class LegPaymentScheduleRateSourceType < Quickfix::IntField
		def LegPaymentScheduleRateSourceType.field
			return 40416
		end
		def initialize(data = nil)
			if( data == nil )
				super(40416)
			else
				super(40416, data)
			end
		end
	end

	class LegPaymentScheduleReferencePage < Quickfix::StringField
		def LegPaymentScheduleReferencePage.field
			return 40417
		end
		def initialize(data = nil)
			if( data == nil )
				super(40417)
			else
				super(40417, data)
			end
		end
	end

	class NoLegPaymentStubs < Quickfix::IntField
		def NoLegPaymentStubs.field
			return 40418
		end
		def initialize(data = nil)
			if( data == nil )
				super(40418)
			else
				super(40418, data)
			end
		end
	end

	class LegPaymentStubType < Quickfix::IntField
		def LegPaymentStubType.field
			return 40419
		end
		def initialize(data = nil)
			if( data == nil )
				super(40419)
			else
				super(40419, data)
			end
		end
	end

	class LegPaymentStubLength < Quickfix::IntField
		def LegPaymentStubLength.field
			return 40420
		end
		def initialize(data = nil)
			if( data == nil )
				super(40420)
			else
				super(40420, data)
			end
		end
	end

	class LegPaymentStubRate < Quickfix::DoubleField
		def LegPaymentStubRate.field
			return 40421
		end
		def initialize(data = nil)
			if( data == nil )
				super(40421)
			else
				super(40421, data)
			end
		end
	end

	class LegPaymentStubFixedAmount < Quickfix::DoubleField
		def LegPaymentStubFixedAmount.field
			return 40422
		end
		def initialize(data = nil)
			if( data == nil )
				super(40422)
			else
				super(40422, data)
			end
		end
	end

	class LegPaymentStubFixedCurrency < Quickfix::StringField
		def LegPaymentStubFixedCurrency.field
			return 40423
		end
		def initialize(data = nil)
			if( data == nil )
				super(40423)
			else
				super(40423, data)
			end
		end
	end

	class LegPaymentStubIndex < Quickfix::StringField
		def LegPaymentStubIndex.field
			return 40424
		end
		def initialize(data = nil)
			if( data == nil )
				super(40424)
			else
				super(40424, data)
			end
		end
	end

	class LegPaymentStubIndexSource < Quickfix::IntField
		def LegPaymentStubIndexSource.field
			return 40425
		end
		def initialize(data = nil)
			if( data == nil )
				super(40425)
			else
				super(40425, data)
			end
		end
	end

	class LegPaymentStubIndexCurvePeriod < Quickfix::IntField
		def LegPaymentStubIndexCurvePeriod.field
			return 40426
		end
		def initialize(data = nil)
			if( data == nil )
				super(40426)
			else
				super(40426, data)
			end
		end
	end

	class LegPaymentStubIndexCurveUnit < Quickfix::StringField
		def LegPaymentStubIndexCurveUnit.field
			return 40427
		end
		def initialize(data = nil)
			if( data == nil )
				super(40427)
			else
				super(40427, data)
			end
		end
	end

	class LegPaymentStubIndexRateMultiplier < Quickfix::DoubleField
		def LegPaymentStubIndexRateMultiplier.field
			return 40428
		end
		def initialize(data = nil)
			if( data == nil )
				super(40428)
			else
				super(40428, data)
			end
		end
	end

	class LegPaymentStubIndexRateSpread < Quickfix::DoubleField
		def LegPaymentStubIndexRateSpread.field
			return 40429
		end
		def initialize(data = nil)
			if( data == nil )
				super(40429)
			else
				super(40429, data)
			end
		end
	end

	class LegPaymentStubIndexRateSpreadPositionType < Quickfix::IntField
		def LegPaymentStubIndexRateSpreadPositionType.field
			return 40430
		end
		def initialize(data = nil)
			if( data == nil )
				super(40430)
			else
				super(40430, data)
			end
		end
	end

	class LegPaymentStubIndexRateTreatment < Quickfix::IntField
		def LegPaymentStubIndexRateTreatment.field
			return 40431
		end
		def initialize(data = nil)
			if( data == nil )
				super(40431)
			else
				super(40431, data)
			end
		end
	end

	class LegPaymentStubIndexCapRate < Quickfix::DoubleField
		def LegPaymentStubIndexCapRate.field
			return 40432
		end
		def initialize(data = nil)
			if( data == nil )
				super(40432)
			else
				super(40432, data)
			end
		end
	end

	class LegPaymentStubIndexCapRateBuySide < Quickfix::IntField
		def LegPaymentStubIndexCapRateBuySide.field
			return 40433
		end
		def initialize(data = nil)
			if( data == nil )
				super(40433)
			else
				super(40433, data)
			end
		end
	end

	class LegPaymentStubIndexCapRateSellSide < Quickfix::IntField
		def LegPaymentStubIndexCapRateSellSide.field
			return 40434
		end
		def initialize(data = nil)
			if( data == nil )
				super(40434)
			else
				super(40434, data)
			end
		end
	end

	class LegPaymentStubIndexFloorRate < Quickfix::DoubleField
		def LegPaymentStubIndexFloorRate.field
			return 40435
		end
		def initialize(data = nil)
			if( data == nil )
				super(40435)
			else
				super(40435, data)
			end
		end
	end

	class LegPaymentStubIndexFloorRateBuySide < Quickfix::IntField
		def LegPaymentStubIndexFloorRateBuySide.field
			return 40436
		end
		def initialize(data = nil)
			if( data == nil )
				super(40436)
			else
				super(40436, data)
			end
		end
	end

	class LegPaymentStubIndexFloorRateSellSide < Quickfix::IntField
		def LegPaymentStubIndexFloorRateSellSide.field
			return 40437
		end
		def initialize(data = nil)
			if( data == nil )
				super(40437)
			else
				super(40437, data)
			end
		end
	end

	class LegPaymentStubIndex2 < Quickfix::StringField
		def LegPaymentStubIndex2.field
			return 40438
		end
		def initialize(data = nil)
			if( data == nil )
				super(40438)
			else
				super(40438, data)
			end
		end
	end

	class LegPaymentStubIndex2Source < Quickfix::IntField
		def LegPaymentStubIndex2Source.field
			return 40439
		end
		def initialize(data = nil)
			if( data == nil )
				super(40439)
			else
				super(40439, data)
			end
		end
	end

	class LegPaymentStubIndex2CurvePeriod < Quickfix::IntField
		def LegPaymentStubIndex2CurvePeriod.field
			return 40440
		end
		def initialize(data = nil)
			if( data == nil )
				super(40440)
			else
				super(40440, data)
			end
		end
	end

	class LegPaymentStubIndex2CurveUnit < Quickfix::StringField
		def LegPaymentStubIndex2CurveUnit.field
			return 40441
		end
		def initialize(data = nil)
			if( data == nil )
				super(40441)
			else
				super(40441, data)
			end
		end
	end

	class LegPaymentStubIndex2RateMultiplier < Quickfix::DoubleField
		def LegPaymentStubIndex2RateMultiplier.field
			return 40442
		end
		def initialize(data = nil)
			if( data == nil )
				super(40442)
			else
				super(40442, data)
			end
		end
	end

	class LegPaymentStubIndex2RateSpread < Quickfix::DoubleField
		def LegPaymentStubIndex2RateSpread.field
			return 40443
		end
		def initialize(data = nil)
			if( data == nil )
				super(40443)
			else
				super(40443, data)
			end
		end
	end

	class LegPaymentStubIndex2RateSpreadPositionType < Quickfix::IntField
		def LegPaymentStubIndex2RateSpreadPositionType.field
			return 40444
		end
		def initialize(data = nil)
			if( data == nil )
				super(40444)
			else
				super(40444, data)
			end
		end
	end

	class LegPaymentStubIndex2RateTreatment < Quickfix::IntField
		def LegPaymentStubIndex2RateTreatment.field
			return 40445
		end
		def initialize(data = nil)
			if( data == nil )
				super(40445)
			else
				super(40445, data)
			end
		end
	end

	class LegPaymentStubIndex2CapRate < Quickfix::DoubleField
		def LegPaymentStubIndex2CapRate.field
			return 40446
		end
		def initialize(data = nil)
			if( data == nil )
				super(40446)
			else
				super(40446, data)
			end
		end
	end

	class LegPaymentStubIndex2FloorRate < Quickfix::DoubleField
		def LegPaymentStubIndex2FloorRate.field
			return 40447
		end
		def initialize(data = nil)
			if( data == nil )
				super(40447)
			else
				super(40447, data)
			end
		end
	end

	class NoLegProvisions < Quickfix::IntField
		def NoLegProvisions.field
			return 40448
		end
		def initialize(data = nil)
			if( data == nil )
				super(40448)
			else
				super(40448, data)
			end
		end
	end

	class LegProvisionType < Quickfix::IntField
		def LegProvisionType.field
			return 40449
		end
		def initialize(data = nil)
			if( data == nil )
				super(40449)
			else
				super(40449, data)
			end
		end
	end

	class LegProvisionDateUnadjusted < Quickfix::StringField
		def LegProvisionDateUnadjusted.field
			return 40450
		end
		def initialize(data = nil)
			if( data == nil )
				super(40450)
			else
				super(40450, data)
			end
		end
	end

	class LegProvisionDateBusinessDayConvention < Quickfix::IntField
		def LegProvisionDateBusinessDayConvention.field
			return 40451
		end
		def initialize(data = nil)
			if( data == nil )
				super(40451)
			else
				super(40451, data)
			end
		end
	end

	class LegProvisionDateBusinessCenter < Quickfix::StringField
		def LegProvisionDateBusinessCenter.field
			return 40452
		end
		def initialize(data = nil)
			if( data == nil )
				super(40452)
			else
				super(40452, data)
			end
		end
	end

	class LegProvisionDateAdjusted < Quickfix::StringField
		def LegProvisionDateAdjusted.field
			return 40453
		end
		def initialize(data = nil)
			if( data == nil )
				super(40453)
			else
				super(40453, data)
			end
		end
	end

	class LegProvisionDateTenorPeriod < Quickfix::IntField
		def LegProvisionDateTenorPeriod.field
			return 40454
		end
		def initialize(data = nil)
			if( data == nil )
				super(40454)
			else
				super(40454, data)
			end
		end
	end

	class LegProvisionDateTenorUnit < Quickfix::StringField
		def LegProvisionDateTenorUnit.field
			return 40455
		end
		def initialize(data = nil)
			if( data == nil )
				super(40455)
			else
				super(40455, data)
			end
		end
	end

	class LegProvisionCalculationAgent < Quickfix::IntField
		def LegProvisionCalculationAgent.field
			return 40456
		end
		def initialize(data = nil)
			if( data == nil )
				super(40456)
			else
				super(40456, data)
			end
		end
	end

	class LegProvisionOptionSinglePartyBuyerSide < Quickfix::IntField
		def LegProvisionOptionSinglePartyBuyerSide.field
			return 40457
		end
		def initialize(data = nil)
			if( data == nil )
				super(40457)
			else
				super(40457, data)
			end
		end
	end

	class LegProvisionOptionSinglePartySellerSide < Quickfix::IntField
		def LegProvisionOptionSinglePartySellerSide.field
			return 40458
		end
		def initialize(data = nil)
			if( data == nil )
				super(40458)
			else
				super(40458, data)
			end
		end
	end

	class LegProvisionOptionExerciseStyle < Quickfix::IntField
		def LegProvisionOptionExerciseStyle.field
			return 40459
		end
		def initialize(data = nil)
			if( data == nil )
				super(40459)
			else
				super(40459, data)
			end
		end
	end

	class LegProvisionOptionExerciseMultipleNotional < Quickfix::DoubleField
		def LegProvisionOptionExerciseMultipleNotional.field
			return 40460
		end
		def initialize(data = nil)
			if( data == nil )
				super(40460)
			else
				super(40460, data)
			end
		end
	end

	class LegProvisionOptionExerciseMinimumNotional < Quickfix::DoubleField
		def LegProvisionOptionExerciseMinimumNotional.field
			return 40461
		end
		def initialize(data = nil)
			if( data == nil )
				super(40461)
			else
				super(40461, data)
			end
		end
	end

	class LegProvisionOptionExerciseMaximumNotional < Quickfix::DoubleField
		def LegProvisionOptionExerciseMaximumNotional.field
			return 40462
		end
		def initialize(data = nil)
			if( data == nil )
				super(40462)
			else
				super(40462, data)
			end
		end
	end

	class LegProvisionOptionMinimumNumber < Quickfix::IntField
		def LegProvisionOptionMinimumNumber.field
			return 40463
		end
		def initialize(data = nil)
			if( data == nil )
				super(40463)
			else
				super(40463, data)
			end
		end
	end

	class LegProvisionOptionMaximumNumber < Quickfix::IntField
		def LegProvisionOptionMaximumNumber.field
			return 40464
		end
		def initialize(data = nil)
			if( data == nil )
				super(40464)
			else
				super(40464, data)
			end
		end
	end

	class LegProvisionOptionExerciseConfirmation < Quickfix::BoolField
		def LegProvisionOptionExerciseConfirmation.field
			return 40465
		end
		def initialize(data = nil)
			if( data == nil )
				super(40465)
			else
				super(40465, data)
			end
		end
	end

	class LegProvisionCashSettlMethod < Quickfix::IntField
		def LegProvisionCashSettlMethod.field
			return 40466
		end
		def initialize(data = nil)
			if( data == nil )
				super(40466)
			else
				super(40466, data)
			end
		end
	end

	class LegProvisionCashSettlCurrency < Quickfix::StringField
		def LegProvisionCashSettlCurrency.field
			return 40467
		end
		def initialize(data = nil)
			if( data == nil )
				super(40467)
			else
				super(40467, data)
			end
		end
	end

	class LegProvisionCashSettlCurrency2 < Quickfix::StringField
		def LegProvisionCashSettlCurrency2.field
			return 40468
		end
		def initialize(data = nil)
			if( data == nil )
				super(40468)
			else
				super(40468, data)
			end
		end
	end

	class LegProvisionCashSettlQuoteType < Quickfix::IntField
		def LegProvisionCashSettlQuoteType.field
			return 40469
		end
		def initialize(data = nil)
			if( data == nil )
				super(40469)
			else
				super(40469, data)
			end
		end
	end

	class LegProvisionCashSettlQuoteSource < Quickfix::IntField
		def LegProvisionCashSettlQuoteSource.field
			return 40470
		end
		def initialize(data = nil)
			if( data == nil )
				super(40470)
			else
				super(40470, data)
			end
		end
	end

	class BusinessCenter < Quickfix::StringField
		def BusinessCenter.field
			return 40471
		end
		def initialize(data = nil)
			if( data == nil )
				super(40471)
			else
				super(40471, data)
			end
		end
	end

	class LegProvisionText < Quickfix::StringField
		def LegProvisionText.field
			return 40472
		end
		def initialize(data = nil)
			if( data == nil )
				super(40472)
			else
				super(40472, data)
			end
		end
	end

	class NoLegProvisionCashSettlPaymentDates < Quickfix::IntField
		def NoLegProvisionCashSettlPaymentDates.field
			return 40473
		end
		def initialize(data = nil)
			if( data == nil )
				super(40473)
			else
				super(40473, data)
			end
		end
	end

	class LegProvisionCashSettlPaymentDate < Quickfix::StringField
		def LegProvisionCashSettlPaymentDate.field
			return 40474
		end
		def initialize(data = nil)
			if( data == nil )
				super(40474)
			else
				super(40474, data)
			end
		end
	end

	class LegProvisionCashSettlPaymentDateType < Quickfix::IntField
		def LegProvisionCashSettlPaymentDateType.field
			return 40475
		end
		def initialize(data = nil)
			if( data == nil )
				super(40475)
			else
				super(40475, data)
			end
		end
	end

	class LegProvisionOptionExerciseBusinessDayConvention < Quickfix::IntField
		def LegProvisionOptionExerciseBusinessDayConvention.field
			return 40476
		end
		def initialize(data = nil)
			if( data == nil )
				super(40476)
			else
				super(40476, data)
			end
		end
	end

	class LegProvisionOptionExerciseBusinessCenter < Quickfix::StringField
		def LegProvisionOptionExerciseBusinessCenter.field
			return 40477
		end
		def initialize(data = nil)
			if( data == nil )
				super(40477)
			else
				super(40477, data)
			end
		end
	end

	class LegProvisionOptionExerciseEarliestDateOffsetPeriod < Quickfix::IntField
		def LegProvisionOptionExerciseEarliestDateOffsetPeriod.field
			return 40478
		end
		def initialize(data = nil)
			if( data == nil )
				super(40478)
			else
				super(40478, data)
			end
		end
	end

	class LegProvisionOptionExerciseEarliestDateOffsetUnit < Quickfix::StringField
		def LegProvisionOptionExerciseEarliestDateOffsetUnit.field
			return 40479
		end
		def initialize(data = nil)
			if( data == nil )
				super(40479)
			else
				super(40479, data)
			end
		end
	end

	class LegProvisionOptionExerciseFrequencyPeriod < Quickfix::IntField
		def LegProvisionOptionExerciseFrequencyPeriod.field
			return 40480
		end
		def initialize(data = nil)
			if( data == nil )
				super(40480)
			else
				super(40480, data)
			end
		end
	end

	class LegProvisionOptionExerciseFrequencyUnit < Quickfix::StringField
		def LegProvisionOptionExerciseFrequencyUnit.field
			return 40481
		end
		def initialize(data = nil)
			if( data == nil )
				super(40481)
			else
				super(40481, data)
			end
		end
	end

	class LegProvisionOptionExerciseStartDateUnadjusted < Quickfix::StringField
		def LegProvisionOptionExerciseStartDateUnadjusted.field
			return 40482
		end
		def initialize(data = nil)
			if( data == nil )
				super(40482)
			else
				super(40482, data)
			end
		end
	end

	class LegProvisionOptionExerciseStartDateRelativeTo < Quickfix::IntField
		def LegProvisionOptionExerciseStartDateRelativeTo.field
			return 40483
		end
		def initialize(data = nil)
			if( data == nil )
				super(40483)
			else
				super(40483, data)
			end
		end
	end

	class LegProvisionOptionExerciseStartDateOffsetPeriod < Quickfix::IntField
		def LegProvisionOptionExerciseStartDateOffsetPeriod.field
			return 40484
		end
		def initialize(data = nil)
			if( data == nil )
				super(40484)
			else
				super(40484, data)
			end
		end
	end

	class LegProvisionOptionExerciseStartDateOffsetUnit < Quickfix::StringField
		def LegProvisionOptionExerciseStartDateOffsetUnit.field
			return 40485
		end
		def initialize(data = nil)
			if( data == nil )
				super(40485)
			else
				super(40485, data)
			end
		end
	end

	class LegProvisionOptionExerciseStartDateOffsetDayType < Quickfix::IntField
		def LegProvisionOptionExerciseStartDateOffsetDayType.field
			return 40486
		end
		def initialize(data = nil)
			if( data == nil )
				super(40486)
			else
				super(40486, data)
			end
		end
	end

	class LegProvisionOptionExerciseStartDateAdjusted < Quickfix::StringField
		def LegProvisionOptionExerciseStartDateAdjusted.field
			return 40487
		end
		def initialize(data = nil)
			if( data == nil )
				super(40487)
			else
				super(40487, data)
			end
		end
	end

	class LegProvisionOptionExercisePeriodSkip < Quickfix::IntField
		def LegProvisionOptionExercisePeriodSkip.field
			return 40488
		end
		def initialize(data = nil)
			if( data == nil )
				super(40488)
			else
				super(40488, data)
			end
		end
	end

	class LegProvisionOptionExerciseBoundsFirstDateUnadjusted < Quickfix::StringField
		def LegProvisionOptionExerciseBoundsFirstDateUnadjusted.field
			return 40489
		end
		def initialize(data = nil)
			if( data == nil )
				super(40489)
			else
				super(40489, data)
			end
		end
	end

	class LegProvisionOptionExerciseBoundsLastDateUnadjusted < Quickfix::StringField
		def LegProvisionOptionExerciseBoundsLastDateUnadjusted.field
			return 40490
		end
		def initialize(data = nil)
			if( data == nil )
				super(40490)
			else
				super(40490, data)
			end
		end
	end

	class LegProvisionOptionExerciseEarliestTime < Quickfix::StringField
		def LegProvisionOptionExerciseEarliestTime.field
			return 40491
		end
		def initialize(data = nil)
			if( data == nil )
				super(40491)
			else
				super(40491, data)
			end
		end
	end

	class LegProvisionOptionExerciseEarliestTimeBusinessCenter < Quickfix::StringField
		def LegProvisionOptionExerciseEarliestTimeBusinessCenter.field
			return 40492
		end
		def initialize(data = nil)
			if( data == nil )
				super(40492)
			else
				super(40492, data)
			end
		end
	end

	class LegProvisionOptionExerciseLatestTime < Quickfix::StringField
		def LegProvisionOptionExerciseLatestTime.field
			return 40493
		end
		def initialize(data = nil)
			if( data == nil )
				super(40493)
			else
				super(40493, data)
			end
		end
	end

	class LegProvisionOptionExerciseLatestTimeBusinessCenter < Quickfix::StringField
		def LegProvisionOptionExerciseLatestTimeBusinessCenter.field
			return 40494
		end
		def initialize(data = nil)
			if( data == nil )
				super(40494)
			else
				super(40494, data)
			end
		end
	end

	class NoLegProvisionOptionExerciseFixedDates < Quickfix::IntField
		def NoLegProvisionOptionExerciseFixedDates.field
			return 40495
		end
		def initialize(data = nil)
			if( data == nil )
				super(40495)
			else
				super(40495, data)
			end
		end
	end

	class LegProvisionOptionExerciseFixedDate < Quickfix::StringField
		def LegProvisionOptionExerciseFixedDate.field
			return 40496
		end
		def initialize(data = nil)
			if( data == nil )
				super(40496)
			else
				super(40496, data)
			end
		end
	end

	class LegProvisionOptionExerciseFixedDateType < Quickfix::IntField
		def LegProvisionOptionExerciseFixedDateType.field
			return 40497
		end
		def initialize(data = nil)
			if( data == nil )
				super(40497)
			else
				super(40497, data)
			end
		end
	end

	class LegProvisionOptionExpirationDateUnadjusted < Quickfix::StringField
		def LegProvisionOptionExpirationDateUnadjusted.field
			return 40498
		end
		def initialize(data = nil)
			if( data == nil )
				super(40498)
			else
				super(40498, data)
			end
		end
	end

	class LegProvisionOptionExpirationDateBusinessDayConvention < Quickfix::IntField
		def LegProvisionOptionExpirationDateBusinessDayConvention.field
			return 40499
		end
		def initialize(data = nil)
			if( data == nil )
				super(40499)
			else
				super(40499, data)
			end
		end
	end

	class LegProvisionOptionExpirationDateBusinessCenter < Quickfix::StringField
		def LegProvisionOptionExpirationDateBusinessCenter.field
			return 40500
		end
		def initialize(data = nil)
			if( data == nil )
				super(40500)
			else
				super(40500, data)
			end
		end
	end

	class LegProvisionOptionExpirationDateRelativeTo < Quickfix::IntField
		def LegProvisionOptionExpirationDateRelativeTo.field
			return 40501
		end
		def initialize(data = nil)
			if( data == nil )
				super(40501)
			else
				super(40501, data)
			end
		end
	end

	class LegProvisionOptionExpirationDateOffsetPeriod < Quickfix::IntField
		def LegProvisionOptionExpirationDateOffsetPeriod.field
			return 40502
		end
		def initialize(data = nil)
			if( data == nil )
				super(40502)
			else
				super(40502, data)
			end
		end
	end

	class LegProvisionOptionExpirationDateOffsetUnit < Quickfix::StringField
		def LegProvisionOptionExpirationDateOffsetUnit.field
			return 40503
		end
		def initialize(data = nil)
			if( data == nil )
				super(40503)
			else
				super(40503, data)
			end
		end
	end

	class LegProvisionOptionExpirationDateOffsetDayType < Quickfix::IntField
		def LegProvisionOptionExpirationDateOffsetDayType.field
			return 40504
		end
		def initialize(data = nil)
			if( data == nil )
				super(40504)
			else
				super(40504, data)
			end
		end
	end

	class LegProvisionOptionExpirationDateAdjusted < Quickfix::StringField
		def LegProvisionOptionExpirationDateAdjusted.field
			return 40505
		end
		def initialize(data = nil)
			if( data == nil )
				super(40505)
			else
				super(40505, data)
			end
		end
	end

	class LegProvisionOptionExpirationTime < Quickfix::StringField
		def LegProvisionOptionExpirationTime.field
			return 40506
		end
		def initialize(data = nil)
			if( data == nil )
				super(40506)
			else
				super(40506, data)
			end
		end
	end

	class LegProvisionOptionExpirationTimeBusinessCenter < Quickfix::StringField
		def LegProvisionOptionExpirationTimeBusinessCenter.field
			return 40507
		end
		def initialize(data = nil)
			if( data == nil )
				super(40507)
			else
				super(40507, data)
			end
		end
	end

	class LegProvisionOptionRelevantUnderlyingDateUnadjusted < Quickfix::StringField
		def LegProvisionOptionRelevantUnderlyingDateUnadjusted.field
			return 40508
		end
		def initialize(data = nil)
			if( data == nil )
				super(40508)
			else
				super(40508, data)
			end
		end
	end

	class LegProvisionOptionRelevantUnderlyingDateBusinessDayConvention < Quickfix::IntField
		def LegProvisionOptionRelevantUnderlyingDateBusinessDayConvention.field
			return 40509
		end
		def initialize(data = nil)
			if( data == nil )
				super(40509)
			else
				super(40509, data)
			end
		end
	end

	class LegProvisionOptionRelevantUnderlyingDateBusinessCenter < Quickfix::StringField
		def LegProvisionOptionRelevantUnderlyingDateBusinessCenter.field
			return 40510
		end
		def initialize(data = nil)
			if( data == nil )
				super(40510)
			else
				super(40510, data)
			end
		end
	end

	class LegProvisionOptionRelevantUnderlyingDateRelativeTo < Quickfix::IntField
		def LegProvisionOptionRelevantUnderlyingDateRelativeTo.field
			return 40511
		end
		def initialize(data = nil)
			if( data == nil )
				super(40511)
			else
				super(40511, data)
			end
		end
	end

	class LegProvisionOptionRelevantUnderlyingDateOffsetPeriod < Quickfix::IntField
		def LegProvisionOptionRelevantUnderlyingDateOffsetPeriod.field
			return 40512
		end
		def initialize(data = nil)
			if( data == nil )
				super(40512)
			else
				super(40512, data)
			end
		end
	end

	class LegProvisionOptionRelevantUnderlyingDateOffsetUnit < Quickfix::StringField
		def LegProvisionOptionRelevantUnderlyingDateOffsetUnit.field
			return 40513
		end
		def initialize(data = nil)
			if( data == nil )
				super(40513)
			else
				super(40513, data)
			end
		end
	end

	class LegProvisionOptionRelevantUnderlyingDateOffsetDayType < Quickfix::IntField
		def LegProvisionOptionRelevantUnderlyingDateOffsetDayType.field
			return 40514
		end
		def initialize(data = nil)
			if( data == nil )
				super(40514)
			else
				super(40514, data)
			end
		end
	end

	class LegProvisionOptionRelevantUnderlyingDateAdjusted < Quickfix::StringField
		def LegProvisionOptionRelevantUnderlyingDateAdjusted.field
			return 40515
		end
		def initialize(data = nil)
			if( data == nil )
				super(40515)
			else
				super(40515, data)
			end
		end
	end

	class LegProvisionCashSettlPaymentDateBusinessDayConvention < Quickfix::IntField
		def LegProvisionCashSettlPaymentDateBusinessDayConvention.field
			return 40516
		end
		def initialize(data = nil)
			if( data == nil )
				super(40516)
			else
				super(40516, data)
			end
		end
	end

	class LegProvisionCashSettlPaymentDateBusinessCenter < Quickfix::StringField
		def LegProvisionCashSettlPaymentDateBusinessCenter.field
			return 40517
		end
		def initialize(data = nil)
			if( data == nil )
				super(40517)
			else
				super(40517, data)
			end
		end
	end

	class LegProvisionCashSettlPaymentDateRelativeTo < Quickfix::IntField
		def LegProvisionCashSettlPaymentDateRelativeTo.field
			return 40518
		end
		def initialize(data = nil)
			if( data == nil )
				super(40518)
			else
				super(40518, data)
			end
		end
	end

	class LegProvisionCashSettlPaymentDateOffsetPeriod < Quickfix::IntField
		def LegProvisionCashSettlPaymentDateOffsetPeriod.field
			return 40519
		end
		def initialize(data = nil)
			if( data == nil )
				super(40519)
			else
				super(40519, data)
			end
		end
	end

	class LegProvisionCashSettlPaymentDateOffsetUnit < Quickfix::StringField
		def LegProvisionCashSettlPaymentDateOffsetUnit.field
			return 40520
		end
		def initialize(data = nil)
			if( data == nil )
				super(40520)
			else
				super(40520, data)
			end
		end
	end

	class LegProvisionCashSettlPaymentDateOffsetDayType < Quickfix::IntField
		def LegProvisionCashSettlPaymentDateOffsetDayType.field
			return 40521
		end
		def initialize(data = nil)
			if( data == nil )
				super(40521)
			else
				super(40521, data)
			end
		end
	end

	class LegProvisionCashSettlPaymentDateRangeFirst < Quickfix::StringField
		def LegProvisionCashSettlPaymentDateRangeFirst.field
			return 40522
		end
		def initialize(data = nil)
			if( data == nil )
				super(40522)
			else
				super(40522, data)
			end
		end
	end

	class LegProvisionCashSettlPaymentDateRangeLast < Quickfix::StringField
		def LegProvisionCashSettlPaymentDateRangeLast.field
			return 40523
		end
		def initialize(data = nil)
			if( data == nil )
				super(40523)
			else
				super(40523, data)
			end
		end
	end

	class LegProvisionCashSettlValueTime < Quickfix::StringField
		def LegProvisionCashSettlValueTime.field
			return 40524
		end
		def initialize(data = nil)
			if( data == nil )
				super(40524)
			else
				super(40524, data)
			end
		end
	end

	class LegProvisionCashSettlValueTimeBusinessCenter < Quickfix::StringField
		def LegProvisionCashSettlValueTimeBusinessCenter.field
			return 40525
		end
		def initialize(data = nil)
			if( data == nil )
				super(40525)
			else
				super(40525, data)
			end
		end
	end

	class LegProvisionCashSettlValueDateBusinessDayConvention < Quickfix::IntField
		def LegProvisionCashSettlValueDateBusinessDayConvention.field
			return 40526
		end
		def initialize(data = nil)
			if( data == nil )
				super(40526)
			else
				super(40526, data)
			end
		end
	end

	class LegProvisionCashSettlValueDateBusinessCenter < Quickfix::StringField
		def LegProvisionCashSettlValueDateBusinessCenter.field
			return 40527
		end
		def initialize(data = nil)
			if( data == nil )
				super(40527)
			else
				super(40527, data)
			end
		end
	end

	class LegProvisionCashSettlValueDateRelativeTo < Quickfix::IntField
		def LegProvisionCashSettlValueDateRelativeTo.field
			return 40528
		end
		def initialize(data = nil)
			if( data == nil )
				super(40528)
			else
				super(40528, data)
			end
		end
	end

	class LegProvisionCashSettlValueDateOffsetPeriod < Quickfix::IntField
		def LegProvisionCashSettlValueDateOffsetPeriod.field
			return 40529
		end
		def initialize(data = nil)
			if( data == nil )
				super(40529)
			else
				super(40529, data)
			end
		end
	end

	class LegProvisionCashSettlValueDateOffsetUnit < Quickfix::StringField
		def LegProvisionCashSettlValueDateOffsetUnit.field
			return 40530
		end
		def initialize(data = nil)
			if( data == nil )
				super(40530)
			else
				super(40530, data)
			end
		end
	end

	class LegProvisionCashSettlValueDateOffsetDayType < Quickfix::IntField
		def LegProvisionCashSettlValueDateOffsetDayType.field
			return 40531
		end
		def initialize(data = nil)
			if( data == nil )
				super(40531)
			else
				super(40531, data)
			end
		end
	end

	class LegProvisionCashSettlValueDateAdjusted < Quickfix::StringField
		def LegProvisionCashSettlValueDateAdjusted.field
			return 40532
		end
		def initialize(data = nil)
			if( data == nil )
				super(40532)
			else
				super(40532, data)
			end
		end
	end

	class NoLegProvisionPartyIDs < Quickfix::IntField
		def NoLegProvisionPartyIDs.field
			return 40533
		end
		def initialize(data = nil)
			if( data == nil )
				super(40533)
			else
				super(40533, data)
			end
		end
	end

	class LegProvisionPartyID < Quickfix::StringField
		def LegProvisionPartyID.field
			return 40534
		end
		def initialize(data = nil)
			if( data == nil )
				super(40534)
			else
				super(40534, data)
			end
		end
	end

	class LegProvisionPartyIDSource < Quickfix::CharField
		def LegProvisionPartyIDSource.field
			return 40535
		end
		def initialize(data = nil)
			if( data == nil )
				super(40535)
			else
				super(40535, data)
			end
		end
	end

	class LegProvisionPartyRole < Quickfix::IntField
		def LegProvisionPartyRole.field
			return 40536
		end
		def initialize(data = nil)
			if( data == nil )
				super(40536)
			else
				super(40536, data)
			end
		end
	end

	class NoLegProvisionPartySubIDs < Quickfix::IntField
		def NoLegProvisionPartySubIDs.field
			return 40537
		end
		def initialize(data = nil)
			if( data == nil )
				super(40537)
			else
				super(40537, data)
			end
		end
	end

	class LegProvisionPartySubID < Quickfix::StringField
		def LegProvisionPartySubID.field
			return 40538
		end
		def initialize(data = nil)
			if( data == nil )
				super(40538)
			else
				super(40538, data)
			end
		end
	end

	class LegProvisionPartySubIDType < Quickfix::IntField
		def LegProvisionPartySubIDType.field
			return 40539
		end
		def initialize(data = nil)
			if( data == nil )
				super(40539)
			else
				super(40539, data)
			end
		end
	end

	class NoUnderlyingStreams < Quickfix::IntField
		def NoUnderlyingStreams.field
			return 40540
		end
		def initialize(data = nil)
			if( data == nil )
				super(40540)
			else
				super(40540, data)
			end
		end
	end

	class UnderlyingStreamType < Quickfix::IntField
		def UnderlyingStreamType.field
			return 40541
		end
		def initialize(data = nil)
			if( data == nil )
				super(40541)
			else
				super(40541, data)
			end
		end
	end

	class UnderlyingStreamDesc < Quickfix::StringField
		def UnderlyingStreamDesc.field
			return 40542
		end
		def initialize(data = nil)
			if( data == nil )
				super(40542)
			else
				super(40542, data)
			end
		end
	end

	class UnderlyingStreamPaySide < Quickfix::IntField
		def UnderlyingStreamPaySide.field
			return 40543
		end
		def initialize(data = nil)
			if( data == nil )
				super(40543)
			else
				super(40543, data)
			end
		end
	end

	class UnderlyingStreamReceiveSide < Quickfix::IntField
		def UnderlyingStreamReceiveSide.field
			return 40544
		end
		def initialize(data = nil)
			if( data == nil )
				super(40544)
			else
				super(40544, data)
			end
		end
	end

	class UnderlyingStreamNotional < Quickfix::DoubleField
		def UnderlyingStreamNotional.field
			return 40545
		end
		def initialize(data = nil)
			if( data == nil )
				super(40545)
			else
				super(40545, data)
			end
		end
	end

	class UnderlyingStreamCurrency < Quickfix::StringField
		def UnderlyingStreamCurrency.field
			return 40546
		end
		def initialize(data = nil)
			if( data == nil )
				super(40546)
			else
				super(40546, data)
			end
		end
	end

	class UnderlyingStreamText < Quickfix::StringField
		def UnderlyingStreamText.field
			return 40547
		end
		def initialize(data = nil)
			if( data == nil )
				super(40547)
			else
				super(40547, data)
			end
		end
	end

	class UnderlyingStreamTerminationDateUnadjusted < Quickfix::StringField
		def UnderlyingStreamTerminationDateUnadjusted.field
			return 40548
		end
		def initialize(data = nil)
			if( data == nil )
				super(40548)
			else
				super(40548, data)
			end
		end
	end

	class UnderlyingStreamTerminationDateBusinessDayConvention < Quickfix::IntField
		def UnderlyingStreamTerminationDateBusinessDayConvention.field
			return 40549
		end
		def initialize(data = nil)
			if( data == nil )
				super(40549)
			else
				super(40549, data)
			end
		end
	end

	class UnderlyingStreamTerminationDateBusinessCenter < Quickfix::StringField
		def UnderlyingStreamTerminationDateBusinessCenter.field
			return 40550
		end
		def initialize(data = nil)
			if( data == nil )
				super(40550)
			else
				super(40550, data)
			end
		end
	end

	class UnderlyingStreamTerminationDateRelativeTo < Quickfix::IntField
		def UnderlyingStreamTerminationDateRelativeTo.field
			return 40551
		end
		def initialize(data = nil)
			if( data == nil )
				super(40551)
			else
				super(40551, data)
			end
		end
	end

	class UnderlyingStreamTerminationDateOffsetPeriod < Quickfix::IntField
		def UnderlyingStreamTerminationDateOffsetPeriod.field
			return 40552
		end
		def initialize(data = nil)
			if( data == nil )
				super(40552)
			else
				super(40552, data)
			end
		end
	end

	class UnderlyingStreamTerminationDateOffsetUnit < Quickfix::StringField
		def UnderlyingStreamTerminationDateOffsetUnit.field
			return 40553
		end
		def initialize(data = nil)
			if( data == nil )
				super(40553)
			else
				super(40553, data)
			end
		end
	end

	class UnderlyingStreamTerminationDateOffsetDayType < Quickfix::IntField
		def UnderlyingStreamTerminationDateOffsetDayType.field
			return 40554
		end
		def initialize(data = nil)
			if( data == nil )
				super(40554)
			else
				super(40554, data)
			end
		end
	end

	class UnderlyingStreamTerminationDateAdjusted < Quickfix::StringField
		def UnderlyingStreamTerminationDateAdjusted.field
			return 40555
		end
		def initialize(data = nil)
			if( data == nil )
				super(40555)
			else
				super(40555, data)
			end
		end
	end

	class UnderlyingStreamCalculationPeriodBusinessDayConvention < Quickfix::IntField
		def UnderlyingStreamCalculationPeriodBusinessDayConvention.field
			return 40556
		end
		def initialize(data = nil)
			if( data == nil )
				super(40556)
			else
				super(40556, data)
			end
		end
	end

	class UnderlyingStreamCalculationPeriodBusinessCenter < Quickfix::StringField
		def UnderlyingStreamCalculationPeriodBusinessCenter.field
			return 40557
		end
		def initialize(data = nil)
			if( data == nil )
				super(40557)
			else
				super(40557, data)
			end
		end
	end

	class UnderlyingStreamFirstPeriodStartDateUnadjusted < Quickfix::StringField
		def UnderlyingStreamFirstPeriodStartDateUnadjusted.field
			return 40558
		end
		def initialize(data = nil)
			if( data == nil )
				super(40558)
			else
				super(40558, data)
			end
		end
	end

	class UnderlyingStreamFirstPeriodStartDateBusinessDayConvention < Quickfix::IntField
		def UnderlyingStreamFirstPeriodStartDateBusinessDayConvention.field
			return 40559
		end
		def initialize(data = nil)
			if( data == nil )
				super(40559)
			else
				super(40559, data)
			end
		end
	end

	class UnderlyingStreamFirstPeriodStartDateBusinessCenter < Quickfix::StringField
		def UnderlyingStreamFirstPeriodStartDateBusinessCenter.field
			return 40560
		end
		def initialize(data = nil)
			if( data == nil )
				super(40560)
			else
				super(40560, data)
			end
		end
	end

	class UnderlyingStreamFirstPeriodStartDateAdjusted < Quickfix::StringField
		def UnderlyingStreamFirstPeriodStartDateAdjusted.field
			return 40561
		end
		def initialize(data = nil)
			if( data == nil )
				super(40561)
			else
				super(40561, data)
			end
		end
	end

	class UnderlyingStreamFirstRegularPeriodStartDateUnadjusted < Quickfix::StringField
		def UnderlyingStreamFirstRegularPeriodStartDateUnadjusted.field
			return 40562
		end
		def initialize(data = nil)
			if( data == nil )
				super(40562)
			else
				super(40562, data)
			end
		end
	end

	class UnderlyingStreamFirstCompoundingPeriodEndDateUnadjusted < Quickfix::StringField
		def UnderlyingStreamFirstCompoundingPeriodEndDateUnadjusted.field
			return 40563
		end
		def initialize(data = nil)
			if( data == nil )
				super(40563)
			else
				super(40563, data)
			end
		end
	end

	class UnderlyingStreamLastRegularPeriodEndDateUnadjusted < Quickfix::StringField
		def UnderlyingStreamLastRegularPeriodEndDateUnadjusted.field
			return 40564
		end
		def initialize(data = nil)
			if( data == nil )
				super(40564)
			else
				super(40564, data)
			end
		end
	end

	class UnderlyingStreamCalculationFrequencyPeriod < Quickfix::IntField
		def UnderlyingStreamCalculationFrequencyPeriod.field
			return 40565
		end
		def initialize(data = nil)
			if( data == nil )
				super(40565)
			else
				super(40565, data)
			end
		end
	end

	class UnderlyingStreamCalculationFrequencyUnit < Quickfix::StringField
		def UnderlyingStreamCalculationFrequencyUnit.field
			return 40566
		end
		def initialize(data = nil)
			if( data == nil )
				super(40566)
			else
				super(40566, data)
			end
		end
	end

	class UnderlyingStreamCalculationRollConvention < Quickfix::StringField
		def UnderlyingStreamCalculationRollConvention.field
			return 40567
		end
		def initialize(data = nil)
			if( data == nil )
				super(40567)
			else
				super(40567, data)
			end
		end
	end

	class UnderlyingPaymentStreamType < Quickfix::IntField
		def UnderlyingPaymentStreamType.field
			return 40568
		end
		def initialize(data = nil)
			if( data == nil )
				super(40568)
			else
				super(40568, data)
			end
		end
	end

	class UnderlyingPaymentStreamMarketRate < Quickfix::IntField
		def UnderlyingPaymentStreamMarketRate.field
			return 40569
		end
		def initialize(data = nil)
			if( data == nil )
				super(40569)
			else
				super(40569, data)
			end
		end
	end

	class UnderlyingPaymentStreamDelayIndicator < Quickfix::BoolField
		def UnderlyingPaymentStreamDelayIndicator.field
			return 40570
		end
		def initialize(data = nil)
			if( data == nil )
				super(40570)
			else
				super(40570, data)
			end
		end
	end

	class UnderlyingPaymentStreamSettlCurrency < Quickfix::StringField
		def UnderlyingPaymentStreamSettlCurrency.field
			return 40571
		end
		def initialize(data = nil)
			if( data == nil )
				super(40571)
			else
				super(40571, data)
			end
		end
	end

	class UnderlyingPaymentStreamDayCount < Quickfix::IntField
		def UnderlyingPaymentStreamDayCount.field
			return 40572
		end
		def initialize(data = nil)
			if( data == nil )
				super(40572)
			else
				super(40572, data)
			end
		end
	end

	class UnderlyingPaymentStreamAccrualDays < Quickfix::IntField
		def UnderlyingPaymentStreamAccrualDays.field
			return 40573
		end
		def initialize(data = nil)
			if( data == nil )
				super(40573)
			else
				super(40573, data)
			end
		end
	end

	class UnderlyingPaymentStreamDiscountType < Quickfix::IntField
		def UnderlyingPaymentStreamDiscountType.field
			return 40574
		end
		def initialize(data = nil)
			if( data == nil )
				super(40574)
			else
				super(40574, data)
			end
		end
	end

	class UnderlyingPaymentStreamDiscountRate < Quickfix::DoubleField
		def UnderlyingPaymentStreamDiscountRate.field
			return 40575
		end
		def initialize(data = nil)
			if( data == nil )
				super(40575)
			else
				super(40575, data)
			end
		end
	end

	class UnderlyingPaymentStreamDiscountRateDayCount < Quickfix::IntField
		def UnderlyingPaymentStreamDiscountRateDayCount.field
			return 40576
		end
		def initialize(data = nil)
			if( data == nil )
				super(40576)
			else
				super(40576, data)
			end
		end
	end

	class UnderlyingPaymentStreamCompoundingMethod < Quickfix::IntField
		def UnderlyingPaymentStreamCompoundingMethod.field
			return 40577
		end
		def initialize(data = nil)
			if( data == nil )
				super(40577)
			else
				super(40577, data)
			end
		end
	end

	class UnderlyingPaymentStreamInitialPrincipalExchangeIndicator < Quickfix::BoolField
		def UnderlyingPaymentStreamInitialPrincipalExchangeIndicator.field
			return 40578
		end
		def initialize(data = nil)
			if( data == nil )
				super(40578)
			else
				super(40578, data)
			end
		end
	end

	class UnderlyingPaymentStreamInterimPrincipalExchangeIndicator < Quickfix::BoolField
		def UnderlyingPaymentStreamInterimPrincipalExchangeIndicator.field
			return 40579
		end
		def initialize(data = nil)
			if( data == nil )
				super(40579)
			else
				super(40579, data)
			end
		end
	end

	class UnderlyingPaymentStreamFinalPrincipalExchangeIndicator < Quickfix::BoolField
		def UnderlyingPaymentStreamFinalPrincipalExchangeIndicator.field
			return 40580
		end
		def initialize(data = nil)
			if( data == nil )
				super(40580)
			else
				super(40580, data)
			end
		end
	end

	class UnderlyingPaymentStreamPaymentDateBusinessDayConvention < Quickfix::IntField
		def UnderlyingPaymentStreamPaymentDateBusinessDayConvention.field
			return 40581
		end
		def initialize(data = nil)
			if( data == nil )
				super(40581)
			else
				super(40581, data)
			end
		end
	end

	class UnderlyingPaymentStreamPaymentDateBusinessCenter < Quickfix::StringField
		def UnderlyingPaymentStreamPaymentDateBusinessCenter.field
			return 40582
		end
		def initialize(data = nil)
			if( data == nil )
				super(40582)
			else
				super(40582, data)
			end
		end
	end

	class UnderlyingPaymentStreamPaymentFrequencyPeriod < Quickfix::IntField
		def UnderlyingPaymentStreamPaymentFrequencyPeriod.field
			return 40583
		end
		def initialize(data = nil)
			if( data == nil )
				super(40583)
			else
				super(40583, data)
			end
		end
	end

	class UnderlyingPaymentStreamPaymentFrequencyUnit < Quickfix::StringField
		def UnderlyingPaymentStreamPaymentFrequencyUnit.field
			return 40584
		end
		def initialize(data = nil)
			if( data == nil )
				super(40584)
			else
				super(40584, data)
			end
		end
	end

	class UnderlyingPaymentStreamPaymentRollConvention < Quickfix::StringField
		def UnderlyingPaymentStreamPaymentRollConvention.field
			return 40585
		end
		def initialize(data = nil)
			if( data == nil )
				super(40585)
			else
				super(40585, data)
			end
		end
	end

	class UnderlyingPaymentStreamFirstPaymentDateUnadjusted < Quickfix::StringField
		def UnderlyingPaymentStreamFirstPaymentDateUnadjusted.field
			return 40586
		end
		def initialize(data = nil)
			if( data == nil )
				super(40586)
			else
				super(40586, data)
			end
		end
	end

	class UnderlyingPaymentStreamLastRegularPaymentDateUnadjusted < Quickfix::StringField
		def UnderlyingPaymentStreamLastRegularPaymentDateUnadjusted.field
			return 40587
		end
		def initialize(data = nil)
			if( data == nil )
				super(40587)
			else
				super(40587, data)
			end
		end
	end

	class UnderlyingPaymentStreamPaymentDateRelativeTo < Quickfix::IntField
		def UnderlyingPaymentStreamPaymentDateRelativeTo.field
			return 40588
		end
		def initialize(data = nil)
			if( data == nil )
				super(40588)
			else
				super(40588, data)
			end
		end
	end

	class UnderlyingPaymentStreamPaymentDateOffsetPeriod < Quickfix::IntField
		def UnderlyingPaymentStreamPaymentDateOffsetPeriod.field
			return 40589
		end
		def initialize(data = nil)
			if( data == nil )
				super(40589)
			else
				super(40589, data)
			end
		end
	end

	class UnderlyingPaymentStreamPaymentDateOffsetUnit < Quickfix::StringField
		def UnderlyingPaymentStreamPaymentDateOffsetUnit.field
			return 40590
		end
		def initialize(data = nil)
			if( data == nil )
				super(40590)
			else
				super(40590, data)
			end
		end
	end

	class UnderlyingPaymentStreamPaymentDateOffsetDayType < Quickfix::IntField
		def UnderlyingPaymentStreamPaymentDateOffsetDayType.field
			return 40591
		end
		def initialize(data = nil)
			if( data == nil )
				super(40591)
			else
				super(40591, data)
			end
		end
	end

	class UnderlyingPaymentStreamResetDateRelativeTo < Quickfix::IntField
		def UnderlyingPaymentStreamResetDateRelativeTo.field
			return 40592
		end
		def initialize(data = nil)
			if( data == nil )
				super(40592)
			else
				super(40592, data)
			end
		end
	end

	class UnderlyingPaymentStreamResetDateBusinessDayConvention < Quickfix::IntField
		def UnderlyingPaymentStreamResetDateBusinessDayConvention.field
			return 40593
		end
		def initialize(data = nil)
			if( data == nil )
				super(40593)
			else
				super(40593, data)
			end
		end
	end

	class UnderlyingPaymentStreamResetDateBusinessCenter < Quickfix::StringField
		def UnderlyingPaymentStreamResetDateBusinessCenter.field
			return 40594
		end
		def initialize(data = nil)
			if( data == nil )
				super(40594)
			else
				super(40594, data)
			end
		end
	end

	class UnderlyingPaymentStreamResetFrequencyPeriod < Quickfix::IntField
		def UnderlyingPaymentStreamResetFrequencyPeriod.field
			return 40595
		end
		def initialize(data = nil)
			if( data == nil )
				super(40595)
			else
				super(40595, data)
			end
		end
	end

	class UnderlyingPaymentStreamResetFrequencyUnit < Quickfix::StringField
		def UnderlyingPaymentStreamResetFrequencyUnit.field
			return 40596
		end
		def initialize(data = nil)
			if( data == nil )
				super(40596)
			else
				super(40596, data)
			end
		end
	end

	class UnderlyingPaymentStreamResetWeeklyRollConvention < Quickfix::StringField
		def UnderlyingPaymentStreamResetWeeklyRollConvention.field
			return 40597
		end
		def initialize(data = nil)
			if( data == nil )
				super(40597)
			else
				super(40597, data)
			end
		end
	end

	class UnderlyingPaymentStreamInitialFixingDateRelativeTo < Quickfix::IntField
		def UnderlyingPaymentStreamInitialFixingDateRelativeTo.field
			return 40598
		end
		def initialize(data = nil)
			if( data == nil )
				super(40598)
			else
				super(40598, data)
			end
		end
	end

	class UnderlyingPaymentStreamInitialFixingDateBusinessDayConvention < Quickfix::IntField
		def UnderlyingPaymentStreamInitialFixingDateBusinessDayConvention.field
			return 40599
		end
		def initialize(data = nil)
			if( data == nil )
				super(40599)
			else
				super(40599, data)
			end
		end
	end

	class UnderlyingPaymentStreamInitialFixingDateBusinessCenter < Quickfix::StringField
		def UnderlyingPaymentStreamInitialFixingDateBusinessCenter.field
			return 40600
		end
		def initialize(data = nil)
			if( data == nil )
				super(40600)
			else
				super(40600, data)
			end
		end
	end

	class UnderlyingPaymentStreamInitialFixingDateOffsetPeriod < Quickfix::IntField
		def UnderlyingPaymentStreamInitialFixingDateOffsetPeriod.field
			return 40601
		end
		def initialize(data = nil)
			if( data == nil )
				super(40601)
			else
				super(40601, data)
			end
		end
	end

	class UnderlyingPaymentStreamInitialFixingDateOffsetUnit < Quickfix::StringField
		def UnderlyingPaymentStreamInitialFixingDateOffsetUnit.field
			return 40602
		end
		def initialize(data = nil)
			if( data == nil )
				super(40602)
			else
				super(40602, data)
			end
		end
	end

	class UnderlyingPaymentStreamInitialFixingDateOffsetDayType < Quickfix::IntField
		def UnderlyingPaymentStreamInitialFixingDateOffsetDayType.field
			return 40603
		end
		def initialize(data = nil)
			if( data == nil )
				super(40603)
			else
				super(40603, data)
			end
		end
	end

	class UnderlyingPaymentStreamInitialFixingDateAdjusted < Quickfix::StringField
		def UnderlyingPaymentStreamInitialFixingDateAdjusted.field
			return 40604
		end
		def initialize(data = nil)
			if( data == nil )
				super(40604)
			else
				super(40604, data)
			end
		end
	end

	class UnderlyingPaymentStreamFixingDateRelativeTo < Quickfix::IntField
		def UnderlyingPaymentStreamFixingDateRelativeTo.field
			return 40605
		end
		def initialize(data = nil)
			if( data == nil )
				super(40605)
			else
				super(40605, data)
			end
		end
	end

	class UnderlyingPaymentStreamFixingDateBusinessDayConvention < Quickfix::IntField
		def UnderlyingPaymentStreamFixingDateBusinessDayConvention.field
			return 40606
		end
		def initialize(data = nil)
			if( data == nil )
				super(40606)
			else
				super(40606, data)
			end
		end
	end

	class UnderlyingPaymentStreamFixingDateBusinessCenter < Quickfix::StringField
		def UnderlyingPaymentStreamFixingDateBusinessCenter.field
			return 40607
		end
		def initialize(data = nil)
			if( data == nil )
				super(40607)
			else
				super(40607, data)
			end
		end
	end

	class UnderlyingPaymentStreamFixingDateOffsetPeriod < Quickfix::IntField
		def UnderlyingPaymentStreamFixingDateOffsetPeriod.field
			return 40608
		end
		def initialize(data = nil)
			if( data == nil )
				super(40608)
			else
				super(40608, data)
			end
		end
	end

	class UnderlyingPaymentStreamFixingDateOffsetUnit < Quickfix::StringField
		def UnderlyingPaymentStreamFixingDateOffsetUnit.field
			return 40609
		end
		def initialize(data = nil)
			if( data == nil )
				super(40609)
			else
				super(40609, data)
			end
		end
	end

	class UnderlyingPaymentStreamFixingDateOffsetDayType < Quickfix::IntField
		def UnderlyingPaymentStreamFixingDateOffsetDayType.field
			return 40610
		end
		def initialize(data = nil)
			if( data == nil )
				super(40610)
			else
				super(40610, data)
			end
		end
	end

	class UnderlyingPaymentStreamFixingDateAdjusted < Quickfix::StringField
		def UnderlyingPaymentStreamFixingDateAdjusted.field
			return 40611
		end
		def initialize(data = nil)
			if( data == nil )
				super(40611)
			else
				super(40611, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateCutoffDateOffsetPeriod < Quickfix::IntField
		def UnderlyingPaymentStreamRateCutoffDateOffsetPeriod.field
			return 40612
		end
		def initialize(data = nil)
			if( data == nil )
				super(40612)
			else
				super(40612, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateCutoffDateOffsetUnit < Quickfix::StringField
		def UnderlyingPaymentStreamRateCutoffDateOffsetUnit.field
			return 40613
		end
		def initialize(data = nil)
			if( data == nil )
				super(40613)
			else
				super(40613, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateCutoffDateOffsetDayType < Quickfix::IntField
		def UnderlyingPaymentStreamRateCutoffDateOffsetDayType.field
			return 40614
		end
		def initialize(data = nil)
			if( data == nil )
				super(40614)
			else
				super(40614, data)
			end
		end
	end

	class UnderlyingPaymentStreamRate < Quickfix::DoubleField
		def UnderlyingPaymentStreamRate.field
			return 40615
		end
		def initialize(data = nil)
			if( data == nil )
				super(40615)
			else
				super(40615, data)
			end
		end
	end

	class UnderlyingPaymentStreamFixedAmount < Quickfix::DoubleField
		def UnderlyingPaymentStreamFixedAmount.field
			return 40616
		end
		def initialize(data = nil)
			if( data == nil )
				super(40616)
			else
				super(40616, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateOrAmountCurrency < Quickfix::StringField
		def UnderlyingPaymentStreamRateOrAmountCurrency.field
			return 40617
		end
		def initialize(data = nil)
			if( data == nil )
				super(40617)
			else
				super(40617, data)
			end
		end
	end

	class UnderlyingPaymentStreamFutureValueNotional < Quickfix::DoubleField
		def UnderlyingPaymentStreamFutureValueNotional.field
			return 40618
		end
		def initialize(data = nil)
			if( data == nil )
				super(40618)
			else
				super(40618, data)
			end
		end
	end

	class UnderlyingPaymentStreamFutureValueDateAdjusted < Quickfix::StringField
		def UnderlyingPaymentStreamFutureValueDateAdjusted.field
			return 40619
		end
		def initialize(data = nil)
			if( data == nil )
				super(40619)
			else
				super(40619, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateIndex < Quickfix::StringField
		def UnderlyingPaymentStreamRateIndex.field
			return 40620
		end
		def initialize(data = nil)
			if( data == nil )
				super(40620)
			else
				super(40620, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateIndexSource < Quickfix::IntField
		def UnderlyingPaymentStreamRateIndexSource.field
			return 40621
		end
		def initialize(data = nil)
			if( data == nil )
				super(40621)
			else
				super(40621, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateIndexCurveUnit < Quickfix::StringField
		def UnderlyingPaymentStreamRateIndexCurveUnit.field
			return 40622
		end
		def initialize(data = nil)
			if( data == nil )
				super(40622)
			else
				super(40622, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateIndexCurvePeriod < Quickfix::IntField
		def UnderlyingPaymentStreamRateIndexCurvePeriod.field
			return 40623
		end
		def initialize(data = nil)
			if( data == nil )
				super(40623)
			else
				super(40623, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateMultiplier < Quickfix::DoubleField
		def UnderlyingPaymentStreamRateMultiplier.field
			return 40624
		end
		def initialize(data = nil)
			if( data == nil )
				super(40624)
			else
				super(40624, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateSpread < Quickfix::DoubleField
		def UnderlyingPaymentStreamRateSpread.field
			return 40625
		end
		def initialize(data = nil)
			if( data == nil )
				super(40625)
			else
				super(40625, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateSpreadPositionType < Quickfix::IntField
		def UnderlyingPaymentStreamRateSpreadPositionType.field
			return 40626
		end
		def initialize(data = nil)
			if( data == nil )
				super(40626)
			else
				super(40626, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateTreatment < Quickfix::IntField
		def UnderlyingPaymentStreamRateTreatment.field
			return 40627
		end
		def initialize(data = nil)
			if( data == nil )
				super(40627)
			else
				super(40627, data)
			end
		end
	end

	class UnderlyingPaymentStreamCapRate < Quickfix::DoubleField
		def UnderlyingPaymentStreamCapRate.field
			return 40628
		end
		def initialize(data = nil)
			if( data == nil )
				super(40628)
			else
				super(40628, data)
			end
		end
	end

	class UnderlyingPaymentStreamCapRateBuySide < Quickfix::IntField
		def UnderlyingPaymentStreamCapRateBuySide.field
			return 40629
		end
		def initialize(data = nil)
			if( data == nil )
				super(40629)
			else
				super(40629, data)
			end
		end
	end

	class UnderlyingPaymentStreamCapRateSellSide < Quickfix::IntField
		def UnderlyingPaymentStreamCapRateSellSide.field
			return 40630
		end
		def initialize(data = nil)
			if( data == nil )
				super(40630)
			else
				super(40630, data)
			end
		end
	end

	class UnderlyingPaymentStreamFloorRate < Quickfix::DoubleField
		def UnderlyingPaymentStreamFloorRate.field
			return 40631
		end
		def initialize(data = nil)
			if( data == nil )
				super(40631)
			else
				super(40631, data)
			end
		end
	end

	class UnderlyingPaymentStreamFloorRateBuySide < Quickfix::IntField
		def UnderlyingPaymentStreamFloorRateBuySide.field
			return 40632
		end
		def initialize(data = nil)
			if( data == nil )
				super(40632)
			else
				super(40632, data)
			end
		end
	end

	class UnderlyingPaymentStreamFloorRateSellSide < Quickfix::IntField
		def UnderlyingPaymentStreamFloorRateSellSide.field
			return 40633
		end
		def initialize(data = nil)
			if( data == nil )
				super(40633)
			else
				super(40633, data)
			end
		end
	end

	class UnderlyingPaymentStreamInitialRate < Quickfix::DoubleField
		def UnderlyingPaymentStreamInitialRate.field
			return 40634
		end
		def initialize(data = nil)
			if( data == nil )
				super(40634)
			else
				super(40634, data)
			end
		end
	end

	class UnderlyingPaymentStreamFinalRateRoundingDirection < Quickfix::CharField
		def UnderlyingPaymentStreamFinalRateRoundingDirection.field
			return 40635
		end
		def initialize(data = nil)
			if( data == nil )
				super(40635)
			else
				super(40635, data)
			end
		end
	end

	class UnderlyingPaymentStreamFinalRatePrecision < Quickfix::IntField
		def UnderlyingPaymentStreamFinalRatePrecision.field
			return 40636
		end
		def initialize(data = nil)
			if( data == nil )
				super(40636)
			else
				super(40636, data)
			end
		end
	end

	class UnderlyingPaymentStreamAveragingMethod < Quickfix::IntField
		def UnderlyingPaymentStreamAveragingMethod.field
			return 40637
		end
		def initialize(data = nil)
			if( data == nil )
				super(40637)
			else
				super(40637, data)
			end
		end
	end

	class UnderlyingPaymentStreamNegativeRateTreatment < Quickfix::IntField
		def UnderlyingPaymentStreamNegativeRateTreatment.field
			return 40638
		end
		def initialize(data = nil)
			if( data == nil )
				super(40638)
			else
				super(40638, data)
			end
		end
	end

	class UnderlyingPaymentStreamInflationLagPeriod < Quickfix::IntField
		def UnderlyingPaymentStreamInflationLagPeriod.field
			return 40639
		end
		def initialize(data = nil)
			if( data == nil )
				super(40639)
			else
				super(40639, data)
			end
		end
	end

	class UnderlyingPaymentStreamInflationLagUnit < Quickfix::StringField
		def UnderlyingPaymentStreamInflationLagUnit.field
			return 40640
		end
		def initialize(data = nil)
			if( data == nil )
				super(40640)
			else
				super(40640, data)
			end
		end
	end

	class UnderlyingPaymentStreamInflationLagDayType < Quickfix::IntField
		def UnderlyingPaymentStreamInflationLagDayType.field
			return 40641
		end
		def initialize(data = nil)
			if( data == nil )
				super(40641)
			else
				super(40641, data)
			end
		end
	end

	class UnderlyingPaymentStreamInflationInterpolationMethod < Quickfix::IntField
		def UnderlyingPaymentStreamInflationInterpolationMethod.field
			return 40642
		end
		def initialize(data = nil)
			if( data == nil )
				super(40642)
			else
				super(40642, data)
			end
		end
	end

	class UnderlyingPaymentStreamInflationIndexSource < Quickfix::IntField
		def UnderlyingPaymentStreamInflationIndexSource.field
			return 40643
		end
		def initialize(data = nil)
			if( data == nil )
				super(40643)
			else
				super(40643, data)
			end
		end
	end

	class UnderlyingPaymentStreamInflationPublicationSource < Quickfix::StringField
		def UnderlyingPaymentStreamInflationPublicationSource.field
			return 40644
		end
		def initialize(data = nil)
			if( data == nil )
				super(40644)
			else
				super(40644, data)
			end
		end
	end

	class UnderlyingPaymentStreamInflationInitialIndexLevel < Quickfix::DoubleField
		def UnderlyingPaymentStreamInflationInitialIndexLevel.field
			return 40645
		end
		def initialize(data = nil)
			if( data == nil )
				super(40645)
			else
				super(40645, data)
			end
		end
	end

	class UnderlyingPaymentStreamInflationFallbackBondApplicable < Quickfix::BoolField
		def UnderlyingPaymentStreamInflationFallbackBondApplicable.field
			return 40646
		end
		def initialize(data = nil)
			if( data == nil )
				super(40646)
			else
				super(40646, data)
			end
		end
	end

	class UnderlyingPaymentStreamFRADiscounting < Quickfix::IntField
		def UnderlyingPaymentStreamFRADiscounting.field
			return 40647
		end
		def initialize(data = nil)
			if( data == nil )
				super(40647)
			else
				super(40647, data)
			end
		end
	end

	class UnderlyingPaymentStreamNonDeliverableRefCurrency < Quickfix::StringField
		def UnderlyingPaymentStreamNonDeliverableRefCurrency.field
			return 40648
		end
		def initialize(data = nil)
			if( data == nil )
				super(40648)
			else
				super(40648, data)
			end
		end
	end

	class UnderlyingPaymentStreamNonDeliverableFixingDatesBizDayConvention < Quickfix::IntField
		def UnderlyingPaymentStreamNonDeliverableFixingDatesBizDayConvention.field
			return 40649
		end
		def initialize(data = nil)
			if( data == nil )
				super(40649)
			else
				super(40649, data)
			end
		end
	end

	class UnderlyingPaymentStreamNonDeliverableFixingDatesBusinessCenter < Quickfix::StringField
		def UnderlyingPaymentStreamNonDeliverableFixingDatesBusinessCenter.field
			return 40650
		end
		def initialize(data = nil)
			if( data == nil )
				super(40650)
			else
				super(40650, data)
			end
		end
	end

	class UnderlyingPaymentStreamNonDeliverableFixingDatesRelativeTo < Quickfix::IntField
		def UnderlyingPaymentStreamNonDeliverableFixingDatesRelativeTo.field
			return 40651
		end
		def initialize(data = nil)
			if( data == nil )
				super(40651)
			else
				super(40651, data)
			end
		end
	end

	class UnderlyingPaymentStreamNonDeliverableFixingDatesOffsetPeriod < Quickfix::IntField
		def UnderlyingPaymentStreamNonDeliverableFixingDatesOffsetPeriod.field
			return 40652
		end
		def initialize(data = nil)
			if( data == nil )
				super(40652)
			else
				super(40652, data)
			end
		end
	end

	class UnderlyingPaymentStreamNonDeliverableFixingDatesOffsetUnit < Quickfix::StringField
		def UnderlyingPaymentStreamNonDeliverableFixingDatesOffsetUnit.field
			return 40653
		end
		def initialize(data = nil)
			if( data == nil )
				super(40653)
			else
				super(40653, data)
			end
		end
	end

	class UnderlyingPaymentStreamNonDeliverableFixingDatesOffsetDayType < Quickfix::IntField
		def UnderlyingPaymentStreamNonDeliverableFixingDatesOffsetDayType.field
			return 40654
		end
		def initialize(data = nil)
			if( data == nil )
				super(40654)
			else
				super(40654, data)
			end
		end
	end

	class SettlRateFallbackReferencePage < Quickfix::StringField
		def SettlRateFallbackReferencePage.field
			return 40655
		end
		def initialize(data = nil)
			if( data == nil )
				super(40655)
			else
				super(40655, data)
			end
		end
	end

	class NoUnderlyingNonDeliverableFixingDates < Quickfix::IntField
		def NoUnderlyingNonDeliverableFixingDates.field
			return 40656
		end
		def initialize(data = nil)
			if( data == nil )
				super(40656)
			else
				super(40656, data)
			end
		end
	end

	class UnderlyingNonDeliverableFixingDate < Quickfix::StringField
		def UnderlyingNonDeliverableFixingDate.field
			return 40657
		end
		def initialize(data = nil)
			if( data == nil )
				super(40657)
			else
				super(40657, data)
			end
		end
	end

	class UnderlyingNonDeliverableFixingDateType < Quickfix::IntField
		def UnderlyingNonDeliverableFixingDateType.field
			return 40658
		end
		def initialize(data = nil)
			if( data == nil )
				super(40658)
			else
				super(40658, data)
			end
		end
	end

	class NoUnderlyingSettlRateFallbacks < Quickfix::IntField
		def NoUnderlyingSettlRateFallbacks.field
			return 40659
		end
		def initialize(data = nil)
			if( data == nil )
				super(40659)
			else
				super(40659, data)
			end
		end
	end

	class UnderlyingSettlRatePostponementMaximumDays < Quickfix::IntField
		def UnderlyingSettlRatePostponementMaximumDays.field
			return 40660
		end
		def initialize(data = nil)
			if( data == nil )
				super(40660)
			else
				super(40660, data)
			end
		end
	end

	class UnderlyingPaymentStreamNonDeliverableSettlRateSource < Quickfix::IntField
		def UnderlyingPaymentStreamNonDeliverableSettlRateSource.field
			return 40661
		end
		def initialize(data = nil)
			if( data == nil )
				super(40661)
			else
				super(40661, data)
			end
		end
	end

	class UnderlyingSettlRatePostponementSurvey < Quickfix::BoolField
		def UnderlyingSettlRatePostponementSurvey.field
			return 40662
		end
		def initialize(data = nil)
			if( data == nil )
				super(40662)
			else
				super(40662, data)
			end
		end
	end

	class UnderlyingSettlRatePostponementCalculationAgent < Quickfix::IntField
		def UnderlyingSettlRatePostponementCalculationAgent.field
			return 40663
		end
		def initialize(data = nil)
			if( data == nil )
				super(40663)
			else
				super(40663, data)
			end
		end
	end

	class NoUnderlyingPaymentSchedules < Quickfix::IntField
		def NoUnderlyingPaymentSchedules.field
			return 40664
		end
		def initialize(data = nil)
			if( data == nil )
				super(40664)
			else
				super(40664, data)
			end
		end
	end

	class UnderlyingPaymentScheduleType < Quickfix::IntField
		def UnderlyingPaymentScheduleType.field
			return 40665
		end
		def initialize(data = nil)
			if( data == nil )
				super(40665)
			else
				super(40665, data)
			end
		end
	end

	class UnderlyingPaymentScheduleStubType < Quickfix::IntField
		def UnderlyingPaymentScheduleStubType.field
			return 40666
		end
		def initialize(data = nil)
			if( data == nil )
				super(40666)
			else
				super(40666, data)
			end
		end
	end

	class UnderlyingPaymentScheduleStartDateUnadjusted < Quickfix::StringField
		def UnderlyingPaymentScheduleStartDateUnadjusted.field
			return 40667
		end
		def initialize(data = nil)
			if( data == nil )
				super(40667)
			else
				super(40667, data)
			end
		end
	end

	class UnderlyingPaymentScheduleEndDateUnadjusted < Quickfix::StringField
		def UnderlyingPaymentScheduleEndDateUnadjusted.field
			return 40668
		end
		def initialize(data = nil)
			if( data == nil )
				super(40668)
			else
				super(40668, data)
			end
		end
	end

	class UnderlyingPaymentSchedulePaySide < Quickfix::IntField
		def UnderlyingPaymentSchedulePaySide.field
			return 40669
		end
		def initialize(data = nil)
			if( data == nil )
				super(40669)
			else
				super(40669, data)
			end
		end
	end

	class UnderlyingPaymentScheduleReceiveSide < Quickfix::IntField
		def UnderlyingPaymentScheduleReceiveSide.field
			return 40670
		end
		def initialize(data = nil)
			if( data == nil )
				super(40670)
			else
				super(40670, data)
			end
		end
	end

	class UnderlyingPaymentScheduleNotional < Quickfix::DoubleField
		def UnderlyingPaymentScheduleNotional.field
			return 40671
		end
		def initialize(data = nil)
			if( data == nil )
				super(40671)
			else
				super(40671, data)
			end
		end
	end

	class UnderlyingPaymentScheduleCurrency < Quickfix::StringField
		def UnderlyingPaymentScheduleCurrency.field
			return 40672
		end
		def initialize(data = nil)
			if( data == nil )
				super(40672)
			else
				super(40672, data)
			end
		end
	end

	class UnderlyingPaymentScheduleRate < Quickfix::DoubleField
		def UnderlyingPaymentScheduleRate.field
			return 40673
		end
		def initialize(data = nil)
			if( data == nil )
				super(40673)
			else
				super(40673, data)
			end
		end
	end

	class UnderlyingPaymentScheduleRateMultiplier < Quickfix::DoubleField
		def UnderlyingPaymentScheduleRateMultiplier.field
			return 40674
		end
		def initialize(data = nil)
			if( data == nil )
				super(40674)
			else
				super(40674, data)
			end
		end
	end

	class UnderlyingPaymentScheduleRateSpread < Quickfix::DoubleField
		def UnderlyingPaymentScheduleRateSpread.field
			return 40675
		end
		def initialize(data = nil)
			if( data == nil )
				super(40675)
			else
				super(40675, data)
			end
		end
	end

	class UnderlyingPaymentScheduleRateSpreadPositionType < Quickfix::IntField
		def UnderlyingPaymentScheduleRateSpreadPositionType.field
			return 40676
		end
		def initialize(data = nil)
			if( data == nil )
				super(40676)
			else
				super(40676, data)
			end
		end
	end

	class UnderlyingPaymentScheduleRateTreatment < Quickfix::IntField
		def UnderlyingPaymentScheduleRateTreatment.field
			return 40677
		end
		def initialize(data = nil)
			if( data == nil )
				super(40677)
			else
				super(40677, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixedAmount < Quickfix::DoubleField
		def UnderlyingPaymentScheduleFixedAmount.field
			return 40678
		end
		def initialize(data = nil)
			if( data == nil )
				super(40678)
			else
				super(40678, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixedCurrency < Quickfix::StringField
		def UnderlyingPaymentScheduleFixedCurrency.field
			return 40679
		end
		def initialize(data = nil)
			if( data == nil )
				super(40679)
			else
				super(40679, data)
			end
		end
	end

	class UnderlyingPaymentScheduleStepFrequencyPeriod < Quickfix::IntField
		def UnderlyingPaymentScheduleStepFrequencyPeriod.field
			return 40680
		end
		def initialize(data = nil)
			if( data == nil )
				super(40680)
			else
				super(40680, data)
			end
		end
	end

	class UnderlyingPaymentScheduleStepFrequencyUnit < Quickfix::StringField
		def UnderlyingPaymentScheduleStepFrequencyUnit.field
			return 40681
		end
		def initialize(data = nil)
			if( data == nil )
				super(40681)
			else
				super(40681, data)
			end
		end
	end

	class UnderlyingPaymentScheduleStepOffsetValue < Quickfix::DoubleField
		def UnderlyingPaymentScheduleStepOffsetValue.field
			return 40682
		end
		def initialize(data = nil)
			if( data == nil )
				super(40682)
			else
				super(40682, data)
			end
		end
	end

	class UnderlyingPaymentScheduleStepRate < Quickfix::DoubleField
		def UnderlyingPaymentScheduleStepRate.field
			return 40683
		end
		def initialize(data = nil)
			if( data == nil )
				super(40683)
			else
				super(40683, data)
			end
		end
	end

	class UnderlyingPaymentScheduleStepOffsetRate < Quickfix::DoubleField
		def UnderlyingPaymentScheduleStepOffsetRate.field
			return 40684
		end
		def initialize(data = nil)
			if( data == nil )
				super(40684)
			else
				super(40684, data)
			end
		end
	end

	class UnderlyingPaymentScheduleStepRelativeTo < Quickfix::IntField
		def UnderlyingPaymentScheduleStepRelativeTo.field
			return 40685
		end
		def initialize(data = nil)
			if( data == nil )
				super(40685)
			else
				super(40685, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingDateUnadjusted < Quickfix::StringField
		def UnderlyingPaymentScheduleFixingDateUnadjusted.field
			return 40686
		end
		def initialize(data = nil)
			if( data == nil )
				super(40686)
			else
				super(40686, data)
			end
		end
	end

	class UnderlyingPaymentScheduleWeight < Quickfix::DoubleField
		def UnderlyingPaymentScheduleWeight.field
			return 40687
		end
		def initialize(data = nil)
			if( data == nil )
				super(40687)
			else
				super(40687, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingDateRelativeTo < Quickfix::IntField
		def UnderlyingPaymentScheduleFixingDateRelativeTo.field
			return 40688
		end
		def initialize(data = nil)
			if( data == nil )
				super(40688)
			else
				super(40688, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingDateBusinessDayCnvtn < Quickfix::IntField
		def UnderlyingPaymentScheduleFixingDateBusinessDayCnvtn.field
			return 40689
		end
		def initialize(data = nil)
			if( data == nil )
				super(40689)
			else
				super(40689, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingDateBusinessCenter < Quickfix::StringField
		def UnderlyingPaymentScheduleFixingDateBusinessCenter.field
			return 40690
		end
		def initialize(data = nil)
			if( data == nil )
				super(40690)
			else
				super(40690, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingDateOffsetPeriod < Quickfix::IntField
		def UnderlyingPaymentScheduleFixingDateOffsetPeriod.field
			return 40691
		end
		def initialize(data = nil)
			if( data == nil )
				super(40691)
			else
				super(40691, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingDateOffsetUnit < Quickfix::StringField
		def UnderlyingPaymentScheduleFixingDateOffsetUnit.field
			return 40692
		end
		def initialize(data = nil)
			if( data == nil )
				super(40692)
			else
				super(40692, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingDateOffsetDayType < Quickfix::IntField
		def UnderlyingPaymentScheduleFixingDateOffsetDayType.field
			return 40693
		end
		def initialize(data = nil)
			if( data == nil )
				super(40693)
			else
				super(40693, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingDateAdjusted < Quickfix::StringField
		def UnderlyingPaymentScheduleFixingDateAdjusted.field
			return 40694
		end
		def initialize(data = nil)
			if( data == nil )
				super(40694)
			else
				super(40694, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingTime < Quickfix::StringField
		def UnderlyingPaymentScheduleFixingTime.field
			return 40695
		end
		def initialize(data = nil)
			if( data == nil )
				super(40695)
			else
				super(40695, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingTimeBusinessCenter < Quickfix::StringField
		def UnderlyingPaymentScheduleFixingTimeBusinessCenter.field
			return 40696
		end
		def initialize(data = nil)
			if( data == nil )
				super(40696)
			else
				super(40696, data)
			end
		end
	end

	class UnderlyingPaymentScheduleInterimExchangePaymentDateRelativeTo < Quickfix::IntField
		def UnderlyingPaymentScheduleInterimExchangePaymentDateRelativeTo.field
			return 40697
		end
		def initialize(data = nil)
			if( data == nil )
				super(40697)
			else
				super(40697, data)
			end
		end
	end

	class UnderlyingPaymentScheduleInterimExchangeDatesBizDayConvention < Quickfix::IntField
		def UnderlyingPaymentScheduleInterimExchangeDatesBizDayConvention.field
			return 40698
		end
		def initialize(data = nil)
			if( data == nil )
				super(40698)
			else
				super(40698, data)
			end
		end
	end

	class UnderlyingPaymentScheduleInterimExchangeDatesBusinessCenter < Quickfix::StringField
		def UnderlyingPaymentScheduleInterimExchangeDatesBusinessCenter.field
			return 40699
		end
		def initialize(data = nil)
			if( data == nil )
				super(40699)
			else
				super(40699, data)
			end
		end
	end

	class UnderlyingPaymentScheduleInterimExchangeDatesOffsetPeriod < Quickfix::IntField
		def UnderlyingPaymentScheduleInterimExchangeDatesOffsetPeriod.field
			return 40700
		end
		def initialize(data = nil)
			if( data == nil )
				super(40700)
			else
				super(40700, data)
			end
		end
	end

	class UnderlyingPaymentScheduleInterimExchangeDatesOffsetUnit < Quickfix::StringField
		def UnderlyingPaymentScheduleInterimExchangeDatesOffsetUnit.field
			return 40701
		end
		def initialize(data = nil)
			if( data == nil )
				super(40701)
			else
				super(40701, data)
			end
		end
	end

	class UnderlyingPaymentScheduleInterimExchangeDatesOffsetDayType < Quickfix::IntField
		def UnderlyingPaymentScheduleInterimExchangeDatesOffsetDayType.field
			return 40702
		end
		def initialize(data = nil)
			if( data == nil )
				super(40702)
			else
				super(40702, data)
			end
		end
	end

	class UnderlyingPaymentScheduleInterimExchangeDateAdjusted < Quickfix::StringField
		def UnderlyingPaymentScheduleInterimExchangeDateAdjusted.field
			return 40703
		end
		def initialize(data = nil)
			if( data == nil )
				super(40703)
			else
				super(40703, data)
			end
		end
	end

	class NoUnderlyingPaymentScheduleRateSources < Quickfix::IntField
		def NoUnderlyingPaymentScheduleRateSources.field
			return 40704
		end
		def initialize(data = nil)
			if( data == nil )
				super(40704)
			else
				super(40704, data)
			end
		end
	end

	class UnderlyingPaymentScheduleRateSource < Quickfix::IntField
		def UnderlyingPaymentScheduleRateSource.field
			return 40705
		end
		def initialize(data = nil)
			if( data == nil )
				super(40705)
			else
				super(40705, data)
			end
		end
	end

	class UnderlyingPaymentScheduleRateSourceType < Quickfix::IntField
		def UnderlyingPaymentScheduleRateSourceType.field
			return 40706
		end
		def initialize(data = nil)
			if( data == nil )
				super(40706)
			else
				super(40706, data)
			end
		end
	end

	class UnderlyingPaymentScheduleReferencePage < Quickfix::StringField
		def UnderlyingPaymentScheduleReferencePage.field
			return 40707
		end
		def initialize(data = nil)
			if( data == nil )
				super(40707)
			else
				super(40707, data)
			end
		end
	end

	class NoUnderlyingPaymentStubs < Quickfix::IntField
		def NoUnderlyingPaymentStubs.field
			return 40708
		end
		def initialize(data = nil)
			if( data == nil )
				super(40708)
			else
				super(40708, data)
			end
		end
	end

	class UnderlyingPaymentStubType < Quickfix::IntField
		def UnderlyingPaymentStubType.field
			return 40709
		end
		def initialize(data = nil)
			if( data == nil )
				super(40709)
			else
				super(40709, data)
			end
		end
	end

	class UnderlyingPaymentStubLength < Quickfix::IntField
		def UnderlyingPaymentStubLength.field
			return 40710
		end
		def initialize(data = nil)
			if( data == nil )
				super(40710)
			else
				super(40710, data)
			end
		end
	end

	class UnderlyingPaymentStubRate < Quickfix::DoubleField
		def UnderlyingPaymentStubRate.field
			return 40711
		end
		def initialize(data = nil)
			if( data == nil )
				super(40711)
			else
				super(40711, data)
			end
		end
	end

	class UnderlyingPaymentStubFixedAmount < Quickfix::DoubleField
		def UnderlyingPaymentStubFixedAmount.field
			return 40712
		end
		def initialize(data = nil)
			if( data == nil )
				super(40712)
			else
				super(40712, data)
			end
		end
	end

	class UnderlyingPaymentStubFixedCurrency < Quickfix::StringField
		def UnderlyingPaymentStubFixedCurrency.field
			return 40713
		end
		def initialize(data = nil)
			if( data == nil )
				super(40713)
			else
				super(40713, data)
			end
		end
	end

	class UnderlyingPaymentStubIndex < Quickfix::StringField
		def UnderlyingPaymentStubIndex.field
			return 40714
		end
		def initialize(data = nil)
			if( data == nil )
				super(40714)
			else
				super(40714, data)
			end
		end
	end

	class UnderlyingPaymentStubIndexSource < Quickfix::IntField
		def UnderlyingPaymentStubIndexSource.field
			return 40715
		end
		def initialize(data = nil)
			if( data == nil )
				super(40715)
			else
				super(40715, data)
			end
		end
	end

	class UnderlyingPaymentStubIndexCurvePeriod < Quickfix::IntField
		def UnderlyingPaymentStubIndexCurvePeriod.field
			return 40716
		end
		def initialize(data = nil)
			if( data == nil )
				super(40716)
			else
				super(40716, data)
			end
		end
	end

	class UnderlyingPaymentStubIndexCurveUnit < Quickfix::StringField
		def UnderlyingPaymentStubIndexCurveUnit.field
			return 40717
		end
		def initialize(data = nil)
			if( data == nil )
				super(40717)
			else
				super(40717, data)
			end
		end
	end

	class UnderlyingPaymentStubIndexRateMultiplier < Quickfix::DoubleField
		def UnderlyingPaymentStubIndexRateMultiplier.field
			return 40718
		end
		def initialize(data = nil)
			if( data == nil )
				super(40718)
			else
				super(40718, data)
			end
		end
	end

	class UnderlyingPaymentStubIndexRateSpread < Quickfix::DoubleField
		def UnderlyingPaymentStubIndexRateSpread.field
			return 40719
		end
		def initialize(data = nil)
			if( data == nil )
				super(40719)
			else
				super(40719, data)
			end
		end
	end

	class UnderlyingPaymentStubIndexRateSpreadPositionType < Quickfix::IntField
		def UnderlyingPaymentStubIndexRateSpreadPositionType.field
			return 40720
		end
		def initialize(data = nil)
			if( data == nil )
				super(40720)
			else
				super(40720, data)
			end
		end
	end

	class UnderlyingPaymentStubIndexRateTreatment < Quickfix::IntField
		def UnderlyingPaymentStubIndexRateTreatment.field
			return 40721
		end
		def initialize(data = nil)
			if( data == nil )
				super(40721)
			else
				super(40721, data)
			end
		end
	end

	class UnderlyingPaymentStubIndexCapRate < Quickfix::DoubleField
		def UnderlyingPaymentStubIndexCapRate.field
			return 40722
		end
		def initialize(data = nil)
			if( data == nil )
				super(40722)
			else
				super(40722, data)
			end
		end
	end

	class UnderlyingPaymentStubIndexCapRateBuySide < Quickfix::IntField
		def UnderlyingPaymentStubIndexCapRateBuySide.field
			return 40723
		end
		def initialize(data = nil)
			if( data == nil )
				super(40723)
			else
				super(40723, data)
			end
		end
	end

	class UnderlyingPaymentStubIndexCapRateSellSide < Quickfix::IntField
		def UnderlyingPaymentStubIndexCapRateSellSide.field
			return 40724
		end
		def initialize(data = nil)
			if( data == nil )
				super(40724)
			else
				super(40724, data)
			end
		end
	end

	class UnderlyingPaymentStubIndexFloorRate < Quickfix::DoubleField
		def UnderlyingPaymentStubIndexFloorRate.field
			return 40725
		end
		def initialize(data = nil)
			if( data == nil )
				super(40725)
			else
				super(40725, data)
			end
		end
	end

	class UnderlyingPaymentStubIndexFloorRateBuySide < Quickfix::IntField
		def UnderlyingPaymentStubIndexFloorRateBuySide.field
			return 40726
		end
		def initialize(data = nil)
			if( data == nil )
				super(40726)
			else
				super(40726, data)
			end
		end
	end

	class UnderlyingPaymentStubIndexFloorRateSellSide < Quickfix::IntField
		def UnderlyingPaymentStubIndexFloorRateSellSide.field
			return 40727
		end
		def initialize(data = nil)
			if( data == nil )
				super(40727)
			else
				super(40727, data)
			end
		end
	end

	class UnderlyingPaymentStubIndex2 < Quickfix::StringField
		def UnderlyingPaymentStubIndex2.field
			return 40728
		end
		def initialize(data = nil)
			if( data == nil )
				super(40728)
			else
				super(40728, data)
			end
		end
	end

	class UnderlyingPaymentStubIndex2Source < Quickfix::IntField
		def UnderlyingPaymentStubIndex2Source.field
			return 40729
		end
		def initialize(data = nil)
			if( data == nil )
				super(40729)
			else
				super(40729, data)
			end
		end
	end

	class UnderlyingPaymentStubIndex2CurvePeriod < Quickfix::IntField
		def UnderlyingPaymentStubIndex2CurvePeriod.field
			return 40730
		end
		def initialize(data = nil)
			if( data == nil )
				super(40730)
			else
				super(40730, data)
			end
		end
	end

	class UnderlyingPaymentStubIndex2CurveUnit < Quickfix::StringField
		def UnderlyingPaymentStubIndex2CurveUnit.field
			return 40731
		end
		def initialize(data = nil)
			if( data == nil )
				super(40731)
			else
				super(40731, data)
			end
		end
	end

	class UnderlyingPaymentStubIndex2RateMultiplier < Quickfix::DoubleField
		def UnderlyingPaymentStubIndex2RateMultiplier.field
			return 40732
		end
		def initialize(data = nil)
			if( data == nil )
				super(40732)
			else
				super(40732, data)
			end
		end
	end

	class UnderlyingPaymentStubIndex2RateSpread < Quickfix::DoubleField
		def UnderlyingPaymentStubIndex2RateSpread.field
			return 40733
		end
		def initialize(data = nil)
			if( data == nil )
				super(40733)
			else
				super(40733, data)
			end
		end
	end

	class UnderlyingPaymentStubIndex2RateSpreadPositionType < Quickfix::IntField
		def UnderlyingPaymentStubIndex2RateSpreadPositionType.field
			return 40734
		end
		def initialize(data = nil)
			if( data == nil )
				super(40734)
			else
				super(40734, data)
			end
		end
	end

	class UnderlyingPaymentStubIndex2RateTreatment < Quickfix::IntField
		def UnderlyingPaymentStubIndex2RateTreatment.field
			return 40735
		end
		def initialize(data = nil)
			if( data == nil )
				super(40735)
			else
				super(40735, data)
			end
		end
	end

	class UnderlyingPaymentStubIndex2CapRate < Quickfix::DoubleField
		def UnderlyingPaymentStubIndex2CapRate.field
			return 40736
		end
		def initialize(data = nil)
			if( data == nil )
				super(40736)
			else
				super(40736, data)
			end
		end
	end

	class UnderlyingPaymentStubIndex2FloorRate < Quickfix::DoubleField
		def UnderlyingPaymentStubIndex2FloorRate.field
			return 40737
		end
		def initialize(data = nil)
			if( data == nil )
				super(40737)
			else
				super(40737, data)
			end
		end
	end

	class PaymentStreamType < Quickfix::IntField
		def PaymentStreamType.field
			return 40738
		end
		def initialize(data = nil)
			if( data == nil )
				super(40738)
			else
				super(40738, data)
			end
		end
	end

	class PaymentStreamMarketRate < Quickfix::IntField
		def PaymentStreamMarketRate.field
			return 40739
		end
		def initialize(data = nil)
			if( data == nil )
				super(40739)
			else
				super(40739, data)
			end
		end
	end

	class PaymentStreamDelayIndicator < Quickfix::BoolField
		def PaymentStreamDelayIndicator.field
			return 40740
		end
		def initialize(data = nil)
			if( data == nil )
				super(40740)
			else
				super(40740, data)
			end
		end
	end

	class PaymentStreamSettlCurrency < Quickfix::StringField
		def PaymentStreamSettlCurrency.field
			return 40741
		end
		def initialize(data = nil)
			if( data == nil )
				super(40741)
			else
				super(40741, data)
			end
		end
	end

	class PaymentStreamDayCount < Quickfix::IntField
		def PaymentStreamDayCount.field
			return 40742
		end
		def initialize(data = nil)
			if( data == nil )
				super(40742)
			else
				super(40742, data)
			end
		end
	end

	class PaymentStreamAccrualDays < Quickfix::IntField
		def PaymentStreamAccrualDays.field
			return 40743
		end
		def initialize(data = nil)
			if( data == nil )
				super(40743)
			else
				super(40743, data)
			end
		end
	end

	class PaymentStreamDiscountType < Quickfix::IntField
		def PaymentStreamDiscountType.field
			return 40744
		end
		def initialize(data = nil)
			if( data == nil )
				super(40744)
			else
				super(40744, data)
			end
		end
	end

	class PaymentStreamDiscountRate < Quickfix::DoubleField
		def PaymentStreamDiscountRate.field
			return 40745
		end
		def initialize(data = nil)
			if( data == nil )
				super(40745)
			else
				super(40745, data)
			end
		end
	end

	class PaymentStreamDiscountRateDayCount < Quickfix::IntField
		def PaymentStreamDiscountRateDayCount.field
			return 40746
		end
		def initialize(data = nil)
			if( data == nil )
				super(40746)
			else
				super(40746, data)
			end
		end
	end

	class PaymentStreamCompoundingMethod < Quickfix::IntField
		def PaymentStreamCompoundingMethod.field
			return 40747
		end
		def initialize(data = nil)
			if( data == nil )
				super(40747)
			else
				super(40747, data)
			end
		end
	end

	class PaymentStreamInitialPrincipalExchangeIndicator < Quickfix::BoolField
		def PaymentStreamInitialPrincipalExchangeIndicator.field
			return 40748
		end
		def initialize(data = nil)
			if( data == nil )
				super(40748)
			else
				super(40748, data)
			end
		end
	end

	class PaymentStreamInterimPrincipalExchangeIndicator < Quickfix::BoolField
		def PaymentStreamInterimPrincipalExchangeIndicator.field
			return 40749
		end
		def initialize(data = nil)
			if( data == nil )
				super(40749)
			else
				super(40749, data)
			end
		end
	end

	class PaymentStreamFinalPrincipalExchangeIndicator < Quickfix::BoolField
		def PaymentStreamFinalPrincipalExchangeIndicator.field
			return 40750
		end
		def initialize(data = nil)
			if( data == nil )
				super(40750)
			else
				super(40750, data)
			end
		end
	end

	class PaymentStreamPaymentDateBusinessDayConvention < Quickfix::IntField
		def PaymentStreamPaymentDateBusinessDayConvention.field
			return 40751
		end
		def initialize(data = nil)
			if( data == nil )
				super(40751)
			else
				super(40751, data)
			end
		end
	end

	class PaymentStreamPaymentDateBusinessCenter < Quickfix::StringField
		def PaymentStreamPaymentDateBusinessCenter.field
			return 40752
		end
		def initialize(data = nil)
			if( data == nil )
				super(40752)
			else
				super(40752, data)
			end
		end
	end

	class PaymentStreamPaymentFrequencyPeriod < Quickfix::IntField
		def PaymentStreamPaymentFrequencyPeriod.field
			return 40753
		end
		def initialize(data = nil)
			if( data == nil )
				super(40753)
			else
				super(40753, data)
			end
		end
	end

	class PaymentStreamPaymentFrequencyUnit < Quickfix::StringField
		def PaymentStreamPaymentFrequencyUnit.field
			return 40754
		end
		def initialize(data = nil)
			if( data == nil )
				super(40754)
			else
				super(40754, data)
			end
		end
	end

	class PaymentStreamPaymentRollConvention < Quickfix::StringField
		def PaymentStreamPaymentRollConvention.field
			return 40755
		end
		def initialize(data = nil)
			if( data == nil )
				super(40755)
			else
				super(40755, data)
			end
		end
	end

	class PaymentStreamFirstPaymentDateUnadjusted < Quickfix::StringField
		def PaymentStreamFirstPaymentDateUnadjusted.field
			return 40756
		end
		def initialize(data = nil)
			if( data == nil )
				super(40756)
			else
				super(40756, data)
			end
		end
	end

	class PaymentStreamLastRegularPaymentDateUnadjusted < Quickfix::StringField
		def PaymentStreamLastRegularPaymentDateUnadjusted.field
			return 40757
		end
		def initialize(data = nil)
			if( data == nil )
				super(40757)
			else
				super(40757, data)
			end
		end
	end

	class PaymentStreamPaymentDateRelativeTo < Quickfix::IntField
		def PaymentStreamPaymentDateRelativeTo.field
			return 40758
		end
		def initialize(data = nil)
			if( data == nil )
				super(40758)
			else
				super(40758, data)
			end
		end
	end

	class PaymentStreamPaymentDateOffsetPeriod < Quickfix::IntField
		def PaymentStreamPaymentDateOffsetPeriod.field
			return 40759
		end
		def initialize(data = nil)
			if( data == nil )
				super(40759)
			else
				super(40759, data)
			end
		end
	end

	class PaymentStreamPaymentDateOffsetUnit < Quickfix::StringField
		def PaymentStreamPaymentDateOffsetUnit.field
			return 40760
		end
		def initialize(data = nil)
			if( data == nil )
				super(40760)
			else
				super(40760, data)
			end
		end
	end

	class PaymentStreamResetDateRelativeTo < Quickfix::IntField
		def PaymentStreamResetDateRelativeTo.field
			return 40761
		end
		def initialize(data = nil)
			if( data == nil )
				super(40761)
			else
				super(40761, data)
			end
		end
	end

	class PaymentStreamResetDateBusinessDayConvention < Quickfix::IntField
		def PaymentStreamResetDateBusinessDayConvention.field
			return 40762
		end
		def initialize(data = nil)
			if( data == nil )
				super(40762)
			else
				super(40762, data)
			end
		end
	end

	class PaymentStreamResetDateBusinessCenter < Quickfix::StringField
		def PaymentStreamResetDateBusinessCenter.field
			return 40763
		end
		def initialize(data = nil)
			if( data == nil )
				super(40763)
			else
				super(40763, data)
			end
		end
	end

	class PaymentStreamResetFrequencyPeriod < Quickfix::IntField
		def PaymentStreamResetFrequencyPeriod.field
			return 40764
		end
		def initialize(data = nil)
			if( data == nil )
				super(40764)
			else
				super(40764, data)
			end
		end
	end

	class PaymentStreamResetFrequencyUnit < Quickfix::StringField
		def PaymentStreamResetFrequencyUnit.field
			return 40765
		end
		def initialize(data = nil)
			if( data == nil )
				super(40765)
			else
				super(40765, data)
			end
		end
	end

	class PaymentStreamResetWeeklyRollConvention < Quickfix::StringField
		def PaymentStreamResetWeeklyRollConvention.field
			return 40766
		end
		def initialize(data = nil)
			if( data == nil )
				super(40766)
			else
				super(40766, data)
			end
		end
	end

	class PaymentStreamInitialFixingDateRelativeTo < Quickfix::IntField
		def PaymentStreamInitialFixingDateRelativeTo.field
			return 40767
		end
		def initialize(data = nil)
			if( data == nil )
				super(40767)
			else
				super(40767, data)
			end
		end
	end

	class PaymentStreamInitialFixingDateBusinessDayConvention < Quickfix::IntField
		def PaymentStreamInitialFixingDateBusinessDayConvention.field
			return 40768
		end
		def initialize(data = nil)
			if( data == nil )
				super(40768)
			else
				super(40768, data)
			end
		end
	end

	class PaymentStreamInitialFixingDateBusinessCenter < Quickfix::StringField
		def PaymentStreamInitialFixingDateBusinessCenter.field
			return 40769
		end
		def initialize(data = nil)
			if( data == nil )
				super(40769)
			else
				super(40769, data)
			end
		end
	end

	class PaymentStreamInitialFixingDateOffsetPeriod < Quickfix::IntField
		def PaymentStreamInitialFixingDateOffsetPeriod.field
			return 40770
		end
		def initialize(data = nil)
			if( data == nil )
				super(40770)
			else
				super(40770, data)
			end
		end
	end

	class PaymentStreamInitialFixingDateOffsetUnit < Quickfix::StringField
		def PaymentStreamInitialFixingDateOffsetUnit.field
			return 40771
		end
		def initialize(data = nil)
			if( data == nil )
				super(40771)
			else
				super(40771, data)
			end
		end
	end

	class PaymentStreamInitialFixingDateOffsetDayType < Quickfix::IntField
		def PaymentStreamInitialFixingDateOffsetDayType.field
			return 40772
		end
		def initialize(data = nil)
			if( data == nil )
				super(40772)
			else
				super(40772, data)
			end
		end
	end

	class PaymentStreamInitialFixingDateAdjusted < Quickfix::StringField
		def PaymentStreamInitialFixingDateAdjusted.field
			return 40773
		end
		def initialize(data = nil)
			if( data == nil )
				super(40773)
			else
				super(40773, data)
			end
		end
	end

	class PaymentStreamFixingDateRelativeTo < Quickfix::IntField
		def PaymentStreamFixingDateRelativeTo.field
			return 40774
		end
		def initialize(data = nil)
			if( data == nil )
				super(40774)
			else
				super(40774, data)
			end
		end
	end

	class PaymentStreamFixingDateBusinessDayConvention < Quickfix::IntField
		def PaymentStreamFixingDateBusinessDayConvention.field
			return 40775
		end
		def initialize(data = nil)
			if( data == nil )
				super(40775)
			else
				super(40775, data)
			end
		end
	end

	class PaymentStreamFixingDateBusinessCenter < Quickfix::StringField
		def PaymentStreamFixingDateBusinessCenter.field
			return 40776
		end
		def initialize(data = nil)
			if( data == nil )
				super(40776)
			else
				super(40776, data)
			end
		end
	end

	class PaymentStreamFixingDateOffsetPeriod < Quickfix::IntField
		def PaymentStreamFixingDateOffsetPeriod.field
			return 40777
		end
		def initialize(data = nil)
			if( data == nil )
				super(40777)
			else
				super(40777, data)
			end
		end
	end

	class PaymentStreamFixingDateOffsetUnit < Quickfix::StringField
		def PaymentStreamFixingDateOffsetUnit.field
			return 40778
		end
		def initialize(data = nil)
			if( data == nil )
				super(40778)
			else
				super(40778, data)
			end
		end
	end

	class PaymentStreamFixingDateOffsetDayType < Quickfix::IntField
		def PaymentStreamFixingDateOffsetDayType.field
			return 40779
		end
		def initialize(data = nil)
			if( data == nil )
				super(40779)
			else
				super(40779, data)
			end
		end
	end

	class PaymentStreamFixingDateAdjusted < Quickfix::StringField
		def PaymentStreamFixingDateAdjusted.field
			return 40780
		end
		def initialize(data = nil)
			if( data == nil )
				super(40780)
			else
				super(40780, data)
			end
		end
	end

	class PaymentStreamRateCutoffDateOffsetPeriod < Quickfix::IntField
		def PaymentStreamRateCutoffDateOffsetPeriod.field
			return 40781
		end
		def initialize(data = nil)
			if( data == nil )
				super(40781)
			else
				super(40781, data)
			end
		end
	end

	class PaymentStreamRateCutoffDateOffsetUnit < Quickfix::StringField
		def PaymentStreamRateCutoffDateOffsetUnit.field
			return 40782
		end
		def initialize(data = nil)
			if( data == nil )
				super(40782)
			else
				super(40782, data)
			end
		end
	end

	class PaymentStreamRateCutoffDateOffsetDayType < Quickfix::IntField
		def PaymentStreamRateCutoffDateOffsetDayType.field
			return 40783
		end
		def initialize(data = nil)
			if( data == nil )
				super(40783)
			else
				super(40783, data)
			end
		end
	end

	class PaymentStreamRate < Quickfix::DoubleField
		def PaymentStreamRate.field
			return 40784
		end
		def initialize(data = nil)
			if( data == nil )
				super(40784)
			else
				super(40784, data)
			end
		end
	end

	class PaymentStreamFixedAmount < Quickfix::DoubleField
		def PaymentStreamFixedAmount.field
			return 40785
		end
		def initialize(data = nil)
			if( data == nil )
				super(40785)
			else
				super(40785, data)
			end
		end
	end

	class PaymentStreamRateOrAmountCurrency < Quickfix::StringField
		def PaymentStreamRateOrAmountCurrency.field
			return 40786
		end
		def initialize(data = nil)
			if( data == nil )
				super(40786)
			else
				super(40786, data)
			end
		end
	end

	class PaymentStreamFutureValueNotional < Quickfix::DoubleField
		def PaymentStreamFutureValueNotional.field
			return 40787
		end
		def initialize(data = nil)
			if( data == nil )
				super(40787)
			else
				super(40787, data)
			end
		end
	end

	class PaymentStreamFutureValueDateAdjusted < Quickfix::StringField
		def PaymentStreamFutureValueDateAdjusted.field
			return 40788
		end
		def initialize(data = nil)
			if( data == nil )
				super(40788)
			else
				super(40788, data)
			end
		end
	end

	class PaymentStreamRateIndex < Quickfix::StringField
		def PaymentStreamRateIndex.field
			return 40789
		end
		def initialize(data = nil)
			if( data == nil )
				super(40789)
			else
				super(40789, data)
			end
		end
	end

	class PaymentStreamRateIndexSource < Quickfix::IntField
		def PaymentStreamRateIndexSource.field
			return 40790
		end
		def initialize(data = nil)
			if( data == nil )
				super(40790)
			else
				super(40790, data)
			end
		end
	end

	class PaymentStreamRateIndexCurveUnit < Quickfix::StringField
		def PaymentStreamRateIndexCurveUnit.field
			return 40791
		end
		def initialize(data = nil)
			if( data == nil )
				super(40791)
			else
				super(40791, data)
			end
		end
	end

	class PaymentStreamRateIndexCurvePeriod < Quickfix::IntField
		def PaymentStreamRateIndexCurvePeriod.field
			return 40792
		end
		def initialize(data = nil)
			if( data == nil )
				super(40792)
			else
				super(40792, data)
			end
		end
	end

	class PaymentStreamRateMultiplier < Quickfix::DoubleField
		def PaymentStreamRateMultiplier.field
			return 40793
		end
		def initialize(data = nil)
			if( data == nil )
				super(40793)
			else
				super(40793, data)
			end
		end
	end

	class PaymentStreamRateSpread < Quickfix::DoubleField
		def PaymentStreamRateSpread.field
			return 40794
		end
		def initialize(data = nil)
			if( data == nil )
				super(40794)
			else
				super(40794, data)
			end
		end
	end

	class PaymentStreamRateSpreadPositionType < Quickfix::IntField
		def PaymentStreamRateSpreadPositionType.field
			return 40795
		end
		def initialize(data = nil)
			if( data == nil )
				super(40795)
			else
				super(40795, data)
			end
		end
	end

	class PaymentStreamRateTreatment < Quickfix::IntField
		def PaymentStreamRateTreatment.field
			return 40796
		end
		def initialize(data = nil)
			if( data == nil )
				super(40796)
			else
				super(40796, data)
			end
		end
	end

	class PaymentStreamCapRate < Quickfix::DoubleField
		def PaymentStreamCapRate.field
			return 40797
		end
		def initialize(data = nil)
			if( data == nil )
				super(40797)
			else
				super(40797, data)
			end
		end
	end

	class PaymentStreamCapRateBuySide < Quickfix::IntField
		def PaymentStreamCapRateBuySide.field
			return 40798
		end
		def initialize(data = nil)
			if( data == nil )
				super(40798)
			else
				super(40798, data)
			end
		end
	end

	class PaymentStreamCapRateSellSide < Quickfix::IntField
		def PaymentStreamCapRateSellSide.field
			return 40799
		end
		def initialize(data = nil)
			if( data == nil )
				super(40799)
			else
				super(40799, data)
			end
		end
	end

	class PaymentStreamFloorRate < Quickfix::DoubleField
		def PaymentStreamFloorRate.field
			return 40800
		end
		def initialize(data = nil)
			if( data == nil )
				super(40800)
			else
				super(40800, data)
			end
		end
	end

	class PaymentStreamFloorRateBuySide < Quickfix::IntField
		def PaymentStreamFloorRateBuySide.field
			return 40801
		end
		def initialize(data = nil)
			if( data == nil )
				super(40801)
			else
				super(40801, data)
			end
		end
	end

	class PaymentStreamFloorRateSellSide < Quickfix::IntField
		def PaymentStreamFloorRateSellSide.field
			return 40802
		end
		def initialize(data = nil)
			if( data == nil )
				super(40802)
			else
				super(40802, data)
			end
		end
	end

	class PaymentStreamInitialRate < Quickfix::DoubleField
		def PaymentStreamInitialRate.field
			return 40803
		end
		def initialize(data = nil)
			if( data == nil )
				super(40803)
			else
				super(40803, data)
			end
		end
	end

	class PaymentStreamFinalRateRoundingDirection < Quickfix::CharField
		def PaymentStreamFinalRateRoundingDirection.field
			return 40804
		end
		def initialize(data = nil)
			if( data == nil )
				super(40804)
			else
				super(40804, data)
			end
		end
	end

	class PaymentStreamFinalRatePrecision < Quickfix::IntField
		def PaymentStreamFinalRatePrecision.field
			return 40805
		end
		def initialize(data = nil)
			if( data == nil )
				super(40805)
			else
				super(40805, data)
			end
		end
	end

	class PaymentStreamAveragingMethod < Quickfix::IntField
		def PaymentStreamAveragingMethod.field
			return 40806
		end
		def initialize(data = nil)
			if( data == nil )
				super(40806)
			else
				super(40806, data)
			end
		end
	end

	class PaymentStreamNegativeRateTreatment < Quickfix::IntField
		def PaymentStreamNegativeRateTreatment.field
			return 40807
		end
		def initialize(data = nil)
			if( data == nil )
				super(40807)
			else
				super(40807, data)
			end
		end
	end

	class PaymentStreamInflationLagPeriod < Quickfix::IntField
		def PaymentStreamInflationLagPeriod.field
			return 40808
		end
		def initialize(data = nil)
			if( data == nil )
				super(40808)
			else
				super(40808, data)
			end
		end
	end

	class PaymentStreamInflationLagUnit < Quickfix::StringField
		def PaymentStreamInflationLagUnit.field
			return 40809
		end
		def initialize(data = nil)
			if( data == nil )
				super(40809)
			else
				super(40809, data)
			end
		end
	end

	class PaymentStreamInflationLagDayType < Quickfix::IntField
		def PaymentStreamInflationLagDayType.field
			return 40810
		end
		def initialize(data = nil)
			if( data == nil )
				super(40810)
			else
				super(40810, data)
			end
		end
	end

	class PaymentStreamInflationInterpolationMethod < Quickfix::IntField
		def PaymentStreamInflationInterpolationMethod.field
			return 40811
		end
		def initialize(data = nil)
			if( data == nil )
				super(40811)
			else
				super(40811, data)
			end
		end
	end

	class PaymentStreamInflationIndexSource < Quickfix::IntField
		def PaymentStreamInflationIndexSource.field
			return 40812
		end
		def initialize(data = nil)
			if( data == nil )
				super(40812)
			else
				super(40812, data)
			end
		end
	end

	class PaymentStreamInflationPublicationSource < Quickfix::StringField
		def PaymentStreamInflationPublicationSource.field
			return 40813
		end
		def initialize(data = nil)
			if( data == nil )
				super(40813)
			else
				super(40813, data)
			end
		end
	end

	class PaymentStreamInflationInitialIndexLevel < Quickfix::DoubleField
		def PaymentStreamInflationInitialIndexLevel.field
			return 40814
		end
		def initialize(data = nil)
			if( data == nil )
				super(40814)
			else
				super(40814, data)
			end
		end
	end

	class PaymentStreamInflationFallbackBondApplicable < Quickfix::BoolField
		def PaymentStreamInflationFallbackBondApplicable.field
			return 40815
		end
		def initialize(data = nil)
			if( data == nil )
				super(40815)
			else
				super(40815, data)
			end
		end
	end

	class PaymentStreamFRADiscounting < Quickfix::IntField
		def PaymentStreamFRADiscounting.field
			return 40816
		end
		def initialize(data = nil)
			if( data == nil )
				super(40816)
			else
				super(40816, data)
			end
		end
	end

	class PaymentStreamNonDeliverableRefCurrency < Quickfix::StringField
		def PaymentStreamNonDeliverableRefCurrency.field
			return 40817
		end
		def initialize(data = nil)
			if( data == nil )
				super(40817)
			else
				super(40817, data)
			end
		end
	end

	class PaymentStreamNonDeliverableFixingDatesBusinessDayConvention < Quickfix::IntField
		def PaymentStreamNonDeliverableFixingDatesBusinessDayConvention.field
			return 40818
		end
		def initialize(data = nil)
			if( data == nil )
				super(40818)
			else
				super(40818, data)
			end
		end
	end

	class PaymentStreamNonDeliverableFixingDatesBusinessCenter < Quickfix::StringField
		def PaymentStreamNonDeliverableFixingDatesBusinessCenter.field
			return 40819
		end
		def initialize(data = nil)
			if( data == nil )
				super(40819)
			else
				super(40819, data)
			end
		end
	end

	class PaymentStreamNonDeliverableFixingDatesRelativeTo < Quickfix::IntField
		def PaymentStreamNonDeliverableFixingDatesRelativeTo.field
			return 40820
		end
		def initialize(data = nil)
			if( data == nil )
				super(40820)
			else
				super(40820, data)
			end
		end
	end

	class PaymentStreamNonDeliverableFixingDatesOffsetPeriod < Quickfix::IntField
		def PaymentStreamNonDeliverableFixingDatesOffsetPeriod.field
			return 40821
		end
		def initialize(data = nil)
			if( data == nil )
				super(40821)
			else
				super(40821, data)
			end
		end
	end

	class PaymentStreamNonDeliverableFixingDatesOffsetUnit < Quickfix::StringField
		def PaymentStreamNonDeliverableFixingDatesOffsetUnit.field
			return 40822
		end
		def initialize(data = nil)
			if( data == nil )
				super(40822)
			else
				super(40822, data)
			end
		end
	end

	class PaymentStreamNonDeliverableFixingDatesOffsetDayType < Quickfix::IntField
		def PaymentStreamNonDeliverableFixingDatesOffsetDayType.field
			return 40823
		end
		def initialize(data = nil)
			if( data == nil )
				super(40823)
			else
				super(40823, data)
			end
		end
	end

	class UnderlyingPaymentStreamNonDeliverableSettlReferencePage < Quickfix::StringField
		def UnderlyingPaymentStreamNonDeliverableSettlReferencePage.field
			return 40824
		end
		def initialize(data = nil)
			if( data == nil )
				super(40824)
			else
				super(40824, data)
			end
		end
	end

	class NoNonDeliverableFixingDates < Quickfix::IntField
		def NoNonDeliverableFixingDates.field
			return 40825
		end
		def initialize(data = nil)
			if( data == nil )
				super(40825)
			else
				super(40825, data)
			end
		end
	end

	class NonDeliverableFixingDate < Quickfix::StringField
		def NonDeliverableFixingDate.field
			return 40826
		end
		def initialize(data = nil)
			if( data == nil )
				super(40826)
			else
				super(40826, data)
			end
		end
	end

	class NonDeliverableFixingDateType < Quickfix::IntField
		def NonDeliverableFixingDateType.field
			return 40827
		end
		def initialize(data = nil)
			if( data == nil )
				super(40827)
			else
				super(40827, data)
			end
		end
	end

	class NoPaymentSchedules < Quickfix::IntField
		def NoPaymentSchedules.field
			return 40828
		end
		def initialize(data = nil)
			if( data == nil )
				super(40828)
			else
				super(40828, data)
			end
		end
	end

	class PaymentScheduleType < Quickfix::IntField
		def PaymentScheduleType.field
			return 40829
		end
		def initialize(data = nil)
			if( data == nil )
				super(40829)
			else
				super(40829, data)
			end
		end
	end

	class PaymentScheduleStubType < Quickfix::IntField
		def PaymentScheduleStubType.field
			return 40830
		end
		def initialize(data = nil)
			if( data == nil )
				super(40830)
			else
				super(40830, data)
			end
		end
	end

	class PaymentScheduleStartDateUnadjusted < Quickfix::StringField
		def PaymentScheduleStartDateUnadjusted.field
			return 40831
		end
		def initialize(data = nil)
			if( data == nil )
				super(40831)
			else
				super(40831, data)
			end
		end
	end

	class PaymentScheduleEndDateUnadjusted < Quickfix::StringField
		def PaymentScheduleEndDateUnadjusted.field
			return 40832
		end
		def initialize(data = nil)
			if( data == nil )
				super(40832)
			else
				super(40832, data)
			end
		end
	end

	class PaymentSchedulePaySide < Quickfix::IntField
		def PaymentSchedulePaySide.field
			return 40833
		end
		def initialize(data = nil)
			if( data == nil )
				super(40833)
			else
				super(40833, data)
			end
		end
	end

	class PaymentScheduleReceiveSide < Quickfix::IntField
		def PaymentScheduleReceiveSide.field
			return 40834
		end
		def initialize(data = nil)
			if( data == nil )
				super(40834)
			else
				super(40834, data)
			end
		end
	end

	class PaymentScheduleNotional < Quickfix::DoubleField
		def PaymentScheduleNotional.field
			return 40835
		end
		def initialize(data = nil)
			if( data == nil )
				super(40835)
			else
				super(40835, data)
			end
		end
	end

	class PaymentScheduleCurrency < Quickfix::StringField
		def PaymentScheduleCurrency.field
			return 40836
		end
		def initialize(data = nil)
			if( data == nil )
				super(40836)
			else
				super(40836, data)
			end
		end
	end

	class PaymentScheduleRate < Quickfix::DoubleField
		def PaymentScheduleRate.field
			return 40837
		end
		def initialize(data = nil)
			if( data == nil )
				super(40837)
			else
				super(40837, data)
			end
		end
	end

	class PaymentScheduleRateMultiplier < Quickfix::DoubleField
		def PaymentScheduleRateMultiplier.field
			return 40838
		end
		def initialize(data = nil)
			if( data == nil )
				super(40838)
			else
				super(40838, data)
			end
		end
	end

	class PaymentScheduleRateSpread < Quickfix::DoubleField
		def PaymentScheduleRateSpread.field
			return 40839
		end
		def initialize(data = nil)
			if( data == nil )
				super(40839)
			else
				super(40839, data)
			end
		end
	end

	class PaymentScheduleRateSpreadPositionType < Quickfix::IntField
		def PaymentScheduleRateSpreadPositionType.field
			return 40840
		end
		def initialize(data = nil)
			if( data == nil )
				super(40840)
			else
				super(40840, data)
			end
		end
	end

	class PaymentScheduleRateTreatment < Quickfix::IntField
		def PaymentScheduleRateTreatment.field
			return 40841
		end
		def initialize(data = nil)
			if( data == nil )
				super(40841)
			else
				super(40841, data)
			end
		end
	end

	class PaymentScheduleFixedAmount < Quickfix::DoubleField
		def PaymentScheduleFixedAmount.field
			return 40842
		end
		def initialize(data = nil)
			if( data == nil )
				super(40842)
			else
				super(40842, data)
			end
		end
	end

	class PaymentScheduleFixedCurrency < Quickfix::StringField
		def PaymentScheduleFixedCurrency.field
			return 40843
		end
		def initialize(data = nil)
			if( data == nil )
				super(40843)
			else
				super(40843, data)
			end
		end
	end

	class PaymentScheduleStepFrequencyPeriod < Quickfix::IntField
		def PaymentScheduleStepFrequencyPeriod.field
			return 40844
		end
		def initialize(data = nil)
			if( data == nil )
				super(40844)
			else
				super(40844, data)
			end
		end
	end

	class PaymentScheduleStepFrequencyUnit < Quickfix::StringField
		def PaymentScheduleStepFrequencyUnit.field
			return 40845
		end
		def initialize(data = nil)
			if( data == nil )
				super(40845)
			else
				super(40845, data)
			end
		end
	end

	class PaymentScheduleStepOffsetValue < Quickfix::DoubleField
		def PaymentScheduleStepOffsetValue.field
			return 40846
		end
		def initialize(data = nil)
			if( data == nil )
				super(40846)
			else
				super(40846, data)
			end
		end
	end

	class PaymentScheduleStepRate < Quickfix::DoubleField
		def PaymentScheduleStepRate.field
			return 40847
		end
		def initialize(data = nil)
			if( data == nil )
				super(40847)
			else
				super(40847, data)
			end
		end
	end

	class PaymentScheduleStepOffsetRate < Quickfix::DoubleField
		def PaymentScheduleStepOffsetRate.field
			return 40848
		end
		def initialize(data = nil)
			if( data == nil )
				super(40848)
			else
				super(40848, data)
			end
		end
	end

	class PaymentScheduleStepRelativeTo < Quickfix::IntField
		def PaymentScheduleStepRelativeTo.field
			return 40849
		end
		def initialize(data = nil)
			if( data == nil )
				super(40849)
			else
				super(40849, data)
			end
		end
	end

	class PaymentScheduleFixingDateUnadjusted < Quickfix::StringField
		def PaymentScheduleFixingDateUnadjusted.field
			return 40850
		end
		def initialize(data = nil)
			if( data == nil )
				super(40850)
			else
				super(40850, data)
			end
		end
	end

	class PaymentScheduleWeight < Quickfix::DoubleField
		def PaymentScheduleWeight.field
			return 40851
		end
		def initialize(data = nil)
			if( data == nil )
				super(40851)
			else
				super(40851, data)
			end
		end
	end

	class PaymentScheduleFixingDateRelativeTo < Quickfix::IntField
		def PaymentScheduleFixingDateRelativeTo.field
			return 40852
		end
		def initialize(data = nil)
			if( data == nil )
				super(40852)
			else
				super(40852, data)
			end
		end
	end

	class PaymentScheduleFixingDateBusinessDayConvention < Quickfix::IntField
		def PaymentScheduleFixingDateBusinessDayConvention.field
			return 40853
		end
		def initialize(data = nil)
			if( data == nil )
				super(40853)
			else
				super(40853, data)
			end
		end
	end

	class PaymentScheduleFixingDateBusinessCenter < Quickfix::StringField
		def PaymentScheduleFixingDateBusinessCenter.field
			return 40854
		end
		def initialize(data = nil)
			if( data == nil )
				super(40854)
			else
				super(40854, data)
			end
		end
	end

	class PaymentScheduleFixingDateOffsetPeriod < Quickfix::IntField
		def PaymentScheduleFixingDateOffsetPeriod.field
			return 40855
		end
		def initialize(data = nil)
			if( data == nil )
				super(40855)
			else
				super(40855, data)
			end
		end
	end

	class PaymentScheduleFixingDateOffsetUnit < Quickfix::StringField
		def PaymentScheduleFixingDateOffsetUnit.field
			return 40856
		end
		def initialize(data = nil)
			if( data == nil )
				super(40856)
			else
				super(40856, data)
			end
		end
	end

	class PaymentScheduleFixingDateOffsetDayType < Quickfix::IntField
		def PaymentScheduleFixingDateOffsetDayType.field
			return 40857
		end
		def initialize(data = nil)
			if( data == nil )
				super(40857)
			else
				super(40857, data)
			end
		end
	end

	class PaymentScheduleFixingDateAdjusted < Quickfix::StringField
		def PaymentScheduleFixingDateAdjusted.field
			return 40858
		end
		def initialize(data = nil)
			if( data == nil )
				super(40858)
			else
				super(40858, data)
			end
		end
	end

	class PaymentScheduleFixingTime < Quickfix::StringField
		def PaymentScheduleFixingTime.field
			return 40859
		end
		def initialize(data = nil)
			if( data == nil )
				super(40859)
			else
				super(40859, data)
			end
		end
	end

	class PaymentScheduleFixingTimeBusinessCenter < Quickfix::StringField
		def PaymentScheduleFixingTimeBusinessCenter.field
			return 40860
		end
		def initialize(data = nil)
			if( data == nil )
				super(40860)
			else
				super(40860, data)
			end
		end
	end

	class PaymentScheduleInterimExchangePaymentDateRelativeTo < Quickfix::IntField
		def PaymentScheduleInterimExchangePaymentDateRelativeTo.field
			return 40861
		end
		def initialize(data = nil)
			if( data == nil )
				super(40861)
			else
				super(40861, data)
			end
		end
	end

	class PaymentScheduleInterimExchangeDatesBusinessDayConvention < Quickfix::IntField
		def PaymentScheduleInterimExchangeDatesBusinessDayConvention.field
			return 40862
		end
		def initialize(data = nil)
			if( data == nil )
				super(40862)
			else
				super(40862, data)
			end
		end
	end

	class PaymentScheduleInterimExchangeDatesBusinessCenter < Quickfix::StringField
		def PaymentScheduleInterimExchangeDatesBusinessCenter.field
			return 40863
		end
		def initialize(data = nil)
			if( data == nil )
				super(40863)
			else
				super(40863, data)
			end
		end
	end

	class PaymentScheduleInterimExchangeDatesOffsetPeriod < Quickfix::IntField
		def PaymentScheduleInterimExchangeDatesOffsetPeriod.field
			return 40864
		end
		def initialize(data = nil)
			if( data == nil )
				super(40864)
			else
				super(40864, data)
			end
		end
	end

	class PaymentScheduleInterimExchangeDatesOffsetUnit < Quickfix::StringField
		def PaymentScheduleInterimExchangeDatesOffsetUnit.field
			return 40865
		end
		def initialize(data = nil)
			if( data == nil )
				super(40865)
			else
				super(40865, data)
			end
		end
	end

	class PaymentScheduleInterimExchangeDatesOffsetDayType < Quickfix::IntField
		def PaymentScheduleInterimExchangeDatesOffsetDayType.field
			return 40866
		end
		def initialize(data = nil)
			if( data == nil )
				super(40866)
			else
				super(40866, data)
			end
		end
	end

	class PaymentScheduleInterimExchangeDateAdjusted < Quickfix::StringField
		def PaymentScheduleInterimExchangeDateAdjusted.field
			return 40867
		end
		def initialize(data = nil)
			if( data == nil )
				super(40867)
			else
				super(40867, data)
			end
		end
	end

	class NoPaymentScheduleRateSources < Quickfix::IntField
		def NoPaymentScheduleRateSources.field
			return 40868
		end
		def initialize(data = nil)
			if( data == nil )
				super(40868)
			else
				super(40868, data)
			end
		end
	end

	class PaymentScheduleRateSource < Quickfix::IntField
		def PaymentScheduleRateSource.field
			return 40869
		end
		def initialize(data = nil)
			if( data == nil )
				super(40869)
			else
				super(40869, data)
			end
		end
	end

	class PaymentScheduleRateSourceType < Quickfix::IntField
		def PaymentScheduleRateSourceType.field
			return 40870
		end
		def initialize(data = nil)
			if( data == nil )
				super(40870)
			else
				super(40870, data)
			end
		end
	end

	class PaymentScheduleReferencePage < Quickfix::StringField
		def PaymentScheduleReferencePage.field
			return 40871
		end
		def initialize(data = nil)
			if( data == nil )
				super(40871)
			else
				super(40871, data)
			end
		end
	end

	class NoPaymentStubs < Quickfix::IntField
		def NoPaymentStubs.field
			return 40872
		end
		def initialize(data = nil)
			if( data == nil )
				super(40872)
			else
				super(40872, data)
			end
		end
	end

	class PaymentStubType < Quickfix::IntField
		def PaymentStubType.field
			return 40873
		end
		def initialize(data = nil)
			if( data == nil )
				super(40873)
			else
				super(40873, data)
			end
		end
	end

	class PaymentStubLength < Quickfix::IntField
		def PaymentStubLength.field
			return 40874
		end
		def initialize(data = nil)
			if( data == nil )
				super(40874)
			else
				super(40874, data)
			end
		end
	end

	class PaymentStubRate < Quickfix::DoubleField
		def PaymentStubRate.field
			return 40875
		end
		def initialize(data = nil)
			if( data == nil )
				super(40875)
			else
				super(40875, data)
			end
		end
	end

	class PaymentStubFixedAmount < Quickfix::DoubleField
		def PaymentStubFixedAmount.field
			return 40876
		end
		def initialize(data = nil)
			if( data == nil )
				super(40876)
			else
				super(40876, data)
			end
		end
	end

	class PaymentStubFixedCurrency < Quickfix::StringField
		def PaymentStubFixedCurrency.field
			return 40877
		end
		def initialize(data = nil)
			if( data == nil )
				super(40877)
			else
				super(40877, data)
			end
		end
	end

	class PaymentStubIndex < Quickfix::StringField
		def PaymentStubIndex.field
			return 40878
		end
		def initialize(data = nil)
			if( data == nil )
				super(40878)
			else
				super(40878, data)
			end
		end
	end

	class PaymentStubIndexSource < Quickfix::IntField
		def PaymentStubIndexSource.field
			return 40879
		end
		def initialize(data = nil)
			if( data == nil )
				super(40879)
			else
				super(40879, data)
			end
		end
	end

	class PaymentStubIndexCurvePeriod < Quickfix::IntField
		def PaymentStubIndexCurvePeriod.field
			return 40880
		end
		def initialize(data = nil)
			if( data == nil )
				super(40880)
			else
				super(40880, data)
			end
		end
	end

	class PaymentStubIndexCurveUnit < Quickfix::StringField
		def PaymentStubIndexCurveUnit.field
			return 40881
		end
		def initialize(data = nil)
			if( data == nil )
				super(40881)
			else
				super(40881, data)
			end
		end
	end

	class PaymentStubIndexRateMultiplier < Quickfix::DoubleField
		def PaymentStubIndexRateMultiplier.field
			return 40882
		end
		def initialize(data = nil)
			if( data == nil )
				super(40882)
			else
				super(40882, data)
			end
		end
	end

	class PaymentStubIndexRateSpread < Quickfix::DoubleField
		def PaymentStubIndexRateSpread.field
			return 40883
		end
		def initialize(data = nil)
			if( data == nil )
				super(40883)
			else
				super(40883, data)
			end
		end
	end

	class PaymentStubIndexRateSpreadPositionType < Quickfix::IntField
		def PaymentStubIndexRateSpreadPositionType.field
			return 40884
		end
		def initialize(data = nil)
			if( data == nil )
				super(40884)
			else
				super(40884, data)
			end
		end
	end

	class PaymentStubIndexRateTreatment < Quickfix::IntField
		def PaymentStubIndexRateTreatment.field
			return 40885
		end
		def initialize(data = nil)
			if( data == nil )
				super(40885)
			else
				super(40885, data)
			end
		end
	end

	class PaymentStubIndexCapRate < Quickfix::DoubleField
		def PaymentStubIndexCapRate.field
			return 40886
		end
		def initialize(data = nil)
			if( data == nil )
				super(40886)
			else
				super(40886, data)
			end
		end
	end

	class PaymentStubIndexCapRateBuySide < Quickfix::IntField
		def PaymentStubIndexCapRateBuySide.field
			return 40887
		end
		def initialize(data = nil)
			if( data == nil )
				super(40887)
			else
				super(40887, data)
			end
		end
	end

	class PaymentStubIndexCapRateSellSide < Quickfix::IntField
		def PaymentStubIndexCapRateSellSide.field
			return 40888
		end
		def initialize(data = nil)
			if( data == nil )
				super(40888)
			else
				super(40888, data)
			end
		end
	end

	class PaymentStubIndexFloorRate < Quickfix::DoubleField
		def PaymentStubIndexFloorRate.field
			return 40889
		end
		def initialize(data = nil)
			if( data == nil )
				super(40889)
			else
				super(40889, data)
			end
		end
	end

	class PaymentStubIndexFloorRateBuySide < Quickfix::IntField
		def PaymentStubIndexFloorRateBuySide.field
			return 40890
		end
		def initialize(data = nil)
			if( data == nil )
				super(40890)
			else
				super(40890, data)
			end
		end
	end

	class PaymentStubIndexFloorRateSellSide < Quickfix::IntField
		def PaymentStubIndexFloorRateSellSide.field
			return 40891
		end
		def initialize(data = nil)
			if( data == nil )
				super(40891)
			else
				super(40891, data)
			end
		end
	end

	class PaymentStubIndex2 < Quickfix::StringField
		def PaymentStubIndex2.field
			return 40892
		end
		def initialize(data = nil)
			if( data == nil )
				super(40892)
			else
				super(40892, data)
			end
		end
	end

	class PaymentStubIndex2Source < Quickfix::IntField
		def PaymentStubIndex2Source.field
			return 40893
		end
		def initialize(data = nil)
			if( data == nil )
				super(40893)
			else
				super(40893, data)
			end
		end
	end

	class PaymentStubIndex2CurvePeriod < Quickfix::IntField
		def PaymentStubIndex2CurvePeriod.field
			return 40894
		end
		def initialize(data = nil)
			if( data == nil )
				super(40894)
			else
				super(40894, data)
			end
		end
	end

	class PaymentStubIndex2CurveUnit < Quickfix::StringField
		def PaymentStubIndex2CurveUnit.field
			return 40895
		end
		def initialize(data = nil)
			if( data == nil )
				super(40895)
			else
				super(40895, data)
			end
		end
	end

	class PaymentStubIndex2RateMultiplier < Quickfix::DoubleField
		def PaymentStubIndex2RateMultiplier.field
			return 40896
		end
		def initialize(data = nil)
			if( data == nil )
				super(40896)
			else
				super(40896, data)
			end
		end
	end

	class PaymentStubIndex2RateSpread < Quickfix::DoubleField
		def PaymentStubIndex2RateSpread.field
			return 40897
		end
		def initialize(data = nil)
			if( data == nil )
				super(40897)
			else
				super(40897, data)
			end
		end
	end

	class PaymentStubIndex2RateSpreadPositionType < Quickfix::IntField
		def PaymentStubIndex2RateSpreadPositionType.field
			return 40898
		end
		def initialize(data = nil)
			if( data == nil )
				super(40898)
			else
				super(40898, data)
			end
		end
	end

	class PaymentStubIndex2RateTreatment < Quickfix::IntField
		def PaymentStubIndex2RateTreatment.field
			return 40899
		end
		def initialize(data = nil)
			if( data == nil )
				super(40899)
			else
				super(40899, data)
			end
		end
	end

	class PaymentStubIndex2CapRate < Quickfix::DoubleField
		def PaymentStubIndex2CapRate.field
			return 40900
		end
		def initialize(data = nil)
			if( data == nil )
				super(40900)
			else
				super(40900, data)
			end
		end
	end

	class PaymentStubIndex2FloorRate < Quickfix::DoubleField
		def PaymentStubIndex2FloorRate.field
			return 40901
		end
		def initialize(data = nil)
			if( data == nil )
				super(40901)
			else
				super(40901, data)
			end
		end
	end

	class NoLegSettlRateFallbacks < Quickfix::IntField
		def NoLegSettlRateFallbacks.field
			return 40902
		end
		def initialize(data = nil)
			if( data == nil )
				super(40902)
			else
				super(40902, data)
			end
		end
	end

	class LegSettlRatePostponementMaximumDays < Quickfix::IntField
		def LegSettlRatePostponementMaximumDays.field
			return 40903
		end
		def initialize(data = nil)
			if( data == nil )
				super(40903)
			else
				super(40903, data)
			end
		end
	end

	class UnderlyingSettlRateFallbackRateSource < Quickfix::IntField
		def UnderlyingSettlRateFallbackRateSource.field
			return 40904
		end
		def initialize(data = nil)
			if( data == nil )
				super(40904)
			else
				super(40904, data)
			end
		end
	end

	class LegSettlRatePostponementSurvey < Quickfix::BoolField
		def LegSettlRatePostponementSurvey.field
			return 40905
		end
		def initialize(data = nil)
			if( data == nil )
				super(40905)
			else
				super(40905, data)
			end
		end
	end

	class LegSettlRatePostponementCalculationAgent < Quickfix::IntField
		def LegSettlRatePostponementCalculationAgent.field
			return 40906
		end
		def initialize(data = nil)
			if( data == nil )
				super(40906)
			else
				super(40906, data)
			end
		end
	end

	class StreamEffectiveDateUnadjusted < Quickfix::StringField
		def StreamEffectiveDateUnadjusted.field
			return 40907
		end
		def initialize(data = nil)
			if( data == nil )
				super(40907)
			else
				super(40907, data)
			end
		end
	end

	class StreamEffectiveDateBusinessDayConvention < Quickfix::IntField
		def StreamEffectiveDateBusinessDayConvention.field
			return 40908
		end
		def initialize(data = nil)
			if( data == nil )
				super(40908)
			else
				super(40908, data)
			end
		end
	end

	class StreamEffectiveDateBusinessCenter < Quickfix::StringField
		def StreamEffectiveDateBusinessCenter.field
			return 40909
		end
		def initialize(data = nil)
			if( data == nil )
				super(40909)
			else
				super(40909, data)
			end
		end
	end

	class StreamEffectiveDateRelativeTo < Quickfix::IntField
		def StreamEffectiveDateRelativeTo.field
			return 40910
		end
		def initialize(data = nil)
			if( data == nil )
				super(40910)
			else
				super(40910, data)
			end
		end
	end

	class StreamEffectiveDateOffsetPeriod < Quickfix::IntField
		def StreamEffectiveDateOffsetPeriod.field
			return 40911
		end
		def initialize(data = nil)
			if( data == nil )
				super(40911)
			else
				super(40911, data)
			end
		end
	end

	class StreamEffectiveDateOffsetUnit < Quickfix::StringField
		def StreamEffectiveDateOffsetUnit.field
			return 40912
		end
		def initialize(data = nil)
			if( data == nil )
				super(40912)
			else
				super(40912, data)
			end
		end
	end

	class StreamEffectiveDateOffsetDayType < Quickfix::IntField
		def StreamEffectiveDateOffsetDayType.field
			return 40913
		end
		def initialize(data = nil)
			if( data == nil )
				super(40913)
			else
				super(40913, data)
			end
		end
	end

	class StreamEffectiveDateAdjusted < Quickfix::StringField
		def StreamEffectiveDateAdjusted.field
			return 40914
		end
		def initialize(data = nil)
			if( data == nil )
				super(40914)
			else
				super(40914, data)
			end
		end
	end

	class UnderlyingSettlRateFallbackReferencePage < Quickfix::StringField
		def UnderlyingSettlRateFallbackReferencePage.field
			return 40915
		end
		def initialize(data = nil)
			if( data == nil )
				super(40915)
			else
				super(40915, data)
			end
		end
	end

	class PaymentPriceType < Quickfix::IntField
		def PaymentPriceType.field
			return 40919
		end
		def initialize(data = nil)
			if( data == nil )
				super(40919)
			else
				super(40919, data)
			end
		end
	end

	class PaymentStreamPaymentDateOffsetDayType < Quickfix::IntField
		def PaymentStreamPaymentDateOffsetDayType.field
			return 40920
		end
		def initialize(data = nil)
			if( data == nil )
				super(40920)
			else
				super(40920, data)
			end
		end
	end

	class BusinessDayConvention < Quickfix::IntField
		def BusinessDayConvention.field
			return 40921
		end
		def initialize(data = nil)
			if( data == nil )
				super(40921)
			else
				super(40921, data)
			end
		end
	end

	class DateRollConvention < Quickfix::StringField
		def DateRollConvention.field
			return 40922
		end
		def initialize(data = nil)
			if( data == nil )
				super(40922)
			else
				super(40922, data)
			end
		end
	end

	class NoLegBusinessCenters < Quickfix::IntField
		def NoLegBusinessCenters.field
			return 40923
		end
		def initialize(data = nil)
			if( data == nil )
				super(40923)
			else
				super(40923, data)
			end
		end
	end

	class LegBusinessCenter < Quickfix::StringField
		def LegBusinessCenter.field
			return 40924
		end
		def initialize(data = nil)
			if( data == nil )
				super(40924)
			else
				super(40924, data)
			end
		end
	end

	class LegBusinessDayConvention < Quickfix::IntField
		def LegBusinessDayConvention.field
			return 40925
		end
		def initialize(data = nil)
			if( data == nil )
				super(40925)
			else
				super(40925, data)
			end
		end
	end

	class LegDateRollConvention < Quickfix::StringField
		def LegDateRollConvention.field
			return 40926
		end
		def initialize(data = nil)
			if( data == nil )
				super(40926)
			else
				super(40926, data)
			end
		end
	end

	class NoLegPaymentScheduleFixingDateBusinessCenters < Quickfix::IntField
		def NoLegPaymentScheduleFixingDateBusinessCenters.field
			return 40927
		end
		def initialize(data = nil)
			if( data == nil )
				super(40927)
			else
				super(40927, data)
			end
		end
	end

	class NoLegPaymentScheduleInterimExchangeDateBusinessCenters < Quickfix::IntField
		def NoLegPaymentScheduleInterimExchangeDateBusinessCenters.field
			return 40928
		end
		def initialize(data = nil)
			if( data == nil )
				super(40928)
			else
				super(40928, data)
			end
		end
	end

	class NoLegPaymentStreamNonDeliverableFixingDateBusinessCenters < Quickfix::IntField
		def NoLegPaymentStreamNonDeliverableFixingDateBusinessCenters.field
			return 40929
		end
		def initialize(data = nil)
			if( data == nil )
				super(40929)
			else
				super(40929, data)
			end
		end
	end

	class NoLegPaymentStreamPaymentDateBusinessCenters < Quickfix::IntField
		def NoLegPaymentStreamPaymentDateBusinessCenters.field
			return 40930
		end
		def initialize(data = nil)
			if( data == nil )
				super(40930)
			else
				super(40930, data)
			end
		end
	end

	class NoLegPaymentStreamResetDateBusinessCenters < Quickfix::IntField
		def NoLegPaymentStreamResetDateBusinessCenters.field
			return 40931
		end
		def initialize(data = nil)
			if( data == nil )
				super(40931)
			else
				super(40931, data)
			end
		end
	end

	class NoLegPaymentStreamInitialFixingDateBusinessCenters < Quickfix::IntField
		def NoLegPaymentStreamInitialFixingDateBusinessCenters.field
			return 40932
		end
		def initialize(data = nil)
			if( data == nil )
				super(40932)
			else
				super(40932, data)
			end
		end
	end

	class NoLegPaymentStreamFixingDateBusinessCenters < Quickfix::IntField
		def NoLegPaymentStreamFixingDateBusinessCenters.field
			return 40933
		end
		def initialize(data = nil)
			if( data == nil )
				super(40933)
			else
				super(40933, data)
			end
		end
	end

	class NoLegProvisionCashSettlPaymentDateBusinessCenters < Quickfix::IntField
		def NoLegProvisionCashSettlPaymentDateBusinessCenters.field
			return 40934
		end
		def initialize(data = nil)
			if( data == nil )
				super(40934)
			else
				super(40934, data)
			end
		end
	end

	class NoLegProvisionCashSettlValueDateBusinessCenters < Quickfix::IntField
		def NoLegProvisionCashSettlValueDateBusinessCenters.field
			return 40935
		end
		def initialize(data = nil)
			if( data == nil )
				super(40935)
			else
				super(40935, data)
			end
		end
	end

	class NoLegProvisionOptionExerciseBusinessCenters < Quickfix::IntField
		def NoLegProvisionOptionExerciseBusinessCenters.field
			return 40936
		end
		def initialize(data = nil)
			if( data == nil )
				super(40936)
			else
				super(40936, data)
			end
		end
	end

	class NoLegProvisionOptionExpirationDateBusinessCenters < Quickfix::IntField
		def NoLegProvisionOptionExpirationDateBusinessCenters.field
			return 40937
		end
		def initialize(data = nil)
			if( data == nil )
				super(40937)
			else
				super(40937, data)
			end
		end
	end

	class NoLegProvisionOptionRelevantUnderlyingDateBusinessCenters < Quickfix::IntField
		def NoLegProvisionOptionRelevantUnderlyingDateBusinessCenters.field
			return 40938
		end
		def initialize(data = nil)
			if( data == nil )
				super(40938)
			else
				super(40938, data)
			end
		end
	end

	class NoLegProvisionDateBusinessCenters < Quickfix::IntField
		def NoLegProvisionDateBusinessCenters.field
			return 40939
		end
		def initialize(data = nil)
			if( data == nil )
				super(40939)
			else
				super(40939, data)
			end
		end
	end

	class NoLegStreamCalculationPeriodBusinessCenters < Quickfix::IntField
		def NoLegStreamCalculationPeriodBusinessCenters.field
			return 40940
		end
		def initialize(data = nil)
			if( data == nil )
				super(40940)
			else
				super(40940, data)
			end
		end
	end

	class NoLegStreamFirstPeriodStartDateBusinessCenters < Quickfix::IntField
		def NoLegStreamFirstPeriodStartDateBusinessCenters.field
			return 40941
		end
		def initialize(data = nil)
			if( data == nil )
				super(40941)
			else
				super(40941, data)
			end
		end
	end

	class NoLegStreamEffectiveDateBusinessCenters < Quickfix::IntField
		def NoLegStreamEffectiveDateBusinessCenters.field
			return 40942
		end
		def initialize(data = nil)
			if( data == nil )
				super(40942)
			else
				super(40942, data)
			end
		end
	end

	class NoLegStreamTerminationDateBusinessCenters < Quickfix::IntField
		def NoLegStreamTerminationDateBusinessCenters.field
			return 40943
		end
		def initialize(data = nil)
			if( data == nil )
				super(40943)
			else
				super(40943, data)
			end
		end
	end

	class NoPaymentBusinessCenters < Quickfix::IntField
		def NoPaymentBusinessCenters.field
			return 40944
		end
		def initialize(data = nil)
			if( data == nil )
				super(40944)
			else
				super(40944, data)
			end
		end
	end

	class NoPaymentScheduleInterimExchangeDateBusinessCenters < Quickfix::IntField
		def NoPaymentScheduleInterimExchangeDateBusinessCenters.field
			return 40945
		end
		def initialize(data = nil)
			if( data == nil )
				super(40945)
			else
				super(40945, data)
			end
		end
	end

	class NoPaymentStreamNonDeliverableFixingDatesBusinessCenters < Quickfix::IntField
		def NoPaymentStreamNonDeliverableFixingDatesBusinessCenters.field
			return 40946
		end
		def initialize(data = nil)
			if( data == nil )
				super(40946)
			else
				super(40946, data)
			end
		end
	end

	class NoPaymentStreamPaymentDateBusinessCenters < Quickfix::IntField
		def NoPaymentStreamPaymentDateBusinessCenters.field
			return 40947
		end
		def initialize(data = nil)
			if( data == nil )
				super(40947)
			else
				super(40947, data)
			end
		end
	end

	class NoPaymentStreamResetDateBusinessCenters < Quickfix::IntField
		def NoPaymentStreamResetDateBusinessCenters.field
			return 40948
		end
		def initialize(data = nil)
			if( data == nil )
				super(40948)
			else
				super(40948, data)
			end
		end
	end

	class NoPaymentStreamInitialFixingDateBusinessCenters < Quickfix::IntField
		def NoPaymentStreamInitialFixingDateBusinessCenters.field
			return 40949
		end
		def initialize(data = nil)
			if( data == nil )
				super(40949)
			else
				super(40949, data)
			end
		end
	end

	class NoPaymentStreamFixingDateBusinessCenters < Quickfix::IntField
		def NoPaymentStreamFixingDateBusinessCenters.field
			return 40950
		end
		def initialize(data = nil)
			if( data == nil )
				super(40950)
			else
				super(40950, data)
			end
		end
	end

	class NoProtectionTermEventNewsSources < Quickfix::IntField
		def NoProtectionTermEventNewsSources.field
			return 40951
		end
		def initialize(data = nil)
			if( data == nil )
				super(40951)
			else
				super(40951, data)
			end
		end
	end

	class NoProvisionCashSettlPaymentDateBusinessCenters < Quickfix::IntField
		def NoProvisionCashSettlPaymentDateBusinessCenters.field
			return 40952
		end
		def initialize(data = nil)
			if( data == nil )
				super(40952)
			else
				super(40952, data)
			end
		end
	end

	class NoProvisionCashSettlValueDateBusinessCenters < Quickfix::IntField
		def NoProvisionCashSettlValueDateBusinessCenters.field
			return 40953
		end
		def initialize(data = nil)
			if( data == nil )
				super(40953)
			else
				super(40953, data)
			end
		end
	end

	class NoProvisionOptionExerciseBusinessCenters < Quickfix::IntField
		def NoProvisionOptionExerciseBusinessCenters.field
			return 40954
		end
		def initialize(data = nil)
			if( data == nil )
				super(40954)
			else
				super(40954, data)
			end
		end
	end

	class NoProvisionOptionExpirationDateBusinessCenters < Quickfix::IntField
		def NoProvisionOptionExpirationDateBusinessCenters.field
			return 40955
		end
		def initialize(data = nil)
			if( data == nil )
				super(40955)
			else
				super(40955, data)
			end
		end
	end

	class NoProvisionOptionRelevantUnderlyingDateBusinessCenters < Quickfix::IntField
		def NoProvisionOptionRelevantUnderlyingDateBusinessCenters.field
			return 40956
		end
		def initialize(data = nil)
			if( data == nil )
				super(40956)
			else
				super(40956, data)
			end
		end
	end

	class NoProvisionDateBusinessCenters < Quickfix::IntField
		def NoProvisionDateBusinessCenters.field
			return 40957
		end
		def initialize(data = nil)
			if( data == nil )
				super(40957)
			else
				super(40957, data)
			end
		end
	end

	class NoStreamCalculationPeriodBusinessCenters < Quickfix::IntField
		def NoStreamCalculationPeriodBusinessCenters.field
			return 40958
		end
		def initialize(data = nil)
			if( data == nil )
				super(40958)
			else
				super(40958, data)
			end
		end
	end

	class NoStreamFirstPeriodStartDateBusinessCenters < Quickfix::IntField
		def NoStreamFirstPeriodStartDateBusinessCenters.field
			return 40959
		end
		def initialize(data = nil)
			if( data == nil )
				super(40959)
			else
				super(40959, data)
			end
		end
	end

	class NoStreamEffectiveDateBusinessCenters < Quickfix::IntField
		def NoStreamEffectiveDateBusinessCenters.field
			return 40960
		end
		def initialize(data = nil)
			if( data == nil )
				super(40960)
			else
				super(40960, data)
			end
		end
	end

	class NoStreamTerminationDateBusinessCenters < Quickfix::IntField
		def NoStreamTerminationDateBusinessCenters.field
			return 40961
		end
		def initialize(data = nil)
			if( data == nil )
				super(40961)
			else
				super(40961, data)
			end
		end
	end

	class NoUnderlyingBusinessCenters < Quickfix::IntField
		def NoUnderlyingBusinessCenters.field
			return 40962
		end
		def initialize(data = nil)
			if( data == nil )
				super(40962)
			else
				super(40962, data)
			end
		end
	end

	class UnderlyingBusinessCenter < Quickfix::StringField
		def UnderlyingBusinessCenter.field
			return 40963
		end
		def initialize(data = nil)
			if( data == nil )
				super(40963)
			else
				super(40963, data)
			end
		end
	end

	class UnderlyingBusinessDayConvention < Quickfix::IntField
		def UnderlyingBusinessDayConvention.field
			return 40964
		end
		def initialize(data = nil)
			if( data == nil )
				super(40964)
			else
				super(40964, data)
			end
		end
	end

	class UnderlyingDateRollConvention < Quickfix::StringField
		def UnderlyingDateRollConvention.field
			return 40965
		end
		def initialize(data = nil)
			if( data == nil )
				super(40965)
			else
				super(40965, data)
			end
		end
	end

	class NoUnderlyingPaymentScheduleFixingDateBusinessCenters < Quickfix::IntField
		def NoUnderlyingPaymentScheduleFixingDateBusinessCenters.field
			return 40966
		end
		def initialize(data = nil)
			if( data == nil )
				super(40966)
			else
				super(40966, data)
			end
		end
	end

	class NoUnderlyingPaymentScheduleInterimExchangeDateBusinessCenters < Quickfix::IntField
		def NoUnderlyingPaymentScheduleInterimExchangeDateBusinessCenters.field
			return 40967
		end
		def initialize(data = nil)
			if( data == nil )
				super(40967)
			else
				super(40967, data)
			end
		end
	end

	class NoUnderlyingPaymentStreamNonDeliverableFixingDatesBizCenters < Quickfix::IntField
		def NoUnderlyingPaymentStreamNonDeliverableFixingDatesBizCenters.field
			return 40968
		end
		def initialize(data = nil)
			if( data == nil )
				super(40968)
			else
				super(40968, data)
			end
		end
	end

	class NoUnderlyingPaymentStreamPaymentDateBusinessCenters < Quickfix::IntField
		def NoUnderlyingPaymentStreamPaymentDateBusinessCenters.field
			return 40969
		end
		def initialize(data = nil)
			if( data == nil )
				super(40969)
			else
				super(40969, data)
			end
		end
	end

	class NoUnderlyingPaymentStreamResetDateBusinessCenters < Quickfix::IntField
		def NoUnderlyingPaymentStreamResetDateBusinessCenters.field
			return 40970
		end
		def initialize(data = nil)
			if( data == nil )
				super(40970)
			else
				super(40970, data)
			end
		end
	end

	class NoUnderlyingPaymentStreamInitialFixingDateBusinessCenters < Quickfix::IntField
		def NoUnderlyingPaymentStreamInitialFixingDateBusinessCenters.field
			return 40971
		end
		def initialize(data = nil)
			if( data == nil )
				super(40971)
			else
				super(40971, data)
			end
		end
	end

	class NoUnderlyingPaymentStreamFixingDateBusinessCenters < Quickfix::IntField
		def NoUnderlyingPaymentStreamFixingDateBusinessCenters.field
			return 40972
		end
		def initialize(data = nil)
			if( data == nil )
				super(40972)
			else
				super(40972, data)
			end
		end
	end

	class NoUnderlyingStreamCalculationPeriodBusinessCenters < Quickfix::IntField
		def NoUnderlyingStreamCalculationPeriodBusinessCenters.field
			return 40973
		end
		def initialize(data = nil)
			if( data == nil )
				super(40973)
			else
				super(40973, data)
			end
		end
	end

	class NoUnderlyingStreamFirstPeriodStartDateBusinessCenters < Quickfix::IntField
		def NoUnderlyingStreamFirstPeriodStartDateBusinessCenters.field
			return 40974
		end
		def initialize(data = nil)
			if( data == nil )
				super(40974)
			else
				super(40974, data)
			end
		end
	end

	class NoUnderlyingStreamEffectiveDateBusinessCenters < Quickfix::IntField
		def NoUnderlyingStreamEffectiveDateBusinessCenters.field
			return 40975
		end
		def initialize(data = nil)
			if( data == nil )
				super(40975)
			else
				super(40975, data)
			end
		end
	end

	class NoUnderlyingStreamTerminationDateBusinessCenters < Quickfix::IntField
		def NoUnderlyingStreamTerminationDateBusinessCenters.field
			return 40976
		end
		def initialize(data = nil)
			if( data == nil )
				super(40976)
			else
				super(40976, data)
			end
		end
	end

	class NoPaymentScheduleFixingDateBusinessCenters < Quickfix::IntField
		def NoPaymentScheduleFixingDateBusinessCenters.field
			return 40977
		end
		def initialize(data = nil)
			if( data == nil )
				super(40977)
			else
				super(40977, data)
			end
		end
	end

	class EncodedLegStreamTextLen < Quickfix::IntField
		def EncodedLegStreamTextLen.field
			return 40978
		end
		def initialize(data = nil)
			if( data == nil )
				super(40978)
			else
				super(40978, data)
			end
		end
	end

	class EncodedLegStreamText < Quickfix::StringField
		def EncodedLegStreamText.field
			return 40979
		end
		def initialize(data = nil)
			if( data == nil )
				super(40979)
			else
				super(40979, data)
			end
		end
	end

	class EncodedLegProvisionTextLen < Quickfix::IntField
		def EncodedLegProvisionTextLen.field
			return 40980
		end
		def initialize(data = nil)
			if( data == nil )
				super(40980)
			else
				super(40980, data)
			end
		end
	end

	class EncodedLegProvisionText < Quickfix::StringField
		def EncodedLegProvisionText.field
			return 40981
		end
		def initialize(data = nil)
			if( data == nil )
				super(40981)
			else
				super(40981, data)
			end
		end
	end

	class EncodedStreamTextLen < Quickfix::IntField
		def EncodedStreamTextLen.field
			return 40982
		end
		def initialize(data = nil)
			if( data == nil )
				super(40982)
			else
				super(40982, data)
			end
		end
	end

	class EncodedStreamText < Quickfix::StringField
		def EncodedStreamText.field
			return 40983
		end
		def initialize(data = nil)
			if( data == nil )
				super(40983)
			else
				super(40983, data)
			end
		end
	end

	class EncodedPaymentTextLen < Quickfix::IntField
		def EncodedPaymentTextLen.field
			return 40984
		end
		def initialize(data = nil)
			if( data == nil )
				super(40984)
			else
				super(40984, data)
			end
		end
	end

	class EncodedPaymentText < Quickfix::StringField
		def EncodedPaymentText.field
			return 40985
		end
		def initialize(data = nil)
			if( data == nil )
				super(40985)
			else
				super(40985, data)
			end
		end
	end

	class EncodedProvisionTextLen < Quickfix::IntField
		def EncodedProvisionTextLen.field
			return 40986
		end
		def initialize(data = nil)
			if( data == nil )
				super(40986)
			else
				super(40986, data)
			end
		end
	end

	class EncodedProvisionText < Quickfix::StringField
		def EncodedProvisionText.field
			return 40987
		end
		def initialize(data = nil)
			if( data == nil )
				super(40987)
			else
				super(40987, data)
			end
		end
	end

	class EncodedUnderlyingStreamTextLen < Quickfix::IntField
		def EncodedUnderlyingStreamTextLen.field
			return 40988
		end
		def initialize(data = nil)
			if( data == nil )
				super(40988)
			else
				super(40988, data)
			end
		end
	end

	class EncodedUnderlyingStreamText < Quickfix::StringField
		def EncodedUnderlyingStreamText.field
			return 40989
		end
		def initialize(data = nil)
			if( data == nil )
				super(40989)
			else
				super(40989, data)
			end
		end
	end

	class ProvisionCashSettlQuoteReferencePage < Quickfix::StringField
		def ProvisionCashSettlQuoteReferencePage.field
			return 41406
		end
		def initialize(data = nil)
			if( data == nil )
				super(41406)
			else
				super(41406, data)
			end
		end
	end

	class LegProvisionCashSettlQuoteReferencePage < Quickfix::StringField
		def LegProvisionCashSettlQuoteReferencePage.field
			return 41407
		end
		def initialize(data = nil)
			if( data == nil )
				super(41407)
			else
				super(41407, data)
			end
		end
	end

	class EventMonthYear < Quickfix::StringField
		def EventMonthYear.field
			return 2340
		end
		def initialize(data = nil)
			if( data == nil )
				super(2340)
			else
				super(2340, data)
			end
		end
	end

	class LegEventMonthYear < Quickfix::StringField
		def LegEventMonthYear.field
			return 2341
		end
		def initialize(data = nil)
			if( data == nil )
				super(2341)
			else
				super(2341, data)
			end
		end
	end

	class UnderlyingEventMonthYear < Quickfix::StringField
		def UnderlyingEventMonthYear.field
			return 2342
		end
		def initialize(data = nil)
			if( data == nil )
				super(2342)
			else
				super(2342, data)
			end
		end
	end

	class PreviousClearingBusinessDate < Quickfix::StringField
		def PreviousClearingBusinessDate.field
			return 2084
		end
		def initialize(data = nil)
			if( data == nil )
				super(2084)
			else
				super(2084, data)
			end
		end
	end

	class ValuationDate < Quickfix::StringField
		def ValuationDate.field
			return 2085
		end
		def initialize(data = nil)
			if( data == nil )
				super(2085)
			else
				super(2085, data)
			end
		end
	end

	class ValuationTime < Quickfix::StringField
		def ValuationTime.field
			return 2086
		end
		def initialize(data = nil)
			if( data == nil )
				super(2086)
			else
				super(2086, data)
			end
		end
	end

	class ValuationBusinessCenter < Quickfix::StringField
		def ValuationBusinessCenter.field
			return 2087
		end
		def initialize(data = nil)
			if( data == nil )
				super(2087)
			else
				super(2087, data)
			end
		end
	end

	class MarginAmtFXRate < Quickfix::DoubleField
		def MarginAmtFXRate.field
			return 2088
		end
		def initialize(data = nil)
			if( data == nil )
				super(2088)
			else
				super(2088, data)
			end
		end
	end

	class MarginAmtFXRateCalc < Quickfix::CharField
		def MarginAmtFXRateCalc.field
			return 2089
		end
		def initialize(data = nil)
			if( data == nil )
				super(2089)
			else
				super(2089, data)
			end
		end
	end

	class CollateralFXRate < Quickfix::DoubleField
		def CollateralFXRate.field
			return 2090
		end
		def initialize(data = nil)
			if( data == nil )
				super(2090)
			else
				super(2090, data)
			end
		end
	end

	class CollateralFXRateCalc < Quickfix::CharField
		def CollateralFXRateCalc.field
			return 2091
		end
		def initialize(data = nil)
			if( data == nil )
				super(2091)
			else
				super(2091, data)
			end
		end
	end

	class CollateralAmountMarketSegmentID < Quickfix::StringField
		def CollateralAmountMarketSegmentID.field
			return 2092
		end
		def initialize(data = nil)
			if( data == nil )
				super(2092)
			else
				super(2092, data)
			end
		end
	end

	class CollateralAmountMarketID < Quickfix::StringField
		def CollateralAmountMarketID.field
			return 2093
		end
		def initialize(data = nil)
			if( data == nil )
				super(2093)
			else
				super(2093, data)
			end
		end
	end

	class PayCollectFXRate < Quickfix::DoubleField
		def PayCollectFXRate.field
			return 2094
		end
		def initialize(data = nil)
			if( data == nil )
				super(2094)
			else
				super(2094, data)
			end
		end
	end

	class PayCollectFXRateCalc < Quickfix::CharField
		def PayCollectFXRateCalc.field
			return 2095
		end
		def initialize(data = nil)
			if( data == nil )
				super(2095)
			else
				super(2095, data)
			end
		end
	end

	class PosAmtStreamDesc < Quickfix::StringField
		def PosAmtStreamDesc.field
			return 2096
		end
		def initialize(data = nil)
			if( data == nil )
				super(2096)
			else
				super(2096, data)
			end
		end
	end

	class PositionFXRate < Quickfix::DoubleField
		def PositionFXRate.field
			return 2097
		end
		def initialize(data = nil)
			if( data == nil )
				super(2097)
			else
				super(2097, data)
			end
		end
	end

	class PositionFXRateCalc < Quickfix::CharField
		def PositionFXRateCalc.field
			return 2098
		end
		def initialize(data = nil)
			if( data == nil )
				super(2098)
			else
				super(2098, data)
			end
		end
	end

	class PosAmtMarketSegmentID < Quickfix::StringField
		def PosAmtMarketSegmentID.field
			return 2099
		end
		def initialize(data = nil)
			if( data == nil )
				super(2099)
			else
				super(2099, data)
			end
		end
	end

	class PosAmtMarketID < Quickfix::StringField
		def PosAmtMarketID.field
			return 2100
		end
		def initialize(data = nil)
			if( data == nil )
				super(2100)
			else
				super(2100, data)
			end
		end
	end

	class TerminatedIndicator < Quickfix::BoolField
		def TerminatedIndicator.field
			return 2101
		end
		def initialize(data = nil)
			if( data == nil )
				super(2101)
			else
				super(2101, data)
			end
		end
	end

	class ShortMarkingExemptIndicator < Quickfix::BoolField
		def ShortMarkingExemptIndicator.field
			return 2102
		end
		def initialize(data = nil)
			if( data == nil )
				super(2102)
			else
				super(2102, data)
			end
		end
	end

	class RelatedRegulatoryTradeIDSource < Quickfix::StringField
		def RelatedRegulatoryTradeIDSource.field
			return 2103
		end
		def initialize(data = nil)
			if( data == nil )
				super(2103)
			else
				super(2103, data)
			end
		end
	end

	class NoAttachments < Quickfix::IntField
		def NoAttachments.field
			return 2104
		end
		def initialize(data = nil)
			if( data == nil )
				super(2104)
			else
				super(2104, data)
			end
		end
	end

	class AttachmentName < Quickfix::StringField
		def AttachmentName.field
			return 2105
		end
		def initialize(data = nil)
			if( data == nil )
				super(2105)
			else
				super(2105, data)
			end
		end
	end

	class AttachmentMediaType < Quickfix::StringField
		def AttachmentMediaType.field
			return 2106
		end
		def initialize(data = nil)
			if( data == nil )
				super(2106)
			else
				super(2106, data)
			end
		end
	end

	class AttachmentClassification < Quickfix::StringField
		def AttachmentClassification.field
			return 2107
		end
		def initialize(data = nil)
			if( data == nil )
				super(2107)
			else
				super(2107, data)
			end
		end
	end

	class AttachmentExternalURL < Quickfix::StringField
		def AttachmentExternalURL.field
			return 2108
		end
		def initialize(data = nil)
			if( data == nil )
				super(2108)
			else
				super(2108, data)
			end
		end
	end

	class AttachmentEncodingType < Quickfix::IntField
		def AttachmentEncodingType.field
			return 2109
		end
		def initialize(data = nil)
			if( data == nil )
				super(2109)
			else
				super(2109, data)
			end
		end
	end

	class UnencodedAttachmentLen < Quickfix::IntField
		def UnencodedAttachmentLen.field
			return 2110
		end
		def initialize(data = nil)
			if( data == nil )
				super(2110)
			else
				super(2110, data)
			end
		end
	end

	class EncodedAttachmentLen < Quickfix::IntField
		def EncodedAttachmentLen.field
			return 2111
		end
		def initialize(data = nil)
			if( data == nil )
				super(2111)
			else
				super(2111, data)
			end
		end
	end

	class EncodedAttachment < Quickfix::StringField
		def EncodedAttachment.field
			return 2112
		end
		def initialize(data = nil)
			if( data == nil )
				super(2112)
			else
				super(2112, data)
			end
		end
	end

	class NoAttachmentKeywords < Quickfix::IntField
		def NoAttachmentKeywords.field
			return 2113
		end
		def initialize(data = nil)
			if( data == nil )
				super(2113)
			else
				super(2113, data)
			end
		end
	end

	class AttachmentKeyword < Quickfix::StringField
		def AttachmentKeyword.field
			return 2114
		end
		def initialize(data = nil)
			if( data == nil )
				super(2114)
			else
				super(2114, data)
			end
		end
	end

	class NegotiationMethod < Quickfix::IntField
		def NegotiationMethod.field
			return 2115
		end
		def initialize(data = nil)
			if( data == nil )
				super(2115)
			else
				super(2115, data)
			end
		end
	end

	class NextAuctionTime < Quickfix::UtcTimeStampField
		def NextAuctionTime.field
			return 2116
		end
		def initialize(data = nil)
			if( data == nil )
				super(2116)
			else
				super(2116, data)
			end
		end
	end

	class NoAssetAttributes < Quickfix::IntField
		def NoAssetAttributes.field
			return 2304
		end
		def initialize(data = nil)
			if( data == nil )
				super(2304)
			else
				super(2304, data)
			end
		end
	end

	class AssetAttributeType < Quickfix::StringField
		def AssetAttributeType.field
			return 2305
		end
		def initialize(data = nil)
			if( data == nil )
				super(2305)
			else
				super(2305, data)
			end
		end
	end

	class AssetAttributeValue < Quickfix::StringField
		def AssetAttributeValue.field
			return 2306
		end
		def initialize(data = nil)
			if( data == nil )
				super(2306)
			else
				super(2306, data)
			end
		end
	end

	class AssetAttributeLimit < Quickfix::StringField
		def AssetAttributeLimit.field
			return 2307
		end
		def initialize(data = nil)
			if( data == nil )
				super(2307)
			else
				super(2307, data)
			end
		end
	end

	class CommRate < Quickfix::DoubleField
		def CommRate.field
			return 1233
		end
		def initialize(data = nil)
			if( data == nil )
				super(1233)
			else
				super(1233, data)
			end
		end
	end

	class CommUnitOfMeasure < Quickfix::StringField
		def CommUnitOfMeasure.field
			return 1238
		end
		def initialize(data = nil)
			if( data == nil )
				super(1238)
			else
				super(1238, data)
			end
		end
	end

	class NoComplexEventAveragingObservations < Quickfix::IntField
		def NoComplexEventAveragingObservations.field
			return 40994
		end
		def initialize(data = nil)
			if( data == nil )
				super(40994)
			else
				super(40994, data)
			end
		end
	end

	class ComplexEventAveragingObservationNumber < Quickfix::IntField
		def ComplexEventAveragingObservationNumber.field
			return 40995
		end
		def initialize(data = nil)
			if( data == nil )
				super(40995)
			else
				super(40995, data)
			end
		end
	end

	class ComplexEventAveragingWeight < Quickfix::DoubleField
		def ComplexEventAveragingWeight.field
			return 40996
		end
		def initialize(data = nil)
			if( data == nil )
				super(40996)
			else
				super(40996, data)
			end
		end
	end

	class NoComplexEventCreditEvents < Quickfix::IntField
		def NoComplexEventCreditEvents.field
			return 40997
		end
		def initialize(data = nil)
			if( data == nil )
				super(40997)
			else
				super(40997, data)
			end
		end
	end

	class ComplexEventCreditEventType < Quickfix::StringField
		def ComplexEventCreditEventType.field
			return 40998
		end
		def initialize(data = nil)
			if( data == nil )
				super(40998)
			else
				super(40998, data)
			end
		end
	end

	class ComplexEventCreditEventValue < Quickfix::StringField
		def ComplexEventCreditEventValue.field
			return 40999
		end
		def initialize(data = nil)
			if( data == nil )
				super(40999)
			else
				super(40999, data)
			end
		end
	end

	class ComplexEventCreditEventCurrency < Quickfix::StringField
		def ComplexEventCreditEventCurrency.field
			return 41000
		end
		def initialize(data = nil)
			if( data == nil )
				super(41000)
			else
				super(41000, data)
			end
		end
	end

	class ComplexEventCreditEventPeriod < Quickfix::IntField
		def ComplexEventCreditEventPeriod.field
			return 41001
		end
		def initialize(data = nil)
			if( data == nil )
				super(41001)
			else
				super(41001, data)
			end
		end
	end

	class ComplexEventCreditEventUnit < Quickfix::StringField
		def ComplexEventCreditEventUnit.field
			return 41002
		end
		def initialize(data = nil)
			if( data == nil )
				super(41002)
			else
				super(41002, data)
			end
		end
	end

	class ComplexEventCreditEventDayType < Quickfix::IntField
		def ComplexEventCreditEventDayType.field
			return 41003
		end
		def initialize(data = nil)
			if( data == nil )
				super(41003)
			else
				super(41003, data)
			end
		end
	end

	class ComplexEventCreditEventRateSource < Quickfix::IntField
		def ComplexEventCreditEventRateSource.field
			return 41004
		end
		def initialize(data = nil)
			if( data == nil )
				super(41004)
			else
				super(41004, data)
			end
		end
	end

	class NoComplexEventCreditEventQualifiers < Quickfix::IntField
		def NoComplexEventCreditEventQualifiers.field
			return 41005
		end
		def initialize(data = nil)
			if( data == nil )
				super(41005)
			else
				super(41005, data)
			end
		end
	end

	class ComplexEventCreditEventQualifier < Quickfix::CharField
		def ComplexEventCreditEventQualifier.field
			return 41006
		end
		def initialize(data = nil)
			if( data == nil )
				super(41006)
			else
				super(41006, data)
			end
		end
	end

	class NoComplexEventPeriodDateTimes < Quickfix::IntField
		def NoComplexEventPeriodDateTimes.field
			return 41007
		end
		def initialize(data = nil)
			if( data == nil )
				super(41007)
			else
				super(41007, data)
			end
		end
	end

	class ComplexEventPeriodDate < Quickfix::StringField
		def ComplexEventPeriodDate.field
			return 41008
		end
		def initialize(data = nil)
			if( data == nil )
				super(41008)
			else
				super(41008, data)
			end
		end
	end

	class ComplexEventPeriodTime < Quickfix::StringField
		def ComplexEventPeriodTime.field
			return 41009
		end
		def initialize(data = nil)
			if( data == nil )
				super(41009)
			else
				super(41009, data)
			end
		end
	end

	class NoComplexEventPeriods < Quickfix::IntField
		def NoComplexEventPeriods.field
			return 41010
		end
		def initialize(data = nil)
			if( data == nil )
				super(41010)
			else
				super(41010, data)
			end
		end
	end

	class ComplexEventPeriodType < Quickfix::IntField
		def ComplexEventPeriodType.field
			return 41011
		end
		def initialize(data = nil)
			if( data == nil )
				super(41011)
			else
				super(41011, data)
			end
		end
	end

	class ComplexEventBusinessCenter < Quickfix::StringField
		def ComplexEventBusinessCenter.field
			return 41012
		end
		def initialize(data = nil)
			if( data == nil )
				super(41012)
			else
				super(41012, data)
			end
		end
	end

	class NoComplexEventRateSources < Quickfix::IntField
		def NoComplexEventRateSources.field
			return 41013
		end
		def initialize(data = nil)
			if( data == nil )
				super(41013)
			else
				super(41013, data)
			end
		end
	end

	class ComplexEventRateSource < Quickfix::IntField
		def ComplexEventRateSource.field
			return 41014
		end
		def initialize(data = nil)
			if( data == nil )
				super(41014)
			else
				super(41014, data)
			end
		end
	end

	class ComplexEventRateSourceType < Quickfix::IntField
		def ComplexEventRateSourceType.field
			return 41015
		end
		def initialize(data = nil)
			if( data == nil )
				super(41015)
			else
				super(41015, data)
			end
		end
	end

	class ComplexEventReferencePage < Quickfix::StringField
		def ComplexEventReferencePage.field
			return 41016
		end
		def initialize(data = nil)
			if( data == nil )
				super(41016)
			else
				super(41016, data)
			end
		end
	end

	class ComplexEventReferencePageHeading < Quickfix::StringField
		def ComplexEventReferencePageHeading.field
			return 41017
		end
		def initialize(data = nil)
			if( data == nil )
				super(41017)
			else
				super(41017, data)
			end
		end
	end

	class NoComplexEventDateBusinessCenters < Quickfix::IntField
		def NoComplexEventDateBusinessCenters.field
			return 41018
		end
		def initialize(data = nil)
			if( data == nil )
				super(41018)
			else
				super(41018, data)
			end
		end
	end

	class ComplexEventDateBusinessCenter < Quickfix::StringField
		def ComplexEventDateBusinessCenter.field
			return 41019
		end
		def initialize(data = nil)
			if( data == nil )
				super(41019)
			else
				super(41019, data)
			end
		end
	end

	class ComplexEventDateUnadjusted < Quickfix::StringField
		def ComplexEventDateUnadjusted.field
			return 41020
		end
		def initialize(data = nil)
			if( data == nil )
				super(41020)
			else
				super(41020, data)
			end
		end
	end

	class ComplexEventDateRelativeTo < Quickfix::IntField
		def ComplexEventDateRelativeTo.field
			return 41021
		end
		def initialize(data = nil)
			if( data == nil )
				super(41021)
			else
				super(41021, data)
			end
		end
	end

	class ComplexEventDateOffsetPeriod < Quickfix::IntField
		def ComplexEventDateOffsetPeriod.field
			return 41022
		end
		def initialize(data = nil)
			if( data == nil )
				super(41022)
			else
				super(41022, data)
			end
		end
	end

	class ComplexEventDateOffsetUnit < Quickfix::StringField
		def ComplexEventDateOffsetUnit.field
			return 41023
		end
		def initialize(data = nil)
			if( data == nil )
				super(41023)
			else
				super(41023, data)
			end
		end
	end

	class ComplexEventDateOffsetDayType < Quickfix::IntField
		def ComplexEventDateOffsetDayType.field
			return 41024
		end
		def initialize(data = nil)
			if( data == nil )
				super(41024)
			else
				super(41024, data)
			end
		end
	end

	class ComplexEventDateBusinessDayConvention < Quickfix::IntField
		def ComplexEventDateBusinessDayConvention.field
			return 41025
		end
		def initialize(data = nil)
			if( data == nil )
				super(41025)
			else
				super(41025, data)
			end
		end
	end

	class ComplexEventDateAdjusted < Quickfix::StringField
		def ComplexEventDateAdjusted.field
			return 41026
		end
		def initialize(data = nil)
			if( data == nil )
				super(41026)
			else
				super(41026, data)
			end
		end
	end

	class ComplexEventFixingTime < Quickfix::StringField
		def ComplexEventFixingTime.field
			return 41027
		end
		def initialize(data = nil)
			if( data == nil )
				super(41027)
			else
				super(41027, data)
			end
		end
	end

	class ComplexEventFixingTimeBusinessCenter < Quickfix::StringField
		def ComplexEventFixingTimeBusinessCenter.field
			return 41028
		end
		def initialize(data = nil)
			if( data == nil )
				super(41028)
			else
				super(41028, data)
			end
		end
	end

	class NoComplexEventCreditEventSources < Quickfix::IntField
		def NoComplexEventCreditEventSources.field
			return 41029
		end
		def initialize(data = nil)
			if( data == nil )
				super(41029)
			else
				super(41029, data)
			end
		end
	end

	class ComplexEventCreditEventSource < Quickfix::StringField
		def ComplexEventCreditEventSource.field
			return 41030
		end
		def initialize(data = nil)
			if( data == nil )
				super(41030)
			else
				super(41030, data)
			end
		end
	end

	class ComplexOptPayoutPaySide < Quickfix::IntField
		def ComplexOptPayoutPaySide.field
			return 2117
		end
		def initialize(data = nil)
			if( data == nil )
				super(2117)
			else
				super(2117, data)
			end
		end
	end

	class ComplexOptPayoutReceiveSide < Quickfix::IntField
		def ComplexOptPayoutReceiveSide.field
			return 2118
		end
		def initialize(data = nil)
			if( data == nil )
				super(2118)
			else
				super(2118, data)
			end
		end
	end

	class ComplexOptPayoutUnderlier < Quickfix::StringField
		def ComplexOptPayoutUnderlier.field
			return 2119
		end
		def initialize(data = nil)
			if( data == nil )
				super(2119)
			else
				super(2119, data)
			end
		end
	end

	class ComplexOptPayoutPercentage < Quickfix::DoubleField
		def ComplexOptPayoutPercentage.field
			return 2120
		end
		def initialize(data = nil)
			if( data == nil )
				super(2120)
			else
				super(2120, data)
			end
		end
	end

	class ComplexOptPayoutTime < Quickfix::IntField
		def ComplexOptPayoutTime.field
			return 2121
		end
		def initialize(data = nil)
			if( data == nil )
				super(2121)
			else
				super(2121, data)
			end
		end
	end

	class ComplexOptPayoutCurrency < Quickfix::StringField
		def ComplexOptPayoutCurrency.field
			return 2122
		end
		def initialize(data = nil)
			if( data == nil )
				super(2122)
			else
				super(2122, data)
			end
		end
	end

	class ComplexEventPricePercentage < Quickfix::DoubleField
		def ComplexEventPricePercentage.field
			return 2123
		end
		def initialize(data = nil)
			if( data == nil )
				super(2123)
			else
				super(2123, data)
			end
		end
	end

	class ComplexEventCurrencyOne < Quickfix::StringField
		def ComplexEventCurrencyOne.field
			return 2124
		end
		def initialize(data = nil)
			if( data == nil )
				super(2124)
			else
				super(2124, data)
			end
		end
	end

	class ComplexEventCurrencyTwo < Quickfix::StringField
		def ComplexEventCurrencyTwo.field
			return 2125
		end
		def initialize(data = nil)
			if( data == nil )
				super(2125)
			else
				super(2125, data)
			end
		end
	end

	class ComplexEventQuoteBasis < Quickfix::IntField
		def ComplexEventQuoteBasis.field
			return 2126
		end
		def initialize(data = nil)
			if( data == nil )
				super(2126)
			else
				super(2126, data)
			end
		end
	end

	class ComplexEventFixedFXRate < Quickfix::DoubleField
		def ComplexEventFixedFXRate.field
			return 2127
		end
		def initialize(data = nil)
			if( data == nil )
				super(2127)
			else
				super(2127, data)
			end
		end
	end

	class ComplexEventDeterminationMethod < Quickfix::StringField
		def ComplexEventDeterminationMethod.field
			return 2128
		end
		def initialize(data = nil)
			if( data == nil )
				super(2128)
			else
				super(2128, data)
			end
		end
	end

	class ComplexEventCalculationAgent < Quickfix::IntField
		def ComplexEventCalculationAgent.field
			return 2129
		end
		def initialize(data = nil)
			if( data == nil )
				super(2129)
			else
				super(2129, data)
			end
		end
	end

	class ComplexEventStrikePrice < Quickfix::DoubleField
		def ComplexEventStrikePrice.field
			return 2130
		end
		def initialize(data = nil)
			if( data == nil )
				super(2130)
			else
				super(2130, data)
			end
		end
	end

	class ComplexEventStrikeFactor < Quickfix::DoubleField
		def ComplexEventStrikeFactor.field
			return 2131
		end
		def initialize(data = nil)
			if( data == nil )
				super(2131)
			else
				super(2131, data)
			end
		end
	end

	class ComplexEventStrikeNumberOfOptions < Quickfix::IntField
		def ComplexEventStrikeNumberOfOptions.field
			return 2132
		end
		def initialize(data = nil)
			if( data == nil )
				super(2132)
			else
				super(2132, data)
			end
		end
	end

	class ComplexEventCreditEventsXIDRef < Quickfix::StringField
		def ComplexEventCreditEventsXIDRef.field
			return 2133
		end
		def initialize(data = nil)
			if( data == nil )
				super(2133)
			else
				super(2133, data)
			end
		end
	end

	class ComplexEventCreditEventNotifyingParty < Quickfix::IntField
		def ComplexEventCreditEventNotifyingParty.field
			return 2134
		end
		def initialize(data = nil)
			if( data == nil )
				super(2134)
			else
				super(2134, data)
			end
		end
	end

	class ComplexEventCreditEventBusinessCenter < Quickfix::StringField
		def ComplexEventCreditEventBusinessCenter.field
			return 2135
		end
		def initialize(data = nil)
			if( data == nil )
				super(2135)
			else
				super(2135, data)
			end
		end
	end

	class ComplexEventCreditEventStandardSources < Quickfix::BoolField
		def ComplexEventCreditEventStandardSources.field
			return 2136
		end
		def initialize(data = nil)
			if( data == nil )
				super(2136)
			else
				super(2136, data)
			end
		end
	end

	class ComplexEventCreditEventMinimumSources < Quickfix::IntField
		def ComplexEventCreditEventMinimumSources.field
			return 2137
		end
		def initialize(data = nil)
			if( data == nil )
				super(2137)
			else
				super(2137, data)
			end
		end
	end

	class ComplexEventXID < Quickfix::StringField
		def ComplexEventXID.field
			return 2138
		end
		def initialize(data = nil)
			if( data == nil )
				super(2138)
			else
				super(2138, data)
			end
		end
	end

	class ComplexEventXIDRef < Quickfix::StringField
		def ComplexEventXIDRef.field
			return 2139
		end
		def initialize(data = nil)
			if( data == nil )
				super(2139)
			else
				super(2139, data)
			end
		end
	end

	class NoComplexEventSchedules < Quickfix::IntField
		def NoComplexEventSchedules.field
			return 41031
		end
		def initialize(data = nil)
			if( data == nil )
				super(41031)
			else
				super(41031, data)
			end
		end
	end

	class ComplexEventScheduleStartDate < Quickfix::StringField
		def ComplexEventScheduleStartDate.field
			return 41032
		end
		def initialize(data = nil)
			if( data == nil )
				super(41032)
			else
				super(41032, data)
			end
		end
	end

	class ComplexEventScheduleEndDate < Quickfix::StringField
		def ComplexEventScheduleEndDate.field
			return 41033
		end
		def initialize(data = nil)
			if( data == nil )
				super(41033)
			else
				super(41033, data)
			end
		end
	end

	class ComplexEventScheduleFrequencyPeriod < Quickfix::IntField
		def ComplexEventScheduleFrequencyPeriod.field
			return 41034
		end
		def initialize(data = nil)
			if( data == nil )
				super(41034)
			else
				super(41034, data)
			end
		end
	end

	class ComplexEventScheduleFrequencyUnit < Quickfix::StringField
		def ComplexEventScheduleFrequencyUnit.field
			return 41035
		end
		def initialize(data = nil)
			if( data == nil )
				super(41035)
			else
				super(41035, data)
			end
		end
	end

	class ComplexEventScheduleRollConvention < Quickfix::StringField
		def ComplexEventScheduleRollConvention.field
			return 41036
		end
		def initialize(data = nil)
			if( data == nil )
				super(41036)
			else
				super(41036, data)
			end
		end
	end

	class NoDeliverySchedules < Quickfix::IntField
		def NoDeliverySchedules.field
			return 41037
		end
		def initialize(data = nil)
			if( data == nil )
				super(41037)
			else
				super(41037, data)
			end
		end
	end

	class DeliveryScheduleType < Quickfix::IntField
		def DeliveryScheduleType.field
			return 41038
		end
		def initialize(data = nil)
			if( data == nil )
				super(41038)
			else
				super(41038, data)
			end
		end
	end

	class DeliveryScheduleXID < Quickfix::StringField
		def DeliveryScheduleXID.field
			return 41039
		end
		def initialize(data = nil)
			if( data == nil )
				super(41039)
			else
				super(41039, data)
			end
		end
	end

	class DeliveryScheduleNotional < Quickfix::DoubleField
		def DeliveryScheduleNotional.field
			return 41040
		end
		def initialize(data = nil)
			if( data == nil )
				super(41040)
			else
				super(41040, data)
			end
		end
	end

	class DeliveryScheduleNotionalUnitOfMeasure < Quickfix::StringField
		def DeliveryScheduleNotionalUnitOfMeasure.field
			return 41041
		end
		def initialize(data = nil)
			if( data == nil )
				super(41041)
			else
				super(41041, data)
			end
		end
	end

	class DeliveryScheduleNotionalCommodityFrequency < Quickfix::IntField
		def DeliveryScheduleNotionalCommodityFrequency.field
			return 41042
		end
		def initialize(data = nil)
			if( data == nil )
				super(41042)
			else
				super(41042, data)
			end
		end
	end

	class DeliveryScheduleNegativeTolerance < Quickfix::DoubleField
		def DeliveryScheduleNegativeTolerance.field
			return 41043
		end
		def initialize(data = nil)
			if( data == nil )
				super(41043)
			else
				super(41043, data)
			end
		end
	end

	class DeliverySchedulePositiveTolerance < Quickfix::DoubleField
		def DeliverySchedulePositiveTolerance.field
			return 41044
		end
		def initialize(data = nil)
			if( data == nil )
				super(41044)
			else
				super(41044, data)
			end
		end
	end

	class DeliveryScheduleToleranceUnitOfMeasure < Quickfix::StringField
		def DeliveryScheduleToleranceUnitOfMeasure.field
			return 41045
		end
		def initialize(data = nil)
			if( data == nil )
				super(41045)
			else
				super(41045, data)
			end
		end
	end

	class DeliveryScheduleToleranceType < Quickfix::IntField
		def DeliveryScheduleToleranceType.field
			return 41046
		end
		def initialize(data = nil)
			if( data == nil )
				super(41046)
			else
				super(41046, data)
			end
		end
	end

	class DeliveryScheduleSettlCountry < Quickfix::StringField
		def DeliveryScheduleSettlCountry.field
			return 41047
		end
		def initialize(data = nil)
			if( data == nil )
				super(41047)
			else
				super(41047, data)
			end
		end
	end

	class DeliveryScheduleSettlTimeZone < Quickfix::StringField
		def DeliveryScheduleSettlTimeZone.field
			return 41048
		end
		def initialize(data = nil)
			if( data == nil )
				super(41048)
			else
				super(41048, data)
			end
		end
	end

	class DeliveryScheduleSettlFlowType < Quickfix::IntField
		def DeliveryScheduleSettlFlowType.field
			return 41049
		end
		def initialize(data = nil)
			if( data == nil )
				super(41049)
			else
				super(41049, data)
			end
		end
	end

	class DeliveryScheduleSettlHolidaysProcessingInstruction < Quickfix::IntField
		def DeliveryScheduleSettlHolidaysProcessingInstruction.field
			return 41050
		end
		def initialize(data = nil)
			if( data == nil )
				super(41050)
			else
				super(41050, data)
			end
		end
	end

	class NoDeliveryScheduleSettlDays < Quickfix::IntField
		def NoDeliveryScheduleSettlDays.field
			return 41051
		end
		def initialize(data = nil)
			if( data == nil )
				super(41051)
			else
				super(41051, data)
			end
		end
	end

	class DeliveryScheduleSettlDay < Quickfix::IntField
		def DeliveryScheduleSettlDay.field
			return 41052
		end
		def initialize(data = nil)
			if( data == nil )
				super(41052)
			else
				super(41052, data)
			end
		end
	end

	class DeliveryScheduleSettlTotalHours < Quickfix::IntField
		def DeliveryScheduleSettlTotalHours.field
			return 41053
		end
		def initialize(data = nil)
			if( data == nil )
				super(41053)
			else
				super(41053, data)
			end
		end
	end

	class NoDeliveryScheduleSettlTimes < Quickfix::IntField
		def NoDeliveryScheduleSettlTimes.field
			return 41054
		end
		def initialize(data = nil)
			if( data == nil )
				super(41054)
			else
				super(41054, data)
			end
		end
	end

	class DeliveryScheduleSettlStart < Quickfix::StringField
		def DeliveryScheduleSettlStart.field
			return 41055
		end
		def initialize(data = nil)
			if( data == nil )
				super(41055)
			else
				super(41055, data)
			end
		end
	end

	class DeliveryScheduleSettlEnd < Quickfix::StringField
		def DeliveryScheduleSettlEnd.field
			return 41056
		end
		def initialize(data = nil)
			if( data == nil )
				super(41056)
			else
				super(41056, data)
			end
		end
	end

	class DeliveryScheduleSettlTimeType < Quickfix::IntField
		def DeliveryScheduleSettlTimeType.field
			return 41057
		end
		def initialize(data = nil)
			if( data == nil )
				super(41057)
			else
				super(41057, data)
			end
		end
	end

	class DeliveryStreamType < Quickfix::IntField
		def DeliveryStreamType.field
			return 41058
		end
		def initialize(data = nil)
			if( data == nil )
				super(41058)
			else
				super(41058, data)
			end
		end
	end

	class DeliveryStreamPipeline < Quickfix::StringField
		def DeliveryStreamPipeline.field
			return 41059
		end
		def initialize(data = nil)
			if( data == nil )
				super(41059)
			else
				super(41059, data)
			end
		end
	end

	class DeliveryStreamEntryPoint < Quickfix::StringField
		def DeliveryStreamEntryPoint.field
			return 41060
		end
		def initialize(data = nil)
			if( data == nil )
				super(41060)
			else
				super(41060, data)
			end
		end
	end

	class DeliveryStreamWithdrawalPoint < Quickfix::StringField
		def DeliveryStreamWithdrawalPoint.field
			return 41061
		end
		def initialize(data = nil)
			if( data == nil )
				super(41061)
			else
				super(41061, data)
			end
		end
	end

	class DeliveryStreamDeliveryPoint < Quickfix::StringField
		def DeliveryStreamDeliveryPoint.field
			return 41062
		end
		def initialize(data = nil)
			if( data == nil )
				super(41062)
			else
				super(41062, data)
			end
		end
	end

	class DeliveryStreamDeliveryRestriction < Quickfix::IntField
		def DeliveryStreamDeliveryRestriction.field
			return 41063
		end
		def initialize(data = nil)
			if( data == nil )
				super(41063)
			else
				super(41063, data)
			end
		end
	end

	class DeliveryStreamDeliveryContingency < Quickfix::StringField
		def DeliveryStreamDeliveryContingency.field
			return 41064
		end
		def initialize(data = nil)
			if( data == nil )
				super(41064)
			else
				super(41064, data)
			end
		end
	end

	class DeliveryStreamDeliveryContingentPartySide < Quickfix::IntField
		def DeliveryStreamDeliveryContingentPartySide.field
			return 41065
		end
		def initialize(data = nil)
			if( data == nil )
				super(41065)
			else
				super(41065, data)
			end
		end
	end

	class DeliveryStreamDeliverAtSourceIndicator < Quickfix::BoolField
		def DeliveryStreamDeliverAtSourceIndicator.field
			return 41066
		end
		def initialize(data = nil)
			if( data == nil )
				super(41066)
			else
				super(41066, data)
			end
		end
	end

	class DeliveryStreamRiskApportionment < Quickfix::StringField
		def DeliveryStreamRiskApportionment.field
			return 41067
		end
		def initialize(data = nil)
			if( data == nil )
				super(41067)
			else
				super(41067, data)
			end
		end
	end

	class DeliveryStreamRiskApportionmentSource < Quickfix::StringField
		def DeliveryStreamRiskApportionmentSource.field
			return 41218
		end
		def initialize(data = nil)
			if( data == nil )
				super(41218)
			else
				super(41218, data)
			end
		end
	end

	class DeliveryStreamTitleTransferLocation < Quickfix::StringField
		def DeliveryStreamTitleTransferLocation.field
			return 41068
		end
		def initialize(data = nil)
			if( data == nil )
				super(41068)
			else
				super(41068, data)
			end
		end
	end

	class DeliveryStreamTitleTransferCondition < Quickfix::IntField
		def DeliveryStreamTitleTransferCondition.field
			return 41069
		end
		def initialize(data = nil)
			if( data == nil )
				super(41069)
			else
				super(41069, data)
			end
		end
	end

	class DeliveryStreamImporterOfRecord < Quickfix::StringField
		def DeliveryStreamImporterOfRecord.field
			return 41070
		end
		def initialize(data = nil)
			if( data == nil )
				super(41070)
			else
				super(41070, data)
			end
		end
	end

	class DeliveryStreamNegativeTolerance < Quickfix::DoubleField
		def DeliveryStreamNegativeTolerance.field
			return 41071
		end
		def initialize(data = nil)
			if( data == nil )
				super(41071)
			else
				super(41071, data)
			end
		end
	end

	class DeliveryStreamPositiveTolerance < Quickfix::DoubleField
		def DeliveryStreamPositiveTolerance.field
			return 41072
		end
		def initialize(data = nil)
			if( data == nil )
				super(41072)
			else
				super(41072, data)
			end
		end
	end

	class DeliveryStreamToleranceUnitOfMeasure < Quickfix::StringField
		def DeliveryStreamToleranceUnitOfMeasure.field
			return 41073
		end
		def initialize(data = nil)
			if( data == nil )
				super(41073)
			else
				super(41073, data)
			end
		end
	end

	class DeliveryStreamToleranceType < Quickfix::IntField
		def DeliveryStreamToleranceType.field
			return 41074
		end
		def initialize(data = nil)
			if( data == nil )
				super(41074)
			else
				super(41074, data)
			end
		end
	end

	class DeliveryStreamToleranceOptionSide < Quickfix::IntField
		def DeliveryStreamToleranceOptionSide.field
			return 41075
		end
		def initialize(data = nil)
			if( data == nil )
				super(41075)
			else
				super(41075, data)
			end
		end
	end

	class DeliveryStreamTotalPositiveTolerance < Quickfix::DoubleField
		def DeliveryStreamTotalPositiveTolerance.field
			return 41076
		end
		def initialize(data = nil)
			if( data == nil )
				super(41076)
			else
				super(41076, data)
			end
		end
	end

	class DeliveryStreamTotalNegativeTolerance < Quickfix::DoubleField
		def DeliveryStreamTotalNegativeTolerance.field
			return 41077
		end
		def initialize(data = nil)
			if( data == nil )
				super(41077)
			else
				super(41077, data)
			end
		end
	end

	class DeliveryStreamNotionalConversionFactor < Quickfix::DoubleField
		def DeliveryStreamNotionalConversionFactor.field
			return 41078
		end
		def initialize(data = nil)
			if( data == nil )
				super(41078)
			else
				super(41078, data)
			end
		end
	end

	class DeliveryStreamTransportEquipment < Quickfix::StringField
		def DeliveryStreamTransportEquipment.field
			return 41079
		end
		def initialize(data = nil)
			if( data == nil )
				super(41079)
			else
				super(41079, data)
			end
		end
	end

	class DeliveryStreamElectingPartySide < Quickfix::IntField
		def DeliveryStreamElectingPartySide.field
			return 41080
		end
		def initialize(data = nil)
			if( data == nil )
				super(41080)
			else
				super(41080, data)
			end
		end
	end

	class NoDeliveryStreamCycles < Quickfix::IntField
		def NoDeliveryStreamCycles.field
			return 41081
		end
		def initialize(data = nil)
			if( data == nil )
				super(41081)
			else
				super(41081, data)
			end
		end
	end

	class DeliveryStreamCycleDesc < Quickfix::StringField
		def DeliveryStreamCycleDesc.field
			return 41082
		end
		def initialize(data = nil)
			if( data == nil )
				super(41082)
			else
				super(41082, data)
			end
		end
	end

	class EncodedDeliveryStreamCycleDescLen < Quickfix::IntField
		def EncodedDeliveryStreamCycleDescLen.field
			return 41083
		end
		def initialize(data = nil)
			if( data == nil )
				super(41083)
			else
				super(41083, data)
			end
		end
	end

	class EncodedDeliveryStreamCycleDesc < Quickfix::StringField
		def EncodedDeliveryStreamCycleDesc.field
			return 41084
		end
		def initialize(data = nil)
			if( data == nil )
				super(41084)
			else
				super(41084, data)
			end
		end
	end

	class NoDeliveryStreamCommoditySources < Quickfix::IntField
		def NoDeliveryStreamCommoditySources.field
			return 41085
		end
		def initialize(data = nil)
			if( data == nil )
				super(41085)
			else
				super(41085, data)
			end
		end
	end

	class DeliveryStreamCommoditySource < Quickfix::StringField
		def DeliveryStreamCommoditySource.field
			return 41086
		end
		def initialize(data = nil)
			if( data == nil )
				super(41086)
			else
				super(41086, data)
			end
		end
	end

	class DocumentationText < Quickfix::StringField
		def DocumentationText.field
			return 1513
		end
		def initialize(data = nil)
			if( data == nil )
				super(1513)
			else
				super(1513, data)
			end
		end
	end

	class EncodedDocumentationTextLen < Quickfix::IntField
		def EncodedDocumentationTextLen.field
			return 1525
		end
		def initialize(data = nil)
			if( data == nil )
				super(1525)
			else
				super(1525, data)
			end
		end
	end

	class EncodedDocumentationText < Quickfix::StringField
		def EncodedDocumentationText.field
			return 1527
		end
		def initialize(data = nil)
			if( data == nil )
				super(1527)
			else
				super(1527, data)
			end
		end
	end

	class SwapSubClass < Quickfix::StringField
		def SwapSubClass.field
			return 1575
		end
		def initialize(data = nil)
			if( data == nil )
				super(1575)
			else
				super(1575, data)
			end
		end
	end

	class SettlRateIndex < Quickfix::StringField
		def SettlRateIndex.field
			return 1577
		end
		def initialize(data = nil)
			if( data == nil )
				super(1577)
			else
				super(1577, data)
			end
		end
	end

	class SettlRateIndexLocation < Quickfix::StringField
		def SettlRateIndexLocation.field
			return 1580
		end
		def initialize(data = nil)
			if( data == nil )
				super(1580)
			else
				super(1580, data)
			end
		end
	end

	class OptionExpirationDesc < Quickfix::StringField
		def OptionExpirationDesc.field
			return 1581
		end
		def initialize(data = nil)
			if( data == nil )
				super(1581)
			else
				super(1581, data)
			end
		end
	end

	class EncodedOptionExpirationDescLen < Quickfix::IntField
		def EncodedOptionExpirationDescLen.field
			return 1678
		end
		def initialize(data = nil)
			if( data == nil )
				super(1678)
			else
				super(1678, data)
			end
		end
	end

	class EncodedOptionExpirationDesc < Quickfix::StringField
		def EncodedOptionExpirationDesc.field
			return 1697
		end
		def initialize(data = nil)
			if( data == nil )
				super(1697)
			else
				super(1697, data)
			end
		end
	end

	class StrikeUnitOfMeasure < Quickfix::StringField
		def StrikeUnitOfMeasure.field
			return 1698
		end
		def initialize(data = nil)
			if( data == nil )
				super(1698)
			else
				super(1698, data)
			end
		end
	end

	class StrikeIndex < Quickfix::StringField
		def StrikeIndex.field
			return 1866
		end
		def initialize(data = nil)
			if( data == nil )
				super(1866)
			else
				super(1866, data)
			end
		end
	end

	class StrikeIndexSpread < Quickfix::DoubleField
		def StrikeIndexSpread.field
			return 2001
		end
		def initialize(data = nil)
			if( data == nil )
				super(2001)
			else
				super(2001, data)
			end
		end
	end

	class ValuationSource < Quickfix::StringField
		def ValuationSource.field
			return 2002
		end
		def initialize(data = nil)
			if( data == nil )
				super(2002)
			else
				super(2002, data)
			end
		end
	end

	class ValuationReferenceModel < Quickfix::StringField
		def ValuationReferenceModel.field
			return 2140
		end
		def initialize(data = nil)
			if( data == nil )
				super(2140)
			else
				super(2140, data)
			end
		end
	end

	class StrategyType < Quickfix::StringField
		def StrategyType.field
			return 2141
		end
		def initialize(data = nil)
			if( data == nil )
				super(2141)
			else
				super(2141, data)
			end
		end
	end

	class CommonPricingIndicator < Quickfix::BoolField
		def CommonPricingIndicator.field
			return 2142
		end
		def initialize(data = nil)
			if( data == nil )
				super(2142)
			else
				super(2142, data)
			end
		end
	end

	class SettlDisruptionProvision < Quickfix::IntField
		def SettlDisruptionProvision.field
			return 2143
		end
		def initialize(data = nil)
			if( data == nil )
				super(2143)
			else
				super(2143, data)
			end
		end
	end

	class InstrumentRoundingDirection < Quickfix::CharField
		def InstrumentRoundingDirection.field
			return 2144
		end
		def initialize(data = nil)
			if( data == nil )
				super(2144)
			else
				super(2144, data)
			end
		end
	end

	class InstrumentRoundingPrecision < Quickfix::IntField
		def InstrumentRoundingPrecision.field
			return 2145
		end
		def initialize(data = nil)
			if( data == nil )
				super(2145)
			else
				super(2145, data)
			end
		end
	end

	class LegSettleOnOpenFlag < Quickfix::StringField
		def LegSettleOnOpenFlag.field
			return 2146
		end
		def initialize(data = nil)
			if( data == nil )
				super(2146)
			else
				super(2146, data)
			end
		end
	end

	class LegInstrmtAssignmentMethod < Quickfix::CharField
		def LegInstrmtAssignmentMethod.field
			return 2147
		end
		def initialize(data = nil)
			if( data == nil )
				super(2147)
			else
				super(2147, data)
			end
		end
	end

	class LegSecurityStatus < Quickfix::StringField
		def LegSecurityStatus.field
			return 2148
		end
		def initialize(data = nil)
			if( data == nil )
				super(2148)
			else
				super(2148, data)
			end
		end
	end

	class LegRestructuringType < Quickfix::StringField
		def LegRestructuringType.field
			return 2149
		end
		def initialize(data = nil)
			if( data == nil )
				super(2149)
			else
				super(2149, data)
			end
		end
	end

	class LegSeniority < Quickfix::StringField
		def LegSeniority.field
			return 2150
		end
		def initialize(data = nil)
			if( data == nil )
				super(2150)
			else
				super(2150, data)
			end
		end
	end

	class LegNotionalPercentageOutstanding < Quickfix::DoubleField
		def LegNotionalPercentageOutstanding.field
			return 2151
		end
		def initialize(data = nil)
			if( data == nil )
				super(2151)
			else
				super(2151, data)
			end
		end
	end

	class LegOriginalNotionalPercentageOutstanding < Quickfix::DoubleField
		def LegOriginalNotionalPercentageOutstanding.field
			return 2152
		end
		def initialize(data = nil)
			if( data == nil )
				super(2152)
			else
				super(2152, data)
			end
		end
	end

	class LegAttachmentPoint < Quickfix::DoubleField
		def LegAttachmentPoint.field
			return 2153
		end
		def initialize(data = nil)
			if( data == nil )
				super(2153)
			else
				super(2153, data)
			end
		end
	end

	class LegDetachmentPoint < Quickfix::DoubleField
		def LegDetachmentPoint.field
			return 2154
		end
		def initialize(data = nil)
			if( data == nil )
				super(2154)
			else
				super(2154, data)
			end
		end
	end

	class LegObligationType < Quickfix::StringField
		def LegObligationType.field
			return 2155
		end
		def initialize(data = nil)
			if( data == nil )
				super(2155)
			else
				super(2155, data)
			end
		end
	end

	class LegSwapSubClass < Quickfix::StringField
		def LegSwapSubClass.field
			return 2156
		end
		def initialize(data = nil)
			if( data == nil )
				super(2156)
			else
				super(2156, data)
			end
		end
	end

	class LegNthToDefault < Quickfix::IntField
		def LegNthToDefault.field
			return 2157
		end
		def initialize(data = nil)
			if( data == nil )
				super(2157)
			else
				super(2157, data)
			end
		end
	end

	class LegMthToDefault < Quickfix::IntField
		def LegMthToDefault.field
			return 2158
		end
		def initialize(data = nil)
			if( data == nil )
				super(2158)
			else
				super(2158, data)
			end
		end
	end

	class LegSettledEntityMatrixSource < Quickfix::StringField
		def LegSettledEntityMatrixSource.field
			return 2159
		end
		def initialize(data = nil)
			if( data == nil )
				super(2159)
			else
				super(2159, data)
			end
		end
	end

	class LegSettledEntityMatrixPublicationDate < Quickfix::StringField
		def LegSettledEntityMatrixPublicationDate.field
			return 2160
		end
		def initialize(data = nil)
			if( data == nil )
				super(2160)
			else
				super(2160, data)
			end
		end
	end

	class LegCouponType < Quickfix::IntField
		def LegCouponType.field
			return 2161
		end
		def initialize(data = nil)
			if( data == nil )
				super(2161)
			else
				super(2161, data)
			end
		end
	end

	class LegTotalIssuedAmount < Quickfix::DoubleField
		def LegTotalIssuedAmount.field
			return 2162
		end
		def initialize(data = nil)
			if( data == nil )
				super(2162)
			else
				super(2162, data)
			end
		end
	end

	class LegCouponFrequencyPeriod < Quickfix::IntField
		def LegCouponFrequencyPeriod.field
			return 2163
		end
		def initialize(data = nil)
			if( data == nil )
				super(2163)
			else
				super(2163, data)
			end
		end
	end

	class LegCouponFrequencyUnit < Quickfix::StringField
		def LegCouponFrequencyUnit.field
			return 2164
		end
		def initialize(data = nil)
			if( data == nil )
				super(2164)
			else
				super(2164, data)
			end
		end
	end

	class LegCouponDayCount < Quickfix::IntField
		def LegCouponDayCount.field
			return 2165
		end
		def initialize(data = nil)
			if( data == nil )
				super(2165)
			else
				super(2165, data)
			end
		end
	end

	class LegConvertibleBondEquityID < Quickfix::StringField
		def LegConvertibleBondEquityID.field
			return 2166
		end
		def initialize(data = nil)
			if( data == nil )
				super(2166)
			else
				super(2166, data)
			end
		end
	end

	class LegConvertibleBondEquityIDSource < Quickfix::StringField
		def LegConvertibleBondEquityIDSource.field
			return 2167
		end
		def initialize(data = nil)
			if( data == nil )
				super(2167)
			else
				super(2167, data)
			end
		end
	end

	class LegContractPriceRefMonth < Quickfix::StringField
		def LegContractPriceRefMonth.field
			return 2168
		end
		def initialize(data = nil)
			if( data == nil )
				super(2168)
			else
				super(2168, data)
			end
		end
	end

	class LegLienSeniority < Quickfix::IntField
		def LegLienSeniority.field
			return 2169
		end
		def initialize(data = nil)
			if( data == nil )
				super(2169)
			else
				super(2169, data)
			end
		end
	end

	class LegLoanFacility < Quickfix::IntField
		def LegLoanFacility.field
			return 2170
		end
		def initialize(data = nil)
			if( data == nil )
				super(2170)
			else
				super(2170, data)
			end
		end
	end

	class LegReferenceEntityType < Quickfix::IntField
		def LegReferenceEntityType.field
			return 2171
		end
		def initialize(data = nil)
			if( data == nil )
				super(2171)
			else
				super(2171, data)
			end
		end
	end

	class LegIndexSeries < Quickfix::IntField
		def LegIndexSeries.field
			return 2172
		end
		def initialize(data = nil)
			if( data == nil )
				super(2172)
			else
				super(2172, data)
			end
		end
	end

	class LegIndexAnnexVersion < Quickfix::IntField
		def LegIndexAnnexVersion.field
			return 2173
		end
		def initialize(data = nil)
			if( data == nil )
				super(2173)
			else
				super(2173, data)
			end
		end
	end

	class LegIndexAnnexDate < Quickfix::StringField
		def LegIndexAnnexDate.field
			return 2174
		end
		def initialize(data = nil)
			if( data == nil )
				super(2174)
			else
				super(2174, data)
			end
		end
	end

	class LegIndexAnnexSource < Quickfix::StringField
		def LegIndexAnnexSource.field
			return 2175
		end
		def initialize(data = nil)
			if( data == nil )
				super(2175)
			else
				super(2175, data)
			end
		end
	end

	class LegSettlRateIndex < Quickfix::StringField
		def LegSettlRateIndex.field
			return 2176
		end
		def initialize(data = nil)
			if( data == nil )
				super(2176)
			else
				super(2176, data)
			end
		end
	end

	class LegSettlRateIndexLocation < Quickfix::StringField
		def LegSettlRateIndexLocation.field
			return 2177
		end
		def initialize(data = nil)
			if( data == nil )
				super(2177)
			else
				super(2177, data)
			end
		end
	end

	class LegOptionExpirationDesc < Quickfix::StringField
		def LegOptionExpirationDesc.field
			return 2178
		end
		def initialize(data = nil)
			if( data == nil )
				super(2178)
			else
				super(2178, data)
			end
		end
	end

	class EncodedLegOptionExpirationDescLen < Quickfix::IntField
		def EncodedLegOptionExpirationDescLen.field
			return 2179
		end
		def initialize(data = nil)
			if( data == nil )
				super(2179)
			else
				super(2179, data)
			end
		end
	end

	class EncodedLegOptionExpirationDesc < Quickfix::StringField
		def EncodedLegOptionExpirationDesc.field
			return 2180
		end
		def initialize(data = nil)
			if( data == nil )
				super(2180)
			else
				super(2180, data)
			end
		end
	end

	class LegStrikeMultiplier < Quickfix::DoubleField
		def LegStrikeMultiplier.field
			return 2181
		end
		def initialize(data = nil)
			if( data == nil )
				super(2181)
			else
				super(2181, data)
			end
		end
	end

	class LegStrikeValue < Quickfix::DoubleField
		def LegStrikeValue.field
			return 2182
		end
		def initialize(data = nil)
			if( data == nil )
				super(2182)
			else
				super(2182, data)
			end
		end
	end

	class LegStrikeUnitOfMeasure < Quickfix::StringField
		def LegStrikeUnitOfMeasure.field
			return 2183
		end
		def initialize(data = nil)
			if( data == nil )
				super(2183)
			else
				super(2183, data)
			end
		end
	end

	class LegStrikeIndex < Quickfix::StringField
		def LegStrikeIndex.field
			return 2184
		end
		def initialize(data = nil)
			if( data == nil )
				super(2184)
			else
				super(2184, data)
			end
		end
	end

	class LegStrikeIndexSpread < Quickfix::DoubleField
		def LegStrikeIndexSpread.field
			return 2185
		end
		def initialize(data = nil)
			if( data == nil )
				super(2185)
			else
				super(2185, data)
			end
		end
	end

	class LegStrikePriceDeterminationMethod < Quickfix::IntField
		def LegStrikePriceDeterminationMethod.field
			return 2186
		end
		def initialize(data = nil)
			if( data == nil )
				super(2186)
			else
				super(2186, data)
			end
		end
	end

	class LegStrikePriceBoundaryMethod < Quickfix::IntField
		def LegStrikePriceBoundaryMethod.field
			return 2187
		end
		def initialize(data = nil)
			if( data == nil )
				super(2187)
			else
				super(2187, data)
			end
		end
	end

	class LegStrikePriceBoundaryPrecision < Quickfix::DoubleField
		def LegStrikePriceBoundaryPrecision.field
			return 2188
		end
		def initialize(data = nil)
			if( data == nil )
				super(2188)
			else
				super(2188, data)
			end
		end
	end

	class LegUnderlyingPriceDeterminationMethod < Quickfix::IntField
		def LegUnderlyingPriceDeterminationMethod.field
			return 2189
		end
		def initialize(data = nil)
			if( data == nil )
				super(2189)
			else
				super(2189, data)
			end
		end
	end

	class LegMinPriceIncrement < Quickfix::DoubleField
		def LegMinPriceIncrement.field
			return 2190
		end
		def initialize(data = nil)
			if( data == nil )
				super(2190)
			else
				super(2190, data)
			end
		end
	end

	class LegMinPriceIncrementAmount < Quickfix::DoubleField
		def LegMinPriceIncrementAmount.field
			return 2191
		end
		def initialize(data = nil)
			if( data == nil )
				super(2191)
			else
				super(2191, data)
			end
		end
	end

	class LegSettlMethod < Quickfix::StringField
		def LegSettlMethod.field
			return 2192
		end
		def initialize(data = nil)
			if( data == nil )
				super(2192)
			else
				super(2192, data)
			end
		end
	end

	class LegOptPayoutType < Quickfix::IntField
		def LegOptPayoutType.field
			return 2193
		end
		def initialize(data = nil)
			if( data == nil )
				super(2193)
			else
				super(2193, data)
			end
		end
	end

	class LegOptPayoutAmount < Quickfix::DoubleField
		def LegOptPayoutAmount.field
			return 2194
		end
		def initialize(data = nil)
			if( data == nil )
				super(2194)
			else
				super(2194, data)
			end
		end
	end

	class LegPriceQuoteMethod < Quickfix::StringField
		def LegPriceQuoteMethod.field
			return 2195
		end
		def initialize(data = nil)
			if( data == nil )
				super(2195)
			else
				super(2195, data)
			end
		end
	end

	class LegValuationMethod < Quickfix::StringField
		def LegValuationMethod.field
			return 2196
		end
		def initialize(data = nil)
			if( data == nil )
				super(2196)
			else
				super(2196, data)
			end
		end
	end

	class LegValuationSource < Quickfix::StringField
		def LegValuationSource.field
			return 2197
		end
		def initialize(data = nil)
			if( data == nil )
				super(2197)
			else
				super(2197, data)
			end
		end
	end

	class LegValuationReferenceModel < Quickfix::StringField
		def LegValuationReferenceModel.field
			return 2198
		end
		def initialize(data = nil)
			if( data == nil )
				super(2198)
			else
				super(2198, data)
			end
		end
	end

	class LegListMethod < Quickfix::IntField
		def LegListMethod.field
			return 2199
		end
		def initialize(data = nil)
			if( data == nil )
				super(2199)
			else
				super(2199, data)
			end
		end
	end

	class LegCapPrice < Quickfix::DoubleField
		def LegCapPrice.field
			return 2200
		end
		def initialize(data = nil)
			if( data == nil )
				super(2200)
			else
				super(2200, data)
			end
		end
	end

	class LegFloorPrice < Quickfix::DoubleField
		def LegFloorPrice.field
			return 2201
		end
		def initialize(data = nil)
			if( data == nil )
				super(2201)
			else
				super(2201, data)
			end
		end
	end

	class LegFlexibleIndicator < Quickfix::BoolField
		def LegFlexibleIndicator.field
			return 2202
		end
		def initialize(data = nil)
			if( data == nil )
				super(2202)
			else
				super(2202, data)
			end
		end
	end

	class LegFlexProductEligibilityIndicator < Quickfix::BoolField
		def LegFlexProductEligibilityIndicator.field
			return 2203
		end
		def initialize(data = nil)
			if( data == nil )
				super(2203)
			else
				super(2203, data)
			end
		end
	end

	class LegPositionLimit < Quickfix::IntField
		def LegPositionLimit.field
			return 2205
		end
		def initialize(data = nil)
			if( data == nil )
				super(2205)
			else
				super(2205, data)
			end
		end
	end

	class LegNTPositionLimit < Quickfix::IntField
		def LegNTPositionLimit.field
			return 2206
		end
		def initialize(data = nil)
			if( data == nil )
				super(2206)
			else
				super(2206, data)
			end
		end
	end

	class LegCPProgram < Quickfix::IntField
		def LegCPProgram.field
			return 2207
		end
		def initialize(data = nil)
			if( data == nil )
				super(2207)
			else
				super(2207, data)
			end
		end
	end

	class LegCPRegType < Quickfix::StringField
		def LegCPRegType.field
			return 2208
		end
		def initialize(data = nil)
			if( data == nil )
				super(2208)
			else
				super(2208, data)
			end
		end
	end

	class LegShortSaleRestriction < Quickfix::IntField
		def LegShortSaleRestriction.field
			return 2209
		end
		def initialize(data = nil)
			if( data == nil )
				super(2209)
			else
				super(2209, data)
			end
		end
	end

	class LegStrategyType < Quickfix::StringField
		def LegStrategyType.field
			return 2211
		end
		def initialize(data = nil)
			if( data == nil )
				super(2211)
			else
				super(2211, data)
			end
		end
	end

	class LegCommonPricingIndicator < Quickfix::BoolField
		def LegCommonPricingIndicator.field
			return 2212
		end
		def initialize(data = nil)
			if( data == nil )
				super(2212)
			else
				super(2212, data)
			end
		end
	end

	class LegSettlDisruptionProvision < Quickfix::IntField
		def LegSettlDisruptionProvision.field
			return 2213
		end
		def initialize(data = nil)
			if( data == nil )
				super(2213)
			else
				super(2213, data)
			end
		end
	end

	class LegInstrumentRoundingDirection < Quickfix::CharField
		def LegInstrumentRoundingDirection.field
			return 2214
		end
		def initialize(data = nil)
			if( data == nil )
				super(2214)
			else
				super(2214, data)
			end
		end
	end

	class LegInstrumentRoundingPrecision < Quickfix::IntField
		def LegInstrumentRoundingPrecision.field
			return 2215
		end
		def initialize(data = nil)
			if( data == nil )
				super(2215)
			else
				super(2215, data)
			end
		end
	end

	class MarketDisruptionProvision < Quickfix::IntField
		def MarketDisruptionProvision.field
			return 41087
		end
		def initialize(data = nil)
			if( data == nil )
				super(41087)
			else
				super(41087, data)
			end
		end
	end

	class MarketDisruptionFallbackProvision < Quickfix::IntField
		def MarketDisruptionFallbackProvision.field
			return 41088
		end
		def initialize(data = nil)
			if( data == nil )
				super(41088)
			else
				super(41088, data)
			end
		end
	end

	class MarketDisruptionMaximumDays < Quickfix::IntField
		def MarketDisruptionMaximumDays.field
			return 41089
		end
		def initialize(data = nil)
			if( data == nil )
				super(41089)
			else
				super(41089, data)
			end
		end
	end

	class MarketDisruptionMaterialityPercentage < Quickfix::DoubleField
		def MarketDisruptionMaterialityPercentage.field
			return 41090
		end
		def initialize(data = nil)
			if( data == nil )
				super(41090)
			else
				super(41090, data)
			end
		end
	end

	class MarketDisruptionMinimumFuturesContracts < Quickfix::IntField
		def MarketDisruptionMinimumFuturesContracts.field
			return 41091
		end
		def initialize(data = nil)
			if( data == nil )
				super(41091)
			else
				super(41091, data)
			end
		end
	end

	class NoMarketDisruptionEvents < Quickfix::IntField
		def NoMarketDisruptionEvents.field
			return 41092
		end
		def initialize(data = nil)
			if( data == nil )
				super(41092)
			else
				super(41092, data)
			end
		end
	end

	class MarketDisruptionEvent < Quickfix::StringField
		def MarketDisruptionEvent.field
			return 41093
		end
		def initialize(data = nil)
			if( data == nil )
				super(41093)
			else
				super(41093, data)
			end
		end
	end

	class NoMarketDisruptionFallbacks < Quickfix::IntField
		def NoMarketDisruptionFallbacks.field
			return 41094
		end
		def initialize(data = nil)
			if( data == nil )
				super(41094)
			else
				super(41094, data)
			end
		end
	end

	class MarketDisruptionFallbackType < Quickfix::StringField
		def MarketDisruptionFallbackType.field
			return 41095
		end
		def initialize(data = nil)
			if( data == nil )
				super(41095)
			else
				super(41095, data)
			end
		end
	end

	class NoMarketDisruptionFallbackReferencePrices < Quickfix::IntField
		def NoMarketDisruptionFallbackReferencePrices.field
			return 41096
		end
		def initialize(data = nil)
			if( data == nil )
				super(41096)
			else
				super(41096, data)
			end
		end
	end

	class MarketDisruptionFallbackUnderlierType < Quickfix::IntField
		def MarketDisruptionFallbackUnderlierType.field
			return 41097
		end
		def initialize(data = nil)
			if( data == nil )
				super(41097)
			else
				super(41097, data)
			end
		end
	end

	class MarketDisruptionFallbackUnderlierSecurityID < Quickfix::StringField
		def MarketDisruptionFallbackUnderlierSecurityID.field
			return 41098
		end
		def initialize(data = nil)
			if( data == nil )
				super(41098)
			else
				super(41098, data)
			end
		end
	end

	class MarketDisruptionFallbackUnderlierSecurityIDSource < Quickfix::StringField
		def MarketDisruptionFallbackUnderlierSecurityIDSource.field
			return 41099
		end
		def initialize(data = nil)
			if( data == nil )
				super(41099)
			else
				super(41099, data)
			end
		end
	end

	class MarketDisruptionFallbackUnderlierSecurityDesc < Quickfix::StringField
		def MarketDisruptionFallbackUnderlierSecurityDesc.field
			return 41100
		end
		def initialize(data = nil)
			if( data == nil )
				super(41100)
			else
				super(41100, data)
			end
		end
	end

	class EncodedMarketDisruptionFallbackUnderlierSecurityDescLen < Quickfix::IntField
		def EncodedMarketDisruptionFallbackUnderlierSecurityDescLen.field
			return 41101
		end
		def initialize(data = nil)
			if( data == nil )
				super(41101)
			else
				super(41101, data)
			end
		end
	end

	class EncodedMarketDisruptionFallbackUnderlierSecurityDesc < Quickfix::StringField
		def EncodedMarketDisruptionFallbackUnderlierSecurityDesc.field
			return 41102
		end
		def initialize(data = nil)
			if( data == nil )
				super(41102)
			else
				super(41102, data)
			end
		end
	end

	class MarketDisruptionFallbackOpenUnits < Quickfix::DoubleField
		def MarketDisruptionFallbackOpenUnits.field
			return 41103
		end
		def initialize(data = nil)
			if( data == nil )
				super(41103)
			else
				super(41103, data)
			end
		end
	end

	class MarketDisruptionFallbackBasketCurrency < Quickfix::StringField
		def MarketDisruptionFallbackBasketCurrency.field
			return 41104
		end
		def initialize(data = nil)
			if( data == nil )
				super(41104)
			else
				super(41104, data)
			end
		end
	end

	class MarketDisruptionFallbackBasketDivisor < Quickfix::DoubleField
		def MarketDisruptionFallbackBasketDivisor.field
			return 41105
		end
		def initialize(data = nil)
			if( data == nil )
				super(41105)
			else
				super(41105, data)
			end
		end
	end

	class MiscFeeRate < Quickfix::DoubleField
		def MiscFeeRate.field
			return 2216
		end
		def initialize(data = nil)
			if( data == nil )
				super(2216)
			else
				super(2216, data)
			end
		end
	end

	class MiscFeeAmountDue < Quickfix::DoubleField
		def MiscFeeAmountDue.field
			return 2217
		end
		def initialize(data = nil)
			if( data == nil )
				super(2217)
			else
				super(2217, data)
			end
		end
	end

	class ExerciseDesc < Quickfix::StringField
		def ExerciseDesc.field
			return 41106
		end
		def initialize(data = nil)
			if( data == nil )
				super(41106)
			else
				super(41106, data)
			end
		end
	end

	class EncodedExerciseDescLen < Quickfix::IntField
		def EncodedExerciseDescLen.field
			return 41107
		end
		def initialize(data = nil)
			if( data == nil )
				super(41107)
			else
				super(41107, data)
			end
		end
	end

	class EncodedExerciseDesc < Quickfix::StringField
		def EncodedExerciseDesc.field
			return 41108
		end
		def initialize(data = nil)
			if( data == nil )
				super(41108)
			else
				super(41108, data)
			end
		end
	end

	class AutomaticExerciseIndicator < Quickfix::BoolField
		def AutomaticExerciseIndicator.field
			return 41109
		end
		def initialize(data = nil)
			if( data == nil )
				super(41109)
			else
				super(41109, data)
			end
		end
	end

	class AutomaticExerciseThresholdRate < Quickfix::DoubleField
		def AutomaticExerciseThresholdRate.field
			return 41110
		end
		def initialize(data = nil)
			if( data == nil )
				super(41110)
			else
				super(41110, data)
			end
		end
	end

	class ExerciseConfirmationMethod < Quickfix::IntField
		def ExerciseConfirmationMethod.field
			return 41111
		end
		def initialize(data = nil)
			if( data == nil )
				super(41111)
			else
				super(41111, data)
			end
		end
	end

	class ManualNoticeBusinessCenter < Quickfix::StringField
		def ManualNoticeBusinessCenter.field
			return 41112
		end
		def initialize(data = nil)
			if( data == nil )
				super(41112)
			else
				super(41112, data)
			end
		end
	end

	class FallbackExerciseIndicator < Quickfix::BoolField
		def FallbackExerciseIndicator.field
			return 41113
		end
		def initialize(data = nil)
			if( data == nil )
				super(41113)
			else
				super(41113, data)
			end
		end
	end

	class LimitedRightToConfirmIndicator < Quickfix::BoolField
		def LimitedRightToConfirmIndicator.field
			return 41114
		end
		def initialize(data = nil)
			if( data == nil )
				super(41114)
			else
				super(41114, data)
			end
		end
	end

	class ExerciseSplitTicketIndicator < Quickfix::BoolField
		def ExerciseSplitTicketIndicator.field
			return 41115
		end
		def initialize(data = nil)
			if( data == nil )
				super(41115)
			else
				super(41115, data)
			end
		end
	end

	class NoOptionExerciseBusinessCenters < Quickfix::IntField
		def NoOptionExerciseBusinessCenters.field
			return 41116
		end
		def initialize(data = nil)
			if( data == nil )
				super(41116)
			else
				super(41116, data)
			end
		end
	end

	class OptionExerciseBusinessCenter < Quickfix::StringField
		def OptionExerciseBusinessCenter.field
			return 41117
		end
		def initialize(data = nil)
			if( data == nil )
				super(41117)
			else
				super(41117, data)
			end
		end
	end

	class OptionExerciseBusinessDayConvention < Quickfix::IntField
		def OptionExerciseBusinessDayConvention.field
			return 41118
		end
		def initialize(data = nil)
			if( data == nil )
				super(41118)
			else
				super(41118, data)
			end
		end
	end

	class OptionExerciseEarliestDateOffsetDayType < Quickfix::IntField
		def OptionExerciseEarliestDateOffsetDayType.field
			return 41119
		end
		def initialize(data = nil)
			if( data == nil )
				super(41119)
			else
				super(41119, data)
			end
		end
	end

	class OptionExerciseEarliestDateOffsetPeriod < Quickfix::IntField
		def OptionExerciseEarliestDateOffsetPeriod.field
			return 41120
		end
		def initialize(data = nil)
			if( data == nil )
				super(41120)
			else
				super(41120, data)
			end
		end
	end

	class OptionExerciseEarliestDateOffsetUnit < Quickfix::StringField
		def OptionExerciseEarliestDateOffsetUnit.field
			return 41121
		end
		def initialize(data = nil)
			if( data == nil )
				super(41121)
			else
				super(41121, data)
			end
		end
	end

	class OptionExerciseFrequencyPeriod < Quickfix::IntField
		def OptionExerciseFrequencyPeriod.field
			return 41122
		end
		def initialize(data = nil)
			if( data == nil )
				super(41122)
			else
				super(41122, data)
			end
		end
	end

	class OptionExerciseFrequencyUnit < Quickfix::StringField
		def OptionExerciseFrequencyUnit.field
			return 41123
		end
		def initialize(data = nil)
			if( data == nil )
				super(41123)
			else
				super(41123, data)
			end
		end
	end

	class OptionExerciseStartDateUnadjusted < Quickfix::StringField
		def OptionExerciseStartDateUnadjusted.field
			return 41124
		end
		def initialize(data = nil)
			if( data == nil )
				super(41124)
			else
				super(41124, data)
			end
		end
	end

	class OptionExerciseStartDateRelativeTo < Quickfix::IntField
		def OptionExerciseStartDateRelativeTo.field
			return 41125
		end
		def initialize(data = nil)
			if( data == nil )
				super(41125)
			else
				super(41125, data)
			end
		end
	end

	class OptionExerciseStartDateOffsetPeriod < Quickfix::IntField
		def OptionExerciseStartDateOffsetPeriod.field
			return 41126
		end
		def initialize(data = nil)
			if( data == nil )
				super(41126)
			else
				super(41126, data)
			end
		end
	end

	class OptionExerciseStartDateOffsetUnit < Quickfix::StringField
		def OptionExerciseStartDateOffsetUnit.field
			return 41127
		end
		def initialize(data = nil)
			if( data == nil )
				super(41127)
			else
				super(41127, data)
			end
		end
	end

	class OptionExerciseStartDateOffsetDayType < Quickfix::IntField
		def OptionExerciseStartDateOffsetDayType.field
			return 41128
		end
		def initialize(data = nil)
			if( data == nil )
				super(41128)
			else
				super(41128, data)
			end
		end
	end

	class OptionExerciseStartDateAdjusted < Quickfix::StringField
		def OptionExerciseStartDateAdjusted.field
			return 41129
		end
		def initialize(data = nil)
			if( data == nil )
				super(41129)
			else
				super(41129, data)
			end
		end
	end

	class OptionExerciseSkip < Quickfix::IntField
		def OptionExerciseSkip.field
			return 41130
		end
		def initialize(data = nil)
			if( data == nil )
				super(41130)
			else
				super(41130, data)
			end
		end
	end

	class OptionExerciseNominationDeadline < Quickfix::StringField
		def OptionExerciseNominationDeadline.field
			return 41131
		end
		def initialize(data = nil)
			if( data == nil )
				super(41131)
			else
				super(41131, data)
			end
		end
	end

	class OptionExerciseFirstDateUnadjusted < Quickfix::StringField
		def OptionExerciseFirstDateUnadjusted.field
			return 41132
		end
		def initialize(data = nil)
			if( data == nil )
				super(41132)
			else
				super(41132, data)
			end
		end
	end

	class OptionExerciseLastDateUnadjusted < Quickfix::StringField
		def OptionExerciseLastDateUnadjusted.field
			return 41133
		end
		def initialize(data = nil)
			if( data == nil )
				super(41133)
			else
				super(41133, data)
			end
		end
	end

	class OptionExerciseEarliestTime < Quickfix::StringField
		def OptionExerciseEarliestTime.field
			return 41134
		end
		def initialize(data = nil)
			if( data == nil )
				super(41134)
			else
				super(41134, data)
			end
		end
	end

	class OptionExerciseLatestTime < Quickfix::StringField
		def OptionExerciseLatestTime.field
			return 41135
		end
		def initialize(data = nil)
			if( data == nil )
				super(41135)
			else
				super(41135, data)
			end
		end
	end

	class OptionExerciseTimeBusinessCenter < Quickfix::StringField
		def OptionExerciseTimeBusinessCenter.field
			return 41136
		end
		def initialize(data = nil)
			if( data == nil )
				super(41136)
			else
				super(41136, data)
			end
		end
	end

	class NoOptionExerciseDates < Quickfix::IntField
		def NoOptionExerciseDates.field
			return 41137
		end
		def initialize(data = nil)
			if( data == nil )
				super(41137)
			else
				super(41137, data)
			end
		end
	end

	class OptionExerciseDate < Quickfix::StringField
		def OptionExerciseDate.field
			return 41138
		end
		def initialize(data = nil)
			if( data == nil )
				super(41138)
			else
				super(41138, data)
			end
		end
	end

	class OptionExerciseDateType < Quickfix::IntField
		def OptionExerciseDateType.field
			return 41139
		end
		def initialize(data = nil)
			if( data == nil )
				super(41139)
			else
				super(41139, data)
			end
		end
	end

	class NoOptionExerciseExpirationDateBusinessCenters < Quickfix::IntField
		def NoOptionExerciseExpirationDateBusinessCenters.field
			return 41140
		end
		def initialize(data = nil)
			if( data == nil )
				super(41140)
			else
				super(41140, data)
			end
		end
	end

	class OptionExerciseExpirationDateBusinessCenter < Quickfix::StringField
		def OptionExerciseExpirationDateBusinessCenter.field
			return 41141
		end
		def initialize(data = nil)
			if( data == nil )
				super(41141)
			else
				super(41141, data)
			end
		end
	end

	class OptionExerciseExpirationDateBusinessDayConvention < Quickfix::IntField
		def OptionExerciseExpirationDateBusinessDayConvention.field
			return 41142
		end
		def initialize(data = nil)
			if( data == nil )
				super(41142)
			else
				super(41142, data)
			end
		end
	end

	class OptionExerciseExpirationDateRelativeTo < Quickfix::IntField
		def OptionExerciseExpirationDateRelativeTo.field
			return 41143
		end
		def initialize(data = nil)
			if( data == nil )
				super(41143)
			else
				super(41143, data)
			end
		end
	end

	class OptionExerciseExpirationDateOffsetPeriod < Quickfix::IntField
		def OptionExerciseExpirationDateOffsetPeriod.field
			return 41144
		end
		def initialize(data = nil)
			if( data == nil )
				super(41144)
			else
				super(41144, data)
			end
		end
	end

	class OptionExerciseExpirationDateOffsetUnit < Quickfix::StringField
		def OptionExerciseExpirationDateOffsetUnit.field
			return 41145
		end
		def initialize(data = nil)
			if( data == nil )
				super(41145)
			else
				super(41145, data)
			end
		end
	end

	class OptionExerciseExpirationFrequencyPeriod < Quickfix::IntField
		def OptionExerciseExpirationFrequencyPeriod.field
			return 41146
		end
		def initialize(data = nil)
			if( data == nil )
				super(41146)
			else
				super(41146, data)
			end
		end
	end

	class OptionExerciseExpirationFrequencyUnit < Quickfix::StringField
		def OptionExerciseExpirationFrequencyUnit.field
			return 41147
		end
		def initialize(data = nil)
			if( data == nil )
				super(41147)
			else
				super(41147, data)
			end
		end
	end

	class OptionExerciseExpirationRollConvention < Quickfix::StringField
		def OptionExerciseExpirationRollConvention.field
			return 41148
		end
		def initialize(data = nil)
			if( data == nil )
				super(41148)
			else
				super(41148, data)
			end
		end
	end

	class OptionExerciseExpirationDateOffsetDayType < Quickfix::IntField
		def OptionExerciseExpirationDateOffsetDayType.field
			return 41149
		end
		def initialize(data = nil)
			if( data == nil )
				super(41149)
			else
				super(41149, data)
			end
		end
	end

	class OptionExerciseExpirationTime < Quickfix::StringField
		def OptionExerciseExpirationTime.field
			return 41150
		end
		def initialize(data = nil)
			if( data == nil )
				super(41150)
			else
				super(41150, data)
			end
		end
	end

	class OptionExerciseExpirationTimeBusinessCenter < Quickfix::StringField
		def OptionExerciseExpirationTimeBusinessCenter.field
			return 41151
		end
		def initialize(data = nil)
			if( data == nil )
				super(41151)
			else
				super(41151, data)
			end
		end
	end

	class NoOptionExerciseExpirationDates < Quickfix::IntField
		def NoOptionExerciseExpirationDates.field
			return 41152
		end
		def initialize(data = nil)
			if( data == nil )
				super(41152)
			else
				super(41152, data)
			end
		end
	end

	class OptionExerciseExpirationDate < Quickfix::StringField
		def OptionExerciseExpirationDate.field
			return 41153
		end
		def initialize(data = nil)
			if( data == nil )
				super(41153)
			else
				super(41153, data)
			end
		end
	end

	class OptionExerciseExpirationDateType < Quickfix::IntField
		def OptionExerciseExpirationDateType.field
			return 41154
		end
		def initialize(data = nil)
			if( data == nil )
				super(41154)
			else
				super(41154, data)
			end
		end
	end

	class PaymentUnitOfMeasure < Quickfix::StringField
		def PaymentUnitOfMeasure.field
			return 41155
		end
		def initialize(data = nil)
			if( data == nil )
				super(41155)
			else
				super(41155, data)
			end
		end
	end

	class PaymentDateRelativeTo < Quickfix::IntField
		def PaymentDateRelativeTo.field
			return 41156
		end
		def initialize(data = nil)
			if( data == nil )
				super(41156)
			else
				super(41156, data)
			end
		end
	end

	class PaymentDateOffsetPeriod < Quickfix::IntField
		def PaymentDateOffsetPeriod.field
			return 41157
		end
		def initialize(data = nil)
			if( data == nil )
				super(41157)
			else
				super(41157, data)
			end
		end
	end

	class PaymentDateOffsetUnit < Quickfix::StringField
		def PaymentDateOffsetUnit.field
			return 41158
		end
		def initialize(data = nil)
			if( data == nil )
				super(41158)
			else
				super(41158, data)
			end
		end
	end

	class PaymentDateOffsetDayType < Quickfix::IntField
		def PaymentDateOffsetDayType.field
			return 41159
		end
		def initialize(data = nil)
			if( data == nil )
				super(41159)
			else
				super(41159, data)
			end
		end
	end

	class PaymentForwardStartType < Quickfix::IntField
		def PaymentForwardStartType.field
			return 41160
		end
		def initialize(data = nil)
			if( data == nil )
				super(41160)
			else
				super(41160, data)
			end
		end
	end

	class NoPaymentScheduleFixingDays < Quickfix::IntField
		def NoPaymentScheduleFixingDays.field
			return 41161
		end
		def initialize(data = nil)
			if( data == nil )
				super(41161)
			else
				super(41161, data)
			end
		end
	end

	class PaymentScheduleFixingDayOfWeek < Quickfix::IntField
		def PaymentScheduleFixingDayOfWeek.field
			return 41162
		end
		def initialize(data = nil)
			if( data == nil )
				super(41162)
			else
				super(41162, data)
			end
		end
	end

	class PaymentScheduleFixingDayNumber < Quickfix::IntField
		def PaymentScheduleFixingDayNumber.field
			return 41163
		end
		def initialize(data = nil)
			if( data == nil )
				super(41163)
			else
				super(41163, data)
			end
		end
	end

	class PaymentScheduleXID < Quickfix::StringField
		def PaymentScheduleXID.field
			return 41164
		end
		def initialize(data = nil)
			if( data == nil )
				super(41164)
			else
				super(41164, data)
			end
		end
	end

	class PaymentScheduleXIDRef < Quickfix::StringField
		def PaymentScheduleXIDRef.field
			return 41165
		end
		def initialize(data = nil)
			if( data == nil )
				super(41165)
			else
				super(41165, data)
			end
		end
	end

	class PaymentScheduleRateCurrency < Quickfix::StringField
		def PaymentScheduleRateCurrency.field
			return 41166
		end
		def initialize(data = nil)
			if( data == nil )
				super(41166)
			else
				super(41166, data)
			end
		end
	end

	class PaymentScheduleRateUnitOfMeasure < Quickfix::StringField
		def PaymentScheduleRateUnitOfMeasure.field
			return 41167
		end
		def initialize(data = nil)
			if( data == nil )
				super(41167)
			else
				super(41167, data)
			end
		end
	end

	class PaymentScheduleRateConversionFactor < Quickfix::DoubleField
		def PaymentScheduleRateConversionFactor.field
			return 41168
		end
		def initialize(data = nil)
			if( data == nil )
				super(41168)
			else
				super(41168, data)
			end
		end
	end

	class PaymentScheduleRateSpreadType < Quickfix::IntField
		def PaymentScheduleRateSpreadType.field
			return 41169
		end
		def initialize(data = nil)
			if( data == nil )
				super(41169)
			else
				super(41169, data)
			end
		end
	end

	class PaymentScheduleSettlPeriodPrice < Quickfix::DoubleField
		def PaymentScheduleSettlPeriodPrice.field
			return 41170
		end
		def initialize(data = nil)
			if( data == nil )
				super(41170)
			else
				super(41170, data)
			end
		end
	end

	class PaymentScheduleSettlPeriodPriceCurrency < Quickfix::StringField
		def PaymentScheduleSettlPeriodPriceCurrency.field
			return 41171
		end
		def initialize(data = nil)
			if( data == nil )
				super(41171)
			else
				super(41171, data)
			end
		end
	end

	class PaymentScheduleSettlPeriodPriceUnitOfMeasure < Quickfix::StringField
		def PaymentScheduleSettlPeriodPriceUnitOfMeasure.field
			return 41172
		end
		def initialize(data = nil)
			if( data == nil )
				super(41172)
			else
				super(41172, data)
			end
		end
	end

	class PaymentScheduleStepUnitOfMeasure < Quickfix::StringField
		def PaymentScheduleStepUnitOfMeasure.field
			return 41173
		end
		def initialize(data = nil)
			if( data == nil )
				super(41173)
			else
				super(41173, data)
			end
		end
	end

	class PaymentScheduleFixingDayDistribution < Quickfix::IntField
		def PaymentScheduleFixingDayDistribution.field
			return 41174
		end
		def initialize(data = nil)
			if( data == nil )
				super(41174)
			else
				super(41174, data)
			end
		end
	end

	class PaymentScheduleFixingDayCount < Quickfix::IntField
		def PaymentScheduleFixingDayCount.field
			return 41175
		end
		def initialize(data = nil)
			if( data == nil )
				super(41175)
			else
				super(41175, data)
			end
		end
	end

	class PaymentScheduleFixingLagPeriod < Quickfix::IntField
		def PaymentScheduleFixingLagPeriod.field
			return 41176
		end
		def initialize(data = nil)
			if( data == nil )
				super(41176)
			else
				super(41176, data)
			end
		end
	end

	class PaymentScheduleFixingLagUnit < Quickfix::StringField
		def PaymentScheduleFixingLagUnit.field
			return 41177
		end
		def initialize(data = nil)
			if( data == nil )
				super(41177)
			else
				super(41177, data)
			end
		end
	end

	class PaymentScheduleFixingFirstObservationDateOffsetPeriod < Quickfix::IntField
		def PaymentScheduleFixingFirstObservationDateOffsetPeriod.field
			return 41178
		end
		def initialize(data = nil)
			if( data == nil )
				super(41178)
			else
				super(41178, data)
			end
		end
	end

	class PaymentScheduleFixingFirstObservationDateOffsetUnit < Quickfix::StringField
		def PaymentScheduleFixingFirstObservationDateOffsetUnit.field
			return 41179
		end
		def initialize(data = nil)
			if( data == nil )
				super(41179)
			else
				super(41179, data)
			end
		end
	end

	class PaymentStreamFlatRateIndicator < Quickfix::BoolField
		def PaymentStreamFlatRateIndicator.field
			return 41180
		end
		def initialize(data = nil)
			if( data == nil )
				super(41180)
			else
				super(41180, data)
			end
		end
	end

	class PaymentStreamFlatRateAmount < Quickfix::DoubleField
		def PaymentStreamFlatRateAmount.field
			return 41181
		end
		def initialize(data = nil)
			if( data == nil )
				super(41181)
			else
				super(41181, data)
			end
		end
	end

	class PaymentStreamFlatRateCurrency < Quickfix::StringField
		def PaymentStreamFlatRateCurrency.field
			return 41182
		end
		def initialize(data = nil)
			if( data == nil )
				super(41182)
			else
				super(41182, data)
			end
		end
	end

	class PaymentStreamMaximumPaymentAmount < Quickfix::DoubleField
		def PaymentStreamMaximumPaymentAmount.field
			return 41183
		end
		def initialize(data = nil)
			if( data == nil )
				super(41183)
			else
				super(41183, data)
			end
		end
	end

	class PaymentStreamMaximumPaymentCurrency < Quickfix::StringField
		def PaymentStreamMaximumPaymentCurrency.field
			return 41184
		end
		def initialize(data = nil)
			if( data == nil )
				super(41184)
			else
				super(41184, data)
			end
		end
	end

	class PaymentStreamMaximumTransactionAmount < Quickfix::DoubleField
		def PaymentStreamMaximumTransactionAmount.field
			return 41185
		end
		def initialize(data = nil)
			if( data == nil )
				super(41185)
			else
				super(41185, data)
			end
		end
	end

	class PaymentStreamMaximumTransactionCurrency < Quickfix::StringField
		def PaymentStreamMaximumTransactionCurrency.field
			return 41186
		end
		def initialize(data = nil)
			if( data == nil )
				super(41186)
			else
				super(41186, data)
			end
		end
	end

	class PaymentStreamFixedAmountUnitOfMeasure < Quickfix::StringField
		def PaymentStreamFixedAmountUnitOfMeasure.field
			return 41187
		end
		def initialize(data = nil)
			if( data == nil )
				super(41187)
			else
				super(41187, data)
			end
		end
	end

	class PaymentStreamTotalFixedAmount < Quickfix::DoubleField
		def PaymentStreamTotalFixedAmount.field
			return 41188
		end
		def initialize(data = nil)
			if( data == nil )
				super(41188)
			else
				super(41188, data)
			end
		end
	end

	class PaymentStreamWorldScaleRate < Quickfix::DoubleField
		def PaymentStreamWorldScaleRate.field
			return 41189
		end
		def initialize(data = nil)
			if( data == nil )
				super(41189)
			else
				super(41189, data)
			end
		end
	end

	class PaymentStreamContractPrice < Quickfix::DoubleField
		def PaymentStreamContractPrice.field
			return 41190
		end
		def initialize(data = nil)
			if( data == nil )
				super(41190)
			else
				super(41190, data)
			end
		end
	end

	class PaymentStreamContractPriceCurrency < Quickfix::StringField
		def PaymentStreamContractPriceCurrency.field
			return 41191
		end
		def initialize(data = nil)
			if( data == nil )
				super(41191)
			else
				super(41191, data)
			end
		end
	end

	class NoPaymentStreamPricingBusinessCenters < Quickfix::IntField
		def NoPaymentStreamPricingBusinessCenters.field
			return 41192
		end
		def initialize(data = nil)
			if( data == nil )
				super(41192)
			else
				super(41192, data)
			end
		end
	end

	class PaymentStreamPricingBusinessCenter < Quickfix::StringField
		def PaymentStreamPricingBusinessCenter.field
			return 41193
		end
		def initialize(data = nil)
			if( data == nil )
				super(41193)
			else
				super(41193, data)
			end
		end
	end

	class PaymentStreamRateIndex2CurvePeriod < Quickfix::IntField
		def PaymentStreamRateIndex2CurvePeriod.field
			return 41194
		end
		def initialize(data = nil)
			if( data == nil )
				super(41194)
			else
				super(41194, data)
			end
		end
	end

	class PaymentStreamRateIndex2CurveUnit < Quickfix::StringField
		def PaymentStreamRateIndex2CurveUnit.field
			return 41195
		end
		def initialize(data = nil)
			if( data == nil )
				super(41195)
			else
				super(41195, data)
			end
		end
	end

	class PaymentStreamRateIndexLocation < Quickfix::StringField
		def PaymentStreamRateIndexLocation.field
			return 41196
		end
		def initialize(data = nil)
			if( data == nil )
				super(41196)
			else
				super(41196, data)
			end
		end
	end

	class PaymentStreamRateIndexLevel < Quickfix::DoubleField
		def PaymentStreamRateIndexLevel.field
			return 41197
		end
		def initialize(data = nil)
			if( data == nil )
				super(41197)
			else
				super(41197, data)
			end
		end
	end

	class PaymentStreamRateIndexUnitOfMeasure < Quickfix::StringField
		def PaymentStreamRateIndexUnitOfMeasure.field
			return 41198
		end
		def initialize(data = nil)
			if( data == nil )
				super(41198)
			else
				super(41198, data)
			end
		end
	end

	class PaymentStreamSettlLevel < Quickfix::IntField
		def PaymentStreamSettlLevel.field
			return 41199
		end
		def initialize(data = nil)
			if( data == nil )
				super(41199)
			else
				super(41199, data)
			end
		end
	end

	class PaymentStreamReferenceLevel < Quickfix::DoubleField
		def PaymentStreamReferenceLevel.field
			return 41200
		end
		def initialize(data = nil)
			if( data == nil )
				super(41200)
			else
				super(41200, data)
			end
		end
	end

	class PaymentStreamReferenceLevelUnitOfMeasure < Quickfix::StringField
		def PaymentStreamReferenceLevelUnitOfMeasure.field
			return 41201
		end
		def initialize(data = nil)
			if( data == nil )
				super(41201)
			else
				super(41201, data)
			end
		end
	end

	class PaymentStreamReferenceLevelEqualsZeroIndicator < Quickfix::BoolField
		def PaymentStreamReferenceLevelEqualsZeroIndicator.field
			return 41202
		end
		def initialize(data = nil)
			if( data == nil )
				super(41202)
			else
				super(41202, data)
			end
		end
	end

	class PaymentStreamRateSpreadCurrency < Quickfix::StringField
		def PaymentStreamRateSpreadCurrency.field
			return 41203
		end
		def initialize(data = nil)
			if( data == nil )
				super(41203)
			else
				super(41203, data)
			end
		end
	end

	class PaymentStreamRateSpreadUnitOfMeasure < Quickfix::StringField
		def PaymentStreamRateSpreadUnitOfMeasure.field
			return 41204
		end
		def initialize(data = nil)
			if( data == nil )
				super(41204)
			else
				super(41204, data)
			end
		end
	end

	class PaymentStreamRateConversionFactor < Quickfix::DoubleField
		def PaymentStreamRateConversionFactor.field
			return 41205
		end
		def initialize(data = nil)
			if( data == nil )
				super(41205)
			else
				super(41205, data)
			end
		end
	end

	class PaymentStreamRateSpreadType < Quickfix::IntField
		def PaymentStreamRateSpreadType.field
			return 41206
		end
		def initialize(data = nil)
			if( data == nil )
				super(41206)
			else
				super(41206, data)
			end
		end
	end

	class PaymentStreamLastResetRate < Quickfix::DoubleField
		def PaymentStreamLastResetRate.field
			return 41207
		end
		def initialize(data = nil)
			if( data == nil )
				super(41207)
			else
				super(41207, data)
			end
		end
	end

	class PaymentStreamFinalRate < Quickfix::DoubleField
		def PaymentStreamFinalRate.field
			return 41208
		end
		def initialize(data = nil)
			if( data == nil )
				super(41208)
			else
				super(41208, data)
			end
		end
	end

	class PaymentStreamCalculationLagPeriod < Quickfix::IntField
		def PaymentStreamCalculationLagPeriod.field
			return 41209
		end
		def initialize(data = nil)
			if( data == nil )
				super(41209)
			else
				super(41209, data)
			end
		end
	end

	class PaymentStreamCalculationLagUnit < Quickfix::StringField
		def PaymentStreamCalculationLagUnit.field
			return 41210
		end
		def initialize(data = nil)
			if( data == nil )
				super(41210)
			else
				super(41210, data)
			end
		end
	end

	class PaymentStreamFirstObservationDateOffsetPeriod < Quickfix::IntField
		def PaymentStreamFirstObservationDateOffsetPeriod.field
			return 41211
		end
		def initialize(data = nil)
			if( data == nil )
				super(41211)
			else
				super(41211, data)
			end
		end
	end

	class PaymentStreamFirstObservationDateOffsetUnit < Quickfix::StringField
		def PaymentStreamFirstObservationDateOffsetUnit.field
			return 41212
		end
		def initialize(data = nil)
			if( data == nil )
				super(41212)
			else
				super(41212, data)
			end
		end
	end

	class PaymentStreamPricingDayType < Quickfix::IntField
		def PaymentStreamPricingDayType.field
			return 41213
		end
		def initialize(data = nil)
			if( data == nil )
				super(41213)
			else
				super(41213, data)
			end
		end
	end

	class PaymentStreamPricingDayDistribution < Quickfix::IntField
		def PaymentStreamPricingDayDistribution.field
			return 41214
		end
		def initialize(data = nil)
			if( data == nil )
				super(41214)
			else
				super(41214, data)
			end
		end
	end

	class PaymentStreamPricingDayCount < Quickfix::IntField
		def PaymentStreamPricingDayCount.field
			return 41215
		end
		def initialize(data = nil)
			if( data == nil )
				super(41215)
			else
				super(41215, data)
			end
		end
	end

	class PaymentStreamPricingBusinessCalendar < Quickfix::StringField
		def PaymentStreamPricingBusinessCalendar.field
			return 41216
		end
		def initialize(data = nil)
			if( data == nil )
				super(41216)
			else
				super(41216, data)
			end
		end
	end

	class PaymentStreamPricingBusinessDayConvention < Quickfix::IntField
		def PaymentStreamPricingBusinessDayConvention.field
			return 41217
		end
		def initialize(data = nil)
			if( data == nil )
				super(41217)
			else
				super(41217, data)
			end
		end
	end

	class NoPaymentStreamPaymentDates < Quickfix::IntField
		def NoPaymentStreamPaymentDates.field
			return 41220
		end
		def initialize(data = nil)
			if( data == nil )
				super(41220)
			else
				super(41220, data)
			end
		end
	end

	class PaymentStreamPaymentDate < Quickfix::StringField
		def PaymentStreamPaymentDate.field
			return 41221
		end
		def initialize(data = nil)
			if( data == nil )
				super(41221)
			else
				super(41221, data)
			end
		end
	end

	class PaymentStreamPaymentDateType < Quickfix::IntField
		def PaymentStreamPaymentDateType.field
			return 41222
		end
		def initialize(data = nil)
			if( data == nil )
				super(41222)
			else
				super(41222, data)
			end
		end
	end

	class PaymentStreamMasterAgreementPaymentDatesIndicator < Quickfix::BoolField
		def PaymentStreamMasterAgreementPaymentDatesIndicator.field
			return 41223
		end
		def initialize(data = nil)
			if( data == nil )
				super(41223)
			else
				super(41223, data)
			end
		end
	end

	class NoPaymentStreamPricingDates < Quickfix::IntField
		def NoPaymentStreamPricingDates.field
			return 41224
		end
		def initialize(data = nil)
			if( data == nil )
				super(41224)
			else
				super(41224, data)
			end
		end
	end

	class PaymentStreamPricingDate < Quickfix::StringField
		def PaymentStreamPricingDate.field
			return 41225
		end
		def initialize(data = nil)
			if( data == nil )
				super(41225)
			else
				super(41225, data)
			end
		end
	end

	class PaymentStreamPricingDateType < Quickfix::IntField
		def PaymentStreamPricingDateType.field
			return 41226
		end
		def initialize(data = nil)
			if( data == nil )
				super(41226)
			else
				super(41226, data)
			end
		end
	end

	class NoPaymentStreamPricingDays < Quickfix::IntField
		def NoPaymentStreamPricingDays.field
			return 41227
		end
		def initialize(data = nil)
			if( data == nil )
				super(41227)
			else
				super(41227, data)
			end
		end
	end

	class PaymentStreamPricingDayOfWeek < Quickfix::IntField
		def PaymentStreamPricingDayOfWeek.field
			return 41228
		end
		def initialize(data = nil)
			if( data == nil )
				super(41228)
			else
				super(41228, data)
			end
		end
	end

	class PaymentStreamPricingDayNumber < Quickfix::IntField
		def PaymentStreamPricingDayNumber.field
			return 41229
		end
		def initialize(data = nil)
			if( data == nil )
				super(41229)
			else
				super(41229, data)
			end
		end
	end

	class NoPricingDateBusinessCenters < Quickfix::IntField
		def NoPricingDateBusinessCenters.field
			return 41230
		end
		def initialize(data = nil)
			if( data == nil )
				super(41230)
			else
				super(41230, data)
			end
		end
	end

	class PricingDateBusinessCenter < Quickfix::StringField
		def PricingDateBusinessCenter.field
			return 41231
		end
		def initialize(data = nil)
			if( data == nil )
				super(41231)
			else
				super(41231, data)
			end
		end
	end

	class PricingDateUnadjusted < Quickfix::StringField
		def PricingDateUnadjusted.field
			return 41232
		end
		def initialize(data = nil)
			if( data == nil )
				super(41232)
			else
				super(41232, data)
			end
		end
	end

	class PricingDateBusinessDayConvention < Quickfix::IntField
		def PricingDateBusinessDayConvention.field
			return 41233
		end
		def initialize(data = nil)
			if( data == nil )
				super(41233)
			else
				super(41233, data)
			end
		end
	end

	class PricingDateAdjusted < Quickfix::StringField
		def PricingDateAdjusted.field
			return 41234
		end
		def initialize(data = nil)
			if( data == nil )
				super(41234)
			else
				super(41234, data)
			end
		end
	end

	class PricingTime < Quickfix::StringField
		def PricingTime.field
			return 41235
		end
		def initialize(data = nil)
			if( data == nil )
				super(41235)
			else
				super(41235, data)
			end
		end
	end

	class PricingTimeBusinessCenter < Quickfix::StringField
		def PricingTimeBusinessCenter.field
			return 41236
		end
		def initialize(data = nil)
			if( data == nil )
				super(41236)
			else
				super(41236, data)
			end
		end
	end

	class NoStreamAssetAttributes < Quickfix::IntField
		def NoStreamAssetAttributes.field
			return 41237
		end
		def initialize(data = nil)
			if( data == nil )
				super(41237)
			else
				super(41237, data)
			end
		end
	end

	class StreamAssetAttributeType < Quickfix::StringField
		def StreamAssetAttributeType.field
			return 41238
		end
		def initialize(data = nil)
			if( data == nil )
				super(41238)
			else
				super(41238, data)
			end
		end
	end

	class StreamAssetAttributeValue < Quickfix::StringField
		def StreamAssetAttributeValue.field
			return 41239
		end
		def initialize(data = nil)
			if( data == nil )
				super(41239)
			else
				super(41239, data)
			end
		end
	end

	class StreamAssetAttributeLimit < Quickfix::StringField
		def StreamAssetAttributeLimit.field
			return 41240
		end
		def initialize(data = nil)
			if( data == nil )
				super(41240)
			else
				super(41240, data)
			end
		end
	end

	class NoStreamCalculationPeriodDates < Quickfix::IntField
		def NoStreamCalculationPeriodDates.field
			return 41241
		end
		def initialize(data = nil)
			if( data == nil )
				super(41241)
			else
				super(41241, data)
			end
		end
	end

	class StreamCalculationPeriodDate < Quickfix::StringField
		def StreamCalculationPeriodDate.field
			return 41242
		end
		def initialize(data = nil)
			if( data == nil )
				super(41242)
			else
				super(41242, data)
			end
		end
	end

	class StreamCalculationPeriodDateType < Quickfix::IntField
		def StreamCalculationPeriodDateType.field
			return 41243
		end
		def initialize(data = nil)
			if( data == nil )
				super(41243)
			else
				super(41243, data)
			end
		end
	end

	class StreamCalculationPeriodDatesXID < Quickfix::StringField
		def StreamCalculationPeriodDatesXID.field
			return 41244
		end
		def initialize(data = nil)
			if( data == nil )
				super(41244)
			else
				super(41244, data)
			end
		end
	end

	class StreamCalculationPeriodDatesXIDRef < Quickfix::StringField
		def StreamCalculationPeriodDatesXIDRef.field
			return 41245
		end
		def initialize(data = nil)
			if( data == nil )
				super(41245)
			else
				super(41245, data)
			end
		end
	end

	class StreamCalculationBalanceOfFirstPeriod < Quickfix::BoolField
		def StreamCalculationBalanceOfFirstPeriod.field
			return 41246
		end
		def initialize(data = nil)
			if( data == nil )
				super(41246)
			else
				super(41246, data)
			end
		end
	end

	class StreamCalculationCorrectionPeriod < Quickfix::IntField
		def StreamCalculationCorrectionPeriod.field
			return 41247
		end
		def initialize(data = nil)
			if( data == nil )
				super(41247)
			else
				super(41247, data)
			end
		end
	end

	class StreamCalculationCorrectionUnit < Quickfix::StringField
		def StreamCalculationCorrectionUnit.field
			return 41248
		end
		def initialize(data = nil)
			if( data == nil )
				super(41248)
			else
				super(41248, data)
			end
		end
	end

	class NoStreamCommoditySettlBusinessCenters < Quickfix::IntField
		def NoStreamCommoditySettlBusinessCenters.field
			return 41249
		end
		def initialize(data = nil)
			if( data == nil )
				super(41249)
			else
				super(41249, data)
			end
		end
	end

	class StreamCommoditySettlBusinessCenter < Quickfix::StringField
		def StreamCommoditySettlBusinessCenter.field
			return 41250
		end
		def initialize(data = nil)
			if( data == nil )
				super(41250)
			else
				super(41250, data)
			end
		end
	end

	class StreamCommodityBase < Quickfix::StringField
		def StreamCommodityBase.field
			return 41251
		end
		def initialize(data = nil)
			if( data == nil )
				super(41251)
			else
				super(41251, data)
			end
		end
	end

	class StreamCommodityType < Quickfix::StringField
		def StreamCommodityType.field
			return 41252
		end
		def initialize(data = nil)
			if( data == nil )
				super(41252)
			else
				super(41252, data)
			end
		end
	end

	class StreamCommoditySecurityID < Quickfix::StringField
		def StreamCommoditySecurityID.field
			return 41253
		end
		def initialize(data = nil)
			if( data == nil )
				super(41253)
			else
				super(41253, data)
			end
		end
	end

	class StreamCommoditySecurityIDSource < Quickfix::StringField
		def StreamCommoditySecurityIDSource.field
			return 41254
		end
		def initialize(data = nil)
			if( data == nil )
				super(41254)
			else
				super(41254, data)
			end
		end
	end

	class StreamCommodityDesc < Quickfix::StringField
		def StreamCommodityDesc.field
			return 41255
		end
		def initialize(data = nil)
			if( data == nil )
				super(41255)
			else
				super(41255, data)
			end
		end
	end

	class EncodedStreamCommodityDescLen < Quickfix::IntField
		def EncodedStreamCommodityDescLen.field
			return 41256
		end
		def initialize(data = nil)
			if( data == nil )
				super(41256)
			else
				super(41256, data)
			end
		end
	end

	class EncodedStreamCommodityDesc < Quickfix::StringField
		def EncodedStreamCommodityDesc.field
			return 41257
		end
		def initialize(data = nil)
			if( data == nil )
				super(41257)
			else
				super(41257, data)
			end
		end
	end

	class StreamCommodityUnitOfMeasure < Quickfix::StringField
		def StreamCommodityUnitOfMeasure.field
			return 41258
		end
		def initialize(data = nil)
			if( data == nil )
				super(41258)
			else
				super(41258, data)
			end
		end
	end

	class StreamCommodityCurrency < Quickfix::StringField
		def StreamCommodityCurrency.field
			return 41259
		end
		def initialize(data = nil)
			if( data == nil )
				super(41259)
			else
				super(41259, data)
			end
		end
	end

	class StreamCommodityExchange < Quickfix::StringField
		def StreamCommodityExchange.field
			return 41260
		end
		def initialize(data = nil)
			if( data == nil )
				super(41260)
			else
				super(41260, data)
			end
		end
	end

	class StreamCommodityRateSource < Quickfix::IntField
		def StreamCommodityRateSource.field
			return 41261
		end
		def initialize(data = nil)
			if( data == nil )
				super(41261)
			else
				super(41261, data)
			end
		end
	end

	class StreamCommodityRateReferencePage < Quickfix::StringField
		def StreamCommodityRateReferencePage.field
			return 41262
		end
		def initialize(data = nil)
			if( data == nil )
				super(41262)
			else
				super(41262, data)
			end
		end
	end

	class StreamCommodityRateReferencePageHeading < Quickfix::StringField
		def StreamCommodityRateReferencePageHeading.field
			return 41263
		end
		def initialize(data = nil)
			if( data == nil )
				super(41263)
			else
				super(41263, data)
			end
		end
	end

	class StreamDataProvider < Quickfix::StringField
		def StreamDataProvider.field
			return 41264
		end
		def initialize(data = nil)
			if( data == nil )
				super(41264)
			else
				super(41264, data)
			end
		end
	end

	class StreamCommodityPricingType < Quickfix::StringField
		def StreamCommodityPricingType.field
			return 41265
		end
		def initialize(data = nil)
			if( data == nil )
				super(41265)
			else
				super(41265, data)
			end
		end
	end

	class StreamCommodityNearbySettlDayPeriod < Quickfix::IntField
		def StreamCommodityNearbySettlDayPeriod.field
			return 41266
		end
		def initialize(data = nil)
			if( data == nil )
				super(41266)
			else
				super(41266, data)
			end
		end
	end

	class StreamCommodityNearbySettlDayUnit < Quickfix::StringField
		def StreamCommodityNearbySettlDayUnit.field
			return 41267
		end
		def initialize(data = nil)
			if( data == nil )
				super(41267)
			else
				super(41267, data)
			end
		end
	end

	class StreamCommoditySettlDateUnadjusted < Quickfix::StringField
		def StreamCommoditySettlDateUnadjusted.field
			return 41268
		end
		def initialize(data = nil)
			if( data == nil )
				super(41268)
			else
				super(41268, data)
			end
		end
	end

	class StreamCommoditySettlDateBusinessDayConvention < Quickfix::IntField
		def StreamCommoditySettlDateBusinessDayConvention.field
			return 41269
		end
		def initialize(data = nil)
			if( data == nil )
				super(41269)
			else
				super(41269, data)
			end
		end
	end

	class StreamCommoditySettlDateAdjusted < Quickfix::StringField
		def StreamCommoditySettlDateAdjusted.field
			return 41270
		end
		def initialize(data = nil)
			if( data == nil )
				super(41270)
			else
				super(41270, data)
			end
		end
	end

	class StreamCommoditySettlMonth < Quickfix::IntField
		def StreamCommoditySettlMonth.field
			return 41271
		end
		def initialize(data = nil)
			if( data == nil )
				super(41271)
			else
				super(41271, data)
			end
		end
	end

	class StreamCommoditySettlDateRollPeriod < Quickfix::IntField
		def StreamCommoditySettlDateRollPeriod.field
			return 41272
		end
		def initialize(data = nil)
			if( data == nil )
				super(41272)
			else
				super(41272, data)
			end
		end
	end

	class StreamCommoditySettlDateRollUnit < Quickfix::StringField
		def StreamCommoditySettlDateRollUnit.field
			return 41273
		end
		def initialize(data = nil)
			if( data == nil )
				super(41273)
			else
				super(41273, data)
			end
		end
	end

	class StreamCommoditySettlDayType < Quickfix::IntField
		def StreamCommoditySettlDayType.field
			return 41274
		end
		def initialize(data = nil)
			if( data == nil )
				super(41274)
			else
				super(41274, data)
			end
		end
	end

	class StreamCommodityXID < Quickfix::StringField
		def StreamCommodityXID.field
			return 41275
		end
		def initialize(data = nil)
			if( data == nil )
				super(41275)
			else
				super(41275, data)
			end
		end
	end

	class StreamCommodityXIDRef < Quickfix::StringField
		def StreamCommodityXIDRef.field
			return 41276
		end
		def initialize(data = nil)
			if( data == nil )
				super(41276)
			else
				super(41276, data)
			end
		end
	end

	class NoStreamCommodityAltIDs < Quickfix::IntField
		def NoStreamCommodityAltIDs.field
			return 41277
		end
		def initialize(data = nil)
			if( data == nil )
				super(41277)
			else
				super(41277, data)
			end
		end
	end

	class StreamCommodityAltID < Quickfix::StringField
		def StreamCommodityAltID.field
			return 41278
		end
		def initialize(data = nil)
			if( data == nil )
				super(41278)
			else
				super(41278, data)
			end
		end
	end

	class StreamCommodityAltIDSource < Quickfix::StringField
		def StreamCommodityAltIDSource.field
			return 41279
		end
		def initialize(data = nil)
			if( data == nil )
				super(41279)
			else
				super(41279, data)
			end
		end
	end

	class NoStreamCommodityDataSources < Quickfix::IntField
		def NoStreamCommodityDataSources.field
			return 41280
		end
		def initialize(data = nil)
			if( data == nil )
				super(41280)
			else
				super(41280, data)
			end
		end
	end

	class StreamCommodityDataSourceID < Quickfix::StringField
		def StreamCommodityDataSourceID.field
			return 41281
		end
		def initialize(data = nil)
			if( data == nil )
				super(41281)
			else
				super(41281, data)
			end
		end
	end

	class StreamCommodityDataSourceIDType < Quickfix::IntField
		def StreamCommodityDataSourceIDType.field
			return 41282
		end
		def initialize(data = nil)
			if( data == nil )
				super(41282)
			else
				super(41282, data)
			end
		end
	end

	class NoStreamCommoditySettlDays < Quickfix::IntField
		def NoStreamCommoditySettlDays.field
			return 41283
		end
		def initialize(data = nil)
			if( data == nil )
				super(41283)
			else
				super(41283, data)
			end
		end
	end

	class StreamCommoditySettlDay < Quickfix::IntField
		def StreamCommoditySettlDay.field
			return 41284
		end
		def initialize(data = nil)
			if( data == nil )
				super(41284)
			else
				super(41284, data)
			end
		end
	end

	class StreamCommoditySettlTotalHours < Quickfix::IntField
		def StreamCommoditySettlTotalHours.field
			return 41285
		end
		def initialize(data = nil)
			if( data == nil )
				super(41285)
			else
				super(41285, data)
			end
		end
	end

	class NoStreamCommoditySettlTimes < Quickfix::IntField
		def NoStreamCommoditySettlTimes.field
			return 41286
		end
		def initialize(data = nil)
			if( data == nil )
				super(41286)
			else
				super(41286, data)
			end
		end
	end

	class StreamCommoditySettlStart < Quickfix::StringField
		def StreamCommoditySettlStart.field
			return 41287
		end
		def initialize(data = nil)
			if( data == nil )
				super(41287)
			else
				super(41287, data)
			end
		end
	end

	class StreamCommoditySettlEnd < Quickfix::StringField
		def StreamCommoditySettlEnd.field
			return 41288
		end
		def initialize(data = nil)
			if( data == nil )
				super(41288)
			else
				super(41288, data)
			end
		end
	end

	class StreamCommoditySettlTimeType < Quickfix::IntField
		def StreamCommoditySettlTimeType.field
			return 41588
		end
		def initialize(data = nil)
			if( data == nil )
				super(41588)
			else
				super(41588, data)
			end
		end
	end

	class NoStreamCommoditySettlPeriods < Quickfix::IntField
		def NoStreamCommoditySettlPeriods.field
			return 41289
		end
		def initialize(data = nil)
			if( data == nil )
				super(41289)
			else
				super(41289, data)
			end
		end
	end

	class StreamCommoditySettlCountry < Quickfix::StringField
		def StreamCommoditySettlCountry.field
			return 41290
		end
		def initialize(data = nil)
			if( data == nil )
				super(41290)
			else
				super(41290, data)
			end
		end
	end

	class StreamCommoditySettlTimeZone < Quickfix::StringField
		def StreamCommoditySettlTimeZone.field
			return 41291
		end
		def initialize(data = nil)
			if( data == nil )
				super(41291)
			else
				super(41291, data)
			end
		end
	end

	class StreamCommoditySettlFlowType < Quickfix::IntField
		def StreamCommoditySettlFlowType.field
			return 41292
		end
		def initialize(data = nil)
			if( data == nil )
				super(41292)
			else
				super(41292, data)
			end
		end
	end

	class StreamCommoditySettlPeriodNotional < Quickfix::DoubleField
		def StreamCommoditySettlPeriodNotional.field
			return 41293
		end
		def initialize(data = nil)
			if( data == nil )
				super(41293)
			else
				super(41293, data)
			end
		end
	end

	class StreamCommoditySettlPeriodNotionalUnitOfMeasure < Quickfix::StringField
		def StreamCommoditySettlPeriodNotionalUnitOfMeasure.field
			return 41294
		end
		def initialize(data = nil)
			if( data == nil )
				super(41294)
			else
				super(41294, data)
			end
		end
	end

	class StreamCommoditySettlPeriodFrequencyPeriod < Quickfix::IntField
		def StreamCommoditySettlPeriodFrequencyPeriod.field
			return 41295
		end
		def initialize(data = nil)
			if( data == nil )
				super(41295)
			else
				super(41295, data)
			end
		end
	end

	class StreamCommoditySettlPeriodFrequencyUnit < Quickfix::StringField
		def StreamCommoditySettlPeriodFrequencyUnit.field
			return 41296
		end
		def initialize(data = nil)
			if( data == nil )
				super(41296)
			else
				super(41296, data)
			end
		end
	end

	class StreamCommoditySettlPeriodPrice < Quickfix::DoubleField
		def StreamCommoditySettlPeriodPrice.field
			return 41297
		end
		def initialize(data = nil)
			if( data == nil )
				super(41297)
			else
				super(41297, data)
			end
		end
	end

	class StreamCommoditySettlPeriodPriceUnitOfMeasure < Quickfix::StringField
		def StreamCommoditySettlPeriodPriceUnitOfMeasure.field
			return 41298
		end
		def initialize(data = nil)
			if( data == nil )
				super(41298)
			else
				super(41298, data)
			end
		end
	end

	class StreamCommoditySettlPeriodPriceCurrency < Quickfix::StringField
		def StreamCommoditySettlPeriodPriceCurrency.field
			return 41299
		end
		def initialize(data = nil)
			if( data == nil )
				super(41299)
			else
				super(41299, data)
			end
		end
	end

	class StreamCommoditySettlHolidaysProcessingInstruction < Quickfix::IntField
		def StreamCommoditySettlHolidaysProcessingInstruction.field
			return 41300
		end
		def initialize(data = nil)
			if( data == nil )
				super(41300)
			else
				super(41300, data)
			end
		end
	end

	class StreamCommoditySettlPeriodXID < Quickfix::StringField
		def StreamCommoditySettlPeriodXID.field
			return 41301
		end
		def initialize(data = nil)
			if( data == nil )
				super(41301)
			else
				super(41301, data)
			end
		end
	end

	class StreamCommoditySettlPeriodXIDRef < Quickfix::StringField
		def StreamCommoditySettlPeriodXIDRef.field
			return 41302
		end
		def initialize(data = nil)
			if( data == nil )
				super(41302)
			else
				super(41302, data)
			end
		end
	end

	class StreamXID < Quickfix::StringField
		def StreamXID.field
			return 41303
		end
		def initialize(data = nil)
			if( data == nil )
				super(41303)
			else
				super(41303, data)
			end
		end
	end

	class StreamNotionalXIDRef < Quickfix::StringField
		def StreamNotionalXIDRef.field
			return 41305
		end
		def initialize(data = nil)
			if( data == nil )
				super(41305)
			else
				super(41305, data)
			end
		end
	end

	class StreamNotionalFrequencyPeriod < Quickfix::IntField
		def StreamNotionalFrequencyPeriod.field
			return 41306
		end
		def initialize(data = nil)
			if( data == nil )
				super(41306)
			else
				super(41306, data)
			end
		end
	end

	class StreamNotionalFrequencyUnit < Quickfix::StringField
		def StreamNotionalFrequencyUnit.field
			return 41307
		end
		def initialize(data = nil)
			if( data == nil )
				super(41307)
			else
				super(41307, data)
			end
		end
	end

	class StreamNotionalCommodityFrequency < Quickfix::IntField
		def StreamNotionalCommodityFrequency.field
			return 41308
		end
		def initialize(data = nil)
			if( data == nil )
				super(41308)
			else
				super(41308, data)
			end
		end
	end

	class StreamNotionalUnitOfMeasure < Quickfix::StringField
		def StreamNotionalUnitOfMeasure.field
			return 41309
		end
		def initialize(data = nil)
			if( data == nil )
				super(41309)
			else
				super(41309, data)
			end
		end
	end

	class StreamTotalNotional < Quickfix::DoubleField
		def StreamTotalNotional.field
			return 41310
		end
		def initialize(data = nil)
			if( data == nil )
				super(41310)
			else
				super(41310, data)
			end
		end
	end

	class StreamTotalNotionalUnitOfMeasure < Quickfix::StringField
		def StreamTotalNotionalUnitOfMeasure.field
			return 41311
		end
		def initialize(data = nil)
			if( data == nil )
				super(41311)
			else
				super(41311, data)
			end
		end
	end

	class NoMandatoryClearingJurisdictions < Quickfix::IntField
		def NoMandatoryClearingJurisdictions.field
			return 41312
		end
		def initialize(data = nil)
			if( data == nil )
				super(41312)
			else
				super(41312, data)
			end
		end
	end

	class MandatoryClearingJurisdiction < Quickfix::StringField
		def MandatoryClearingJurisdiction.field
			return 41313
		end
		def initialize(data = nil)
			if( data == nil )
				super(41313)
			else
				super(41313, data)
			end
		end
	end

	class LastQtyChanged < Quickfix::DoubleField
		def LastQtyChanged.field
			return 2301
		end
		def initialize(data = nil)
			if( data == nil )
				super(2301)
			else
				super(2301, data)
			end
		end
	end

	class TradeVersion < Quickfix::StringField
		def TradeVersion.field
			return 2302
		end
		def initialize(data = nil)
			if( data == nil )
				super(2302)
			else
				super(2302, data)
			end
		end
	end

	class HistoricalReportIndicator < Quickfix::BoolField
		def HistoricalReportIndicator.field
			return 2303
		end
		def initialize(data = nil)
			if( data == nil )
				super(2303)
			else
				super(2303, data)
			end
		end
	end

	class NoLegAdditionalTermBondRefs < Quickfix::IntField
		def NoLegAdditionalTermBondRefs.field
			return 41316
		end
		def initialize(data = nil)
			if( data == nil )
				super(41316)
			else
				super(41316, data)
			end
		end
	end

	class LegAdditionalTermBondSecurityID < Quickfix::StringField
		def LegAdditionalTermBondSecurityID.field
			return 41317
		end
		def initialize(data = nil)
			if( data == nil )
				super(41317)
			else
				super(41317, data)
			end
		end
	end

	class LegAdditionalTermBondSecurityIDSource < Quickfix::StringField
		def LegAdditionalTermBondSecurityIDSource.field
			return 41318
		end
		def initialize(data = nil)
			if( data == nil )
				super(41318)
			else
				super(41318, data)
			end
		end
	end

	class LegAdditionalTermBondDesc < Quickfix::StringField
		def LegAdditionalTermBondDesc.field
			return 41319
		end
		def initialize(data = nil)
			if( data == nil )
				super(41319)
			else
				super(41319, data)
			end
		end
	end

	class EncodedLegAdditionalTermBondDescLen < Quickfix::IntField
		def EncodedLegAdditionalTermBondDescLen.field
			return 41320
		end
		def initialize(data = nil)
			if( data == nil )
				super(41320)
			else
				super(41320, data)
			end
		end
	end

	class EncodedLegAdditionalTermBondDesc < Quickfix::StringField
		def EncodedLegAdditionalTermBondDesc.field
			return 41321
		end
		def initialize(data = nil)
			if( data == nil )
				super(41321)
			else
				super(41321, data)
			end
		end
	end

	class LegAdditionalTermBondCurrency < Quickfix::StringField
		def LegAdditionalTermBondCurrency.field
			return 41322
		end
		def initialize(data = nil)
			if( data == nil )
				super(41322)
			else
				super(41322, data)
			end
		end
	end

	class LegAdditionalTermBondIssuer < Quickfix::StringField
		def LegAdditionalTermBondIssuer.field
			return 41323
		end
		def initialize(data = nil)
			if( data == nil )
				super(41323)
			else
				super(41323, data)
			end
		end
	end

	class EncodedLegAdditionalTermBondIssuerLen < Quickfix::IntField
		def EncodedLegAdditionalTermBondIssuerLen.field
			return 41324
		end
		def initialize(data = nil)
			if( data == nil )
				super(41324)
			else
				super(41324, data)
			end
		end
	end

	class EncodedLegAdditionalTermBondIssuer < Quickfix::StringField
		def EncodedLegAdditionalTermBondIssuer.field
			return 41325
		end
		def initialize(data = nil)
			if( data == nil )
				super(41325)
			else
				super(41325, data)
			end
		end
	end

	class LegAdditionalTermBondSeniority < Quickfix::StringField
		def LegAdditionalTermBondSeniority.field
			return 41326
		end
		def initialize(data = nil)
			if( data == nil )
				super(41326)
			else
				super(41326, data)
			end
		end
	end

	class LegAdditionalTermBondCouponType < Quickfix::IntField
		def LegAdditionalTermBondCouponType.field
			return 41327
		end
		def initialize(data = nil)
			if( data == nil )
				super(41327)
			else
				super(41327, data)
			end
		end
	end

	class LegAdditionalTermBondCouponRate < Quickfix::DoubleField
		def LegAdditionalTermBondCouponRate.field
			return 41328
		end
		def initialize(data = nil)
			if( data == nil )
				super(41328)
			else
				super(41328, data)
			end
		end
	end

	class LegAdditionalTermBondMaturityDate < Quickfix::StringField
		def LegAdditionalTermBondMaturityDate.field
			return 41329
		end
		def initialize(data = nil)
			if( data == nil )
				super(41329)
			else
				super(41329, data)
			end
		end
	end

	class LegAdditionalTermBondParValue < Quickfix::DoubleField
		def LegAdditionalTermBondParValue.field
			return 41330
		end
		def initialize(data = nil)
			if( data == nil )
				super(41330)
			else
				super(41330, data)
			end
		end
	end

	class LegAdditionalTermBondCurrentTotalIssuedAmount < Quickfix::DoubleField
		def LegAdditionalTermBondCurrentTotalIssuedAmount.field
			return 41331
		end
		def initialize(data = nil)
			if( data == nil )
				super(41331)
			else
				super(41331, data)
			end
		end
	end

	class LegAdditionalTermBondCouponFrequencyPeriod < Quickfix::IntField
		def LegAdditionalTermBondCouponFrequencyPeriod.field
			return 41332
		end
		def initialize(data = nil)
			if( data == nil )
				super(41332)
			else
				super(41332, data)
			end
		end
	end

	class LegAdditionalTermBondCouponFrequencyUnit < Quickfix::StringField
		def LegAdditionalTermBondCouponFrequencyUnit.field
			return 41333
		end
		def initialize(data = nil)
			if( data == nil )
				super(41333)
			else
				super(41333, data)
			end
		end
	end

	class LegAdditionalTermBondDayCount < Quickfix::IntField
		def LegAdditionalTermBondDayCount.field
			return 41334
		end
		def initialize(data = nil)
			if( data == nil )
				super(41334)
			else
				super(41334, data)
			end
		end
	end

	class NoLegAdditionalTerms < Quickfix::IntField
		def NoLegAdditionalTerms.field
			return 41335
		end
		def initialize(data = nil)
			if( data == nil )
				super(41335)
			else
				super(41335, data)
			end
		end
	end

	class LegAdditionalTermConditionPrecedentBondIndicator < Quickfix::BoolField
		def LegAdditionalTermConditionPrecedentBondIndicator.field
			return 41336
		end
		def initialize(data = nil)
			if( data == nil )
				super(41336)
			else
				super(41336, data)
			end
		end
	end

	class LegAdditionalTermDiscrepancyClauseIndicator < Quickfix::BoolField
		def LegAdditionalTermDiscrepancyClauseIndicator.field
			return 41337
		end
		def initialize(data = nil)
			if( data == nil )
				super(41337)
			else
				super(41337, data)
			end
		end
	end

	class NoLegAssetAttributes < Quickfix::IntField
		def NoLegAssetAttributes.field
			return 2308
		end
		def initialize(data = nil)
			if( data == nil )
				super(2308)
			else
				super(2308, data)
			end
		end
	end

	class LegAssetAttributeType < Quickfix::StringField
		def LegAssetAttributeType.field
			return 2309
		end
		def initialize(data = nil)
			if( data == nil )
				super(2309)
			else
				super(2309, data)
			end
		end
	end

	class LegAssetAttributeValue < Quickfix::StringField
		def LegAssetAttributeValue.field
			return 2310
		end
		def initialize(data = nil)
			if( data == nil )
				super(2310)
			else
				super(2310, data)
			end
		end
	end

	class LegAssetAttributeLimit < Quickfix::StringField
		def LegAssetAttributeLimit.field
			return 2311
		end
		def initialize(data = nil)
			if( data == nil )
				super(2311)
			else
				super(2311, data)
			end
		end
	end

	class NoLegCashSettlDealers < Quickfix::IntField
		def NoLegCashSettlDealers.field
			return 41342
		end
		def initialize(data = nil)
			if( data == nil )
				super(41342)
			else
				super(41342, data)
			end
		end
	end

	class LegCashSettlDealer < Quickfix::StringField
		def LegCashSettlDealer.field
			return 41343
		end
		def initialize(data = nil)
			if( data == nil )
				super(41343)
			else
				super(41343, data)
			end
		end
	end

	class NoLegCashSettlTerms < Quickfix::IntField
		def NoLegCashSettlTerms.field
			return 41344
		end
		def initialize(data = nil)
			if( data == nil )
				super(41344)
			else
				super(41344, data)
			end
		end
	end

	class LegCashSettlCurrency < Quickfix::StringField
		def LegCashSettlCurrency.field
			return 41345
		end
		def initialize(data = nil)
			if( data == nil )
				super(41345)
			else
				super(41345, data)
			end
		end
	end

	class LegCasSettlValuationFirstBusinessDayOffset < Quickfix::IntField
		def LegCasSettlValuationFirstBusinessDayOffset.field
			return 41346
		end
		def initialize(data = nil)
			if( data == nil )
				super(41346)
			else
				super(41346, data)
			end
		end
	end

	class LegCashSettlValuationSubsequentBusinessDaysOffset < Quickfix::IntField
		def LegCashSettlValuationSubsequentBusinessDaysOffset.field
			return 41347
		end
		def initialize(data = nil)
			if( data == nil )
				super(41347)
			else
				super(41347, data)
			end
		end
	end

	class LegCashSettlNumOfValuationDates < Quickfix::IntField
		def LegCashSettlNumOfValuationDates.field
			return 41348
		end
		def initialize(data = nil)
			if( data == nil )
				super(41348)
			else
				super(41348, data)
			end
		end
	end

	class LegCashSettlValuationTime < Quickfix::StringField
		def LegCashSettlValuationTime.field
			return 41349
		end
		def initialize(data = nil)
			if( data == nil )
				super(41349)
			else
				super(41349, data)
			end
		end
	end

	class LegCashSettlBusinessCenter < Quickfix::StringField
		def LegCashSettlBusinessCenter.field
			return 41350
		end
		def initialize(data = nil)
			if( data == nil )
				super(41350)
			else
				super(41350, data)
			end
		end
	end

	class LegCashSettlQuoteMethod < Quickfix::IntField
		def LegCashSettlQuoteMethod.field
			return 41351
		end
		def initialize(data = nil)
			if( data == nil )
				super(41351)
			else
				super(41351, data)
			end
		end
	end

	class LegCashSettlQuoteAmount < Quickfix::DoubleField
		def LegCashSettlQuoteAmount.field
			return 41352
		end
		def initialize(data = nil)
			if( data == nil )
				super(41352)
			else
				super(41352, data)
			end
		end
	end

	class LegCashSettlQuoteCurrency < Quickfix::StringField
		def LegCashSettlQuoteCurrency.field
			return 41353
		end
		def initialize(data = nil)
			if( data == nil )
				super(41353)
			else
				super(41353, data)
			end
		end
	end

	class LegCashSettlMinimumQuoteAmount < Quickfix::DoubleField
		def LegCashSettlMinimumQuoteAmount.field
			return 41354
		end
		def initialize(data = nil)
			if( data == nil )
				super(41354)
			else
				super(41354, data)
			end
		end
	end

	class LegCashSettlMinimumQuoteCurrency < Quickfix::StringField
		def LegCashSettlMinimumQuoteCurrency.field
			return 41355
		end
		def initialize(data = nil)
			if( data == nil )
				super(41355)
			else
				super(41355, data)
			end
		end
	end

	class LegCashSettlBusinessDays < Quickfix::IntField
		def LegCashSettlBusinessDays.field
			return 41356
		end
		def initialize(data = nil)
			if( data == nil )
				super(41356)
			else
				super(41356, data)
			end
		end
	end

	class LegCashSettlAmount < Quickfix::DoubleField
		def LegCashSettlAmount.field
			return 41357
		end
		def initialize(data = nil)
			if( data == nil )
				super(41357)
			else
				super(41357, data)
			end
		end
	end

	class LegCashSettlRecoveryFactor < Quickfix::DoubleField
		def LegCashSettlRecoveryFactor.field
			return 41358
		end
		def initialize(data = nil)
			if( data == nil )
				super(41358)
			else
				super(41358, data)
			end
		end
	end

	class LegCashSettlFixedTermIndicator < Quickfix::BoolField
		def LegCashSettlFixedTermIndicator.field
			return 41359
		end
		def initialize(data = nil)
			if( data == nil )
				super(41359)
			else
				super(41359, data)
			end
		end
	end

	class LegCashSettlAccruedInterestIndicator < Quickfix::BoolField
		def LegCashSettlAccruedInterestIndicator.field
			return 41360
		end
		def initialize(data = nil)
			if( data == nil )
				super(41360)
			else
				super(41360, data)
			end
		end
	end

	class LegCashSettlValuationMethod < Quickfix::IntField
		def LegCashSettlValuationMethod.field
			return 41361
		end
		def initialize(data = nil)
			if( data == nil )
				super(41361)
			else
				super(41361, data)
			end
		end
	end

	class LegCashSettlTermXID < Quickfix::StringField
		def LegCashSettlTermXID.field
			return 41362
		end
		def initialize(data = nil)
			if( data == nil )
				super(41362)
			else
				super(41362, data)
			end
		end
	end

	class NoLegComplexEventAveragingObservations < Quickfix::IntField
		def NoLegComplexEventAveragingObservations.field
			return 41363
		end
		def initialize(data = nil)
			if( data == nil )
				super(41363)
			else
				super(41363, data)
			end
		end
	end

	class LegComplexEventAveragingObservationNumber < Quickfix::IntField
		def LegComplexEventAveragingObservationNumber.field
			return 41364
		end
		def initialize(data = nil)
			if( data == nil )
				super(41364)
			else
				super(41364, data)
			end
		end
	end

	class LegComplexEventAveragingWeight < Quickfix::DoubleField
		def LegComplexEventAveragingWeight.field
			return 41365
		end
		def initialize(data = nil)
			if( data == nil )
				super(41365)
			else
				super(41365, data)
			end
		end
	end

	class NoLegComplexEventCreditEvents < Quickfix::IntField
		def NoLegComplexEventCreditEvents.field
			return 41366
		end
		def initialize(data = nil)
			if( data == nil )
				super(41366)
			else
				super(41366, data)
			end
		end
	end

	class LegComplexEventCreditEventType < Quickfix::StringField
		def LegComplexEventCreditEventType.field
			return 41367
		end
		def initialize(data = nil)
			if( data == nil )
				super(41367)
			else
				super(41367, data)
			end
		end
	end

	class LegComplexEventCreditEventValue < Quickfix::StringField
		def LegComplexEventCreditEventValue.field
			return 41368
		end
		def initialize(data = nil)
			if( data == nil )
				super(41368)
			else
				super(41368, data)
			end
		end
	end

	class LegComplexEventCreditEventCurrency < Quickfix::StringField
		def LegComplexEventCreditEventCurrency.field
			return 41369
		end
		def initialize(data = nil)
			if( data == nil )
				super(41369)
			else
				super(41369, data)
			end
		end
	end

	class LegComplexEventCreditEventPeriod < Quickfix::IntField
		def LegComplexEventCreditEventPeriod.field
			return 41370
		end
		def initialize(data = nil)
			if( data == nil )
				super(41370)
			else
				super(41370, data)
			end
		end
	end

	class LegComplexEventCreditEventUnit < Quickfix::StringField
		def LegComplexEventCreditEventUnit.field
			return 41371
		end
		def initialize(data = nil)
			if( data == nil )
				super(41371)
			else
				super(41371, data)
			end
		end
	end

	class LegComplexEventCreditEventDayType < Quickfix::IntField
		def LegComplexEventCreditEventDayType.field
			return 41372
		end
		def initialize(data = nil)
			if( data == nil )
				super(41372)
			else
				super(41372, data)
			end
		end
	end

	class LegComplexEventCreditEventRateSource < Quickfix::IntField
		def LegComplexEventCreditEventRateSource.field
			return 41373
		end
		def initialize(data = nil)
			if( data == nil )
				super(41373)
			else
				super(41373, data)
			end
		end
	end

	class NoLegComplexEventCreditEventQualifiers < Quickfix::IntField
		def NoLegComplexEventCreditEventQualifiers.field
			return 41374
		end
		def initialize(data = nil)
			if( data == nil )
				super(41374)
			else
				super(41374, data)
			end
		end
	end

	class LegComplexEventCreditEventQualifier < Quickfix::CharField
		def LegComplexEventCreditEventQualifier.field
			return 41375
		end
		def initialize(data = nil)
			if( data == nil )
				super(41375)
			else
				super(41375, data)
			end
		end
	end

	class NoLegComplexEventPeriodDateTimes < Quickfix::IntField
		def NoLegComplexEventPeriodDateTimes.field
			return 41376
		end
		def initialize(data = nil)
			if( data == nil )
				super(41376)
			else
				super(41376, data)
			end
		end
	end

	class LegComplexEventPeriodDate < Quickfix::StringField
		def LegComplexEventPeriodDate.field
			return 41377
		end
		def initialize(data = nil)
			if( data == nil )
				super(41377)
			else
				super(41377, data)
			end
		end
	end

	class LegComplexEventPeriodTime < Quickfix::StringField
		def LegComplexEventPeriodTime.field
			return 41378
		end
		def initialize(data = nil)
			if( data == nil )
				super(41378)
			else
				super(41378, data)
			end
		end
	end

	class NoLegComplexEventPeriods < Quickfix::IntField
		def NoLegComplexEventPeriods.field
			return 41379
		end
		def initialize(data = nil)
			if( data == nil )
				super(41379)
			else
				super(41379, data)
			end
		end
	end

	class LegComplexEventPeriodType < Quickfix::IntField
		def LegComplexEventPeriodType.field
			return 41380
		end
		def initialize(data = nil)
			if( data == nil )
				super(41380)
			else
				super(41380, data)
			end
		end
	end

	class LegComplexEventBusinessCenter < Quickfix::StringField
		def LegComplexEventBusinessCenter.field
			return 41381
		end
		def initialize(data = nil)
			if( data == nil )
				super(41381)
			else
				super(41381, data)
			end
		end
	end

	class NoLegComplexEventRateSources < Quickfix::IntField
		def NoLegComplexEventRateSources.field
			return 41382
		end
		def initialize(data = nil)
			if( data == nil )
				super(41382)
			else
				super(41382, data)
			end
		end
	end

	class LegComplexEventRateSource < Quickfix::IntField
		def LegComplexEventRateSource.field
			return 41383
		end
		def initialize(data = nil)
			if( data == nil )
				super(41383)
			else
				super(41383, data)
			end
		end
	end

	class LegComplexEventRateSourceType < Quickfix::IntField
		def LegComplexEventRateSourceType.field
			return 41384
		end
		def initialize(data = nil)
			if( data == nil )
				super(41384)
			else
				super(41384, data)
			end
		end
	end

	class LegComplexEventReferencePage < Quickfix::StringField
		def LegComplexEventReferencePage.field
			return 41385
		end
		def initialize(data = nil)
			if( data == nil )
				super(41385)
			else
				super(41385, data)
			end
		end
	end

	class LegComplexEvenReferencePageHeading < Quickfix::StringField
		def LegComplexEvenReferencePageHeading.field
			return 41386
		end
		def initialize(data = nil)
			if( data == nil )
				super(41386)
			else
				super(41386, data)
			end
		end
	end

	class NoLegComplexEventDateBusinessCenters < Quickfix::IntField
		def NoLegComplexEventDateBusinessCenters.field
			return 41387
		end
		def initialize(data = nil)
			if( data == nil )
				super(41387)
			else
				super(41387, data)
			end
		end
	end

	class LegComplexEventDateBusinessCenter < Quickfix::StringField
		def LegComplexEventDateBusinessCenter.field
			return 41388
		end
		def initialize(data = nil)
			if( data == nil )
				super(41388)
			else
				super(41388, data)
			end
		end
	end

	class LegComplexEventDateUnadjusted < Quickfix::StringField
		def LegComplexEventDateUnadjusted.field
			return 41389
		end
		def initialize(data = nil)
			if( data == nil )
				super(41389)
			else
				super(41389, data)
			end
		end
	end

	class LegComplexEventDateRelativeTo < Quickfix::IntField
		def LegComplexEventDateRelativeTo.field
			return 41390
		end
		def initialize(data = nil)
			if( data == nil )
				super(41390)
			else
				super(41390, data)
			end
		end
	end

	class LegComplexEventDateOffsetPeriod < Quickfix::IntField
		def LegComplexEventDateOffsetPeriod.field
			return 41391
		end
		def initialize(data = nil)
			if( data == nil )
				super(41391)
			else
				super(41391, data)
			end
		end
	end

	class LegComplexEventDateOffsetUnit < Quickfix::StringField
		def LegComplexEventDateOffsetUnit.field
			return 41392
		end
		def initialize(data = nil)
			if( data == nil )
				super(41392)
			else
				super(41392, data)
			end
		end
	end

	class LegComplexEventDateOffsetDayType < Quickfix::IntField
		def LegComplexEventDateOffsetDayType.field
			return 41393
		end
		def initialize(data = nil)
			if( data == nil )
				super(41393)
			else
				super(41393, data)
			end
		end
	end

	class LegComplexEventDateBusinessDayConvention < Quickfix::IntField
		def LegComplexEventDateBusinessDayConvention.field
			return 41394
		end
		def initialize(data = nil)
			if( data == nil )
				super(41394)
			else
				super(41394, data)
			end
		end
	end

	class LegComplexEventDateAdjusted < Quickfix::StringField
		def LegComplexEventDateAdjusted.field
			return 41395
		end
		def initialize(data = nil)
			if( data == nil )
				super(41395)
			else
				super(41395, data)
			end
		end
	end

	class LegComplexEventFixingTime < Quickfix::StringField
		def LegComplexEventFixingTime.field
			return 41396
		end
		def initialize(data = nil)
			if( data == nil )
				super(41396)
			else
				super(41396, data)
			end
		end
	end

	class LegComplexEventFixingTimeBusinessCenter < Quickfix::StringField
		def LegComplexEventFixingTimeBusinessCenter.field
			return 41397
		end
		def initialize(data = nil)
			if( data == nil )
				super(41397)
			else
				super(41397, data)
			end
		end
	end

	class NoLegComplexEventCreditEventSources < Quickfix::IntField
		def NoLegComplexEventCreditEventSources.field
			return 41398
		end
		def initialize(data = nil)
			if( data == nil )
				super(41398)
			else
				super(41398, data)
			end
		end
	end

	class LegComplexEventCreditEventSource < Quickfix::StringField
		def LegComplexEventCreditEventSource.field
			return 41399
		end
		def initialize(data = nil)
			if( data == nil )
				super(41399)
			else
				super(41399, data)
			end
		end
	end

	class NoLegComplexEvents < Quickfix::IntField
		def NoLegComplexEvents.field
			return 2218
		end
		def initialize(data = nil)
			if( data == nil )
				super(2218)
			else
				super(2218, data)
			end
		end
	end

	class LegComplexEventType < Quickfix::IntField
		def LegComplexEventType.field
			return 2219
		end
		def initialize(data = nil)
			if( data == nil )
				super(2219)
			else
				super(2219, data)
			end
		end
	end

	class LegComplexOptPayoutPaySide < Quickfix::IntField
		def LegComplexOptPayoutPaySide.field
			return 2220
		end
		def initialize(data = nil)
			if( data == nil )
				super(2220)
			else
				super(2220, data)
			end
		end
	end

	class LegComplexOptPayoutReceiveSide < Quickfix::IntField
		def LegComplexOptPayoutReceiveSide.field
			return 2221
		end
		def initialize(data = nil)
			if( data == nil )
				super(2221)
			else
				super(2221, data)
			end
		end
	end

	class LegComplexOptPayoutUnderlier < Quickfix::StringField
		def LegComplexOptPayoutUnderlier.field
			return 2222
		end
		def initialize(data = nil)
			if( data == nil )
				super(2222)
			else
				super(2222, data)
			end
		end
	end

	class LegComplexOptPayoutAmount < Quickfix::DoubleField
		def LegComplexOptPayoutAmount.field
			return 2223
		end
		def initialize(data = nil)
			if( data == nil )
				super(2223)
			else
				super(2223, data)
			end
		end
	end

	class LegComplexOptPayoutPercentage < Quickfix::DoubleField
		def LegComplexOptPayoutPercentage.field
			return 2224
		end
		def initialize(data = nil)
			if( data == nil )
				super(2224)
			else
				super(2224, data)
			end
		end
	end

	class LegComplexOptPayoutTime < Quickfix::IntField
		def LegComplexOptPayoutTime.field
			return 2225
		end
		def initialize(data = nil)
			if( data == nil )
				super(2225)
			else
				super(2225, data)
			end
		end
	end

	class LegComplexOptPayoutCurrency < Quickfix::StringField
		def LegComplexOptPayoutCurrency.field
			return 2226
		end
		def initialize(data = nil)
			if( data == nil )
				super(2226)
			else
				super(2226, data)
			end
		end
	end

	class LegComplexEventPrice < Quickfix::DoubleField
		def LegComplexEventPrice.field
			return 2227
		end
		def initialize(data = nil)
			if( data == nil )
				super(2227)
			else
				super(2227, data)
			end
		end
	end

	class LegComplexEventPricePercentage < Quickfix::DoubleField
		def LegComplexEventPricePercentage.field
			return 2228
		end
		def initialize(data = nil)
			if( data == nil )
				super(2228)
			else
				super(2228, data)
			end
		end
	end

	class LegComplexEventPriceBoundaryMethod < Quickfix::IntField
		def LegComplexEventPriceBoundaryMethod.field
			return 2229
		end
		def initialize(data = nil)
			if( data == nil )
				super(2229)
			else
				super(2229, data)
			end
		end
	end

	class LegComplexEventPriceBoundaryPrecision < Quickfix::DoubleField
		def LegComplexEventPriceBoundaryPrecision.field
			return 2230
		end
		def initialize(data = nil)
			if( data == nil )
				super(2230)
			else
				super(2230, data)
			end
		end
	end

	class LegComplexEventPriceTimeType < Quickfix::IntField
		def LegComplexEventPriceTimeType.field
			return 2231
		end
		def initialize(data = nil)
			if( data == nil )
				super(2231)
			else
				super(2231, data)
			end
		end
	end

	class LegComplexEventCondition < Quickfix::IntField
		def LegComplexEventCondition.field
			return 2232
		end
		def initialize(data = nil)
			if( data == nil )
				super(2232)
			else
				super(2232, data)
			end
		end
	end

	class LegComplexEventCurrencyOne < Quickfix::StringField
		def LegComplexEventCurrencyOne.field
			return 2233
		end
		def initialize(data = nil)
			if( data == nil )
				super(2233)
			else
				super(2233, data)
			end
		end
	end

	class LegComplexEventCurrencyTwo < Quickfix::StringField
		def LegComplexEventCurrencyTwo.field
			return 2234
		end
		def initialize(data = nil)
			if( data == nil )
				super(2234)
			else
				super(2234, data)
			end
		end
	end

	class LegComplexEventQuoteBasis < Quickfix::IntField
		def LegComplexEventQuoteBasis.field
			return 2235
		end
		def initialize(data = nil)
			if( data == nil )
				super(2235)
			else
				super(2235, data)
			end
		end
	end

	class LegComplexEventFixedFXRate < Quickfix::DoubleField
		def LegComplexEventFixedFXRate.field
			return 2236
		end
		def initialize(data = nil)
			if( data == nil )
				super(2236)
			else
				super(2236, data)
			end
		end
	end

	class LegComplexEventDeterminationMethod < Quickfix::StringField
		def LegComplexEventDeterminationMethod.field
			return 2237
		end
		def initialize(data = nil)
			if( data == nil )
				super(2237)
			else
				super(2237, data)
			end
		end
	end

	class LegComplexEventCalculationAgent < Quickfix::IntField
		def LegComplexEventCalculationAgent.field
			return 2238
		end
		def initialize(data = nil)
			if( data == nil )
				super(2238)
			else
				super(2238, data)
			end
		end
	end

	class LegComplexEventStrikePrice < Quickfix::DoubleField
		def LegComplexEventStrikePrice.field
			return 2239
		end
		def initialize(data = nil)
			if( data == nil )
				super(2239)
			else
				super(2239, data)
			end
		end
	end

	class LegComplexEventStrikeFactor < Quickfix::DoubleField
		def LegComplexEventStrikeFactor.field
			return 2240
		end
		def initialize(data = nil)
			if( data == nil )
				super(2240)
			else
				super(2240, data)
			end
		end
	end

	class LegComplexEventStrikeNumberOfOptions < Quickfix::IntField
		def LegComplexEventStrikeNumberOfOptions.field
			return 2241
		end
		def initialize(data = nil)
			if( data == nil )
				super(2241)
			else
				super(2241, data)
			end
		end
	end

	class LegComplexEventCreditEventsXIDRef < Quickfix::StringField
		def LegComplexEventCreditEventsXIDRef.field
			return 2242
		end
		def initialize(data = nil)
			if( data == nil )
				super(2242)
			else
				super(2242, data)
			end
		end
	end

	class LegComplexEventCreditEventNotifyingParty < Quickfix::IntField
		def LegComplexEventCreditEventNotifyingParty.field
			return 2243
		end
		def initialize(data = nil)
			if( data == nil )
				super(2243)
			else
				super(2243, data)
			end
		end
	end

	class LegComplexEventCreditEventBusinessCenter < Quickfix::StringField
		def LegComplexEventCreditEventBusinessCenter.field
			return 2244
		end
		def initialize(data = nil)
			if( data == nil )
				super(2244)
			else
				super(2244, data)
			end
		end
	end

	class LegComplexEventCreditEventStandardSources < Quickfix::BoolField
		def LegComplexEventCreditEventStandardSources.field
			return 2245
		end
		def initialize(data = nil)
			if( data == nil )
				super(2245)
			else
				super(2245, data)
			end
		end
	end

	class LegComplexEventCreditEventMinimumSources < Quickfix::IntField
		def LegComplexEventCreditEventMinimumSources.field
			return 2246
		end
		def initialize(data = nil)
			if( data == nil )
				super(2246)
			else
				super(2246, data)
			end
		end
	end

	class LegComplexEventXID < Quickfix::StringField
		def LegComplexEventXID.field
			return 2248
		end
		def initialize(data = nil)
			if( data == nil )
				super(2248)
			else
				super(2248, data)
			end
		end
	end

	class LegComplexEventXIDRef < Quickfix::StringField
		def LegComplexEventXIDRef.field
			return 2249
		end
		def initialize(data = nil)
			if( data == nil )
				super(2249)
			else
				super(2249, data)
			end
		end
	end

	class NoLegComplexEventDates < Quickfix::IntField
		def NoLegComplexEventDates.field
			return 2250
		end
		def initialize(data = nil)
			if( data == nil )
				super(2250)
			else
				super(2250, data)
			end
		end
	end

	class LegComplexEventStartDate < Quickfix::UtcDateField
		def LegComplexEventStartDate.field
			return 2251
		end
		def initialize(data = nil)
			if( data == nil )
				super(2251)
			else
				super(2251, data)
			end
		end
	end

	class LegComplexEventEndDate < Quickfix::UtcDateField
		def LegComplexEventEndDate.field
			return 2252
		end
		def initialize(data = nil)
			if( data == nil )
				super(2252)
			else
				super(2252, data)
			end
		end
	end

	class NoLegComplexEventTimes < Quickfix::IntField
		def NoLegComplexEventTimes.field
			return 2253
		end
		def initialize(data = nil)
			if( data == nil )
				super(2253)
			else
				super(2253, data)
			end
		end
	end

	class LegComplexEventStartTime < Quickfix::UtcTimeOnlyField
		def LegComplexEventStartTime.field
			return 2204
		end
		def initialize(data = nil)
			if( data == nil )
				super(2204)
			else
				super(2204, data)
			end
		end
	end

	class LegComplexEventEndTime < Quickfix::UtcTimeOnlyField
		def LegComplexEventEndTime.field
			return 2247
		end
		def initialize(data = nil)
			if( data == nil )
				super(2247)
			else
				super(2247, data)
			end
		end
	end

	class NoLegComplexEventSchedules < Quickfix::IntField
		def NoLegComplexEventSchedules.field
			return 41400
		end
		def initialize(data = nil)
			if( data == nil )
				super(41400)
			else
				super(41400, data)
			end
		end
	end

	class LegComplexEventScheduleStartDate < Quickfix::StringField
		def LegComplexEventScheduleStartDate.field
			return 41401
		end
		def initialize(data = nil)
			if( data == nil )
				super(41401)
			else
				super(41401, data)
			end
		end
	end

	class LegComplexEventScheduleEndDate < Quickfix::StringField
		def LegComplexEventScheduleEndDate.field
			return 41402
		end
		def initialize(data = nil)
			if( data == nil )
				super(41402)
			else
				super(41402, data)
			end
		end
	end

	class LegComplexEventScheduleFrequencyPeriod < Quickfix::IntField
		def LegComplexEventScheduleFrequencyPeriod.field
			return 41403
		end
		def initialize(data = nil)
			if( data == nil )
				super(41403)
			else
				super(41403, data)
			end
		end
	end

	class LegComplexEventScheduleFrequencyUnit < Quickfix::StringField
		def LegComplexEventScheduleFrequencyUnit.field
			return 41404
		end
		def initialize(data = nil)
			if( data == nil )
				super(41404)
			else
				super(41404, data)
			end
		end
	end

	class LegComplexEventScheduleRollConvention < Quickfix::StringField
		def LegComplexEventScheduleRollConvention.field
			return 41405
		end
		def initialize(data = nil)
			if( data == nil )
				super(41405)
			else
				super(41405, data)
			end
		end
	end

	class NoLegDeliverySchedules < Quickfix::IntField
		def NoLegDeliverySchedules.field
			return 41408
		end
		def initialize(data = nil)
			if( data == nil )
				super(41408)
			else
				super(41408, data)
			end
		end
	end

	class LegDeliveryScheduleType < Quickfix::IntField
		def LegDeliveryScheduleType.field
			return 41409
		end
		def initialize(data = nil)
			if( data == nil )
				super(41409)
			else
				super(41409, data)
			end
		end
	end

	class LegDeliveryScheduleXID < Quickfix::StringField
		def LegDeliveryScheduleXID.field
			return 41410
		end
		def initialize(data = nil)
			if( data == nil )
				super(41410)
			else
				super(41410, data)
			end
		end
	end

	class LegDeliveryScheduleNotional < Quickfix::DoubleField
		def LegDeliveryScheduleNotional.field
			return 41411
		end
		def initialize(data = nil)
			if( data == nil )
				super(41411)
			else
				super(41411, data)
			end
		end
	end

	class LegDeliveryScheduleNotionalUnitOfMeasure < Quickfix::StringField
		def LegDeliveryScheduleNotionalUnitOfMeasure.field
			return 41412
		end
		def initialize(data = nil)
			if( data == nil )
				super(41412)
			else
				super(41412, data)
			end
		end
	end

	class LegDeliveryScheduleNotionalCommodityFrequency < Quickfix::IntField
		def LegDeliveryScheduleNotionalCommodityFrequency.field
			return 41413
		end
		def initialize(data = nil)
			if( data == nil )
				super(41413)
			else
				super(41413, data)
			end
		end
	end

	class LegDeliveryScheduleNegativeTolerance < Quickfix::DoubleField
		def LegDeliveryScheduleNegativeTolerance.field
			return 41414
		end
		def initialize(data = nil)
			if( data == nil )
				super(41414)
			else
				super(41414, data)
			end
		end
	end

	class LegDeliverySchedulePositiveTolerance < Quickfix::DoubleField
		def LegDeliverySchedulePositiveTolerance.field
			return 41415
		end
		def initialize(data = nil)
			if( data == nil )
				super(41415)
			else
				super(41415, data)
			end
		end
	end

	class LegDeliveryScheduleToleranceUnitOfMeasure < Quickfix::StringField
		def LegDeliveryScheduleToleranceUnitOfMeasure.field
			return 41416
		end
		def initialize(data = nil)
			if( data == nil )
				super(41416)
			else
				super(41416, data)
			end
		end
	end

	class LegDeliveryScheduleToleranceType < Quickfix::IntField
		def LegDeliveryScheduleToleranceType.field
			return 41417
		end
		def initialize(data = nil)
			if( data == nil )
				super(41417)
			else
				super(41417, data)
			end
		end
	end

	class LegDeliveryScheduleSettlCountry < Quickfix::StringField
		def LegDeliveryScheduleSettlCountry.field
			return 41418
		end
		def initialize(data = nil)
			if( data == nil )
				super(41418)
			else
				super(41418, data)
			end
		end
	end

	class LegDeliveryScheduleSettlTimeZone < Quickfix::StringField
		def LegDeliveryScheduleSettlTimeZone.field
			return 41419
		end
		def initialize(data = nil)
			if( data == nil )
				super(41419)
			else
				super(41419, data)
			end
		end
	end

	class LegDeliveryScheduleSettlFlowType < Quickfix::IntField
		def LegDeliveryScheduleSettlFlowType.field
			return 41420
		end
		def initialize(data = nil)
			if( data == nil )
				super(41420)
			else
				super(41420, data)
			end
		end
	end

	class LegDeliveryScheduleSettlHolidaysProcessingInstruction < Quickfix::IntField
		def LegDeliveryScheduleSettlHolidaysProcessingInstruction.field
			return 41421
		end
		def initialize(data = nil)
			if( data == nil )
				super(41421)
			else
				super(41421, data)
			end
		end
	end

	class NoLegDeliveryScheduleSettlDays < Quickfix::IntField
		def NoLegDeliveryScheduleSettlDays.field
			return 41422
		end
		def initialize(data = nil)
			if( data == nil )
				super(41422)
			else
				super(41422, data)
			end
		end
	end

	class LegDeliveryScheduleSettlDay < Quickfix::IntField
		def LegDeliveryScheduleSettlDay.field
			return 41423
		end
		def initialize(data = nil)
			if( data == nil )
				super(41423)
			else
				super(41423, data)
			end
		end
	end

	class LegDeliveryScheduleSettlTotalHours < Quickfix::IntField
		def LegDeliveryScheduleSettlTotalHours.field
			return 41424
		end
		def initialize(data = nil)
			if( data == nil )
				super(41424)
			else
				super(41424, data)
			end
		end
	end

	class NoLegDeliveryScheduleSettlTimes < Quickfix::IntField
		def NoLegDeliveryScheduleSettlTimes.field
			return 41425
		end
		def initialize(data = nil)
			if( data == nil )
				super(41425)
			else
				super(41425, data)
			end
		end
	end

	class LegDeliveryScheduleSettlStart < Quickfix::StringField
		def LegDeliveryScheduleSettlStart.field
			return 41426
		end
		def initialize(data = nil)
			if( data == nil )
				super(41426)
			else
				super(41426, data)
			end
		end
	end

	class LegDeliveryScheduleSettlEnd < Quickfix::StringField
		def LegDeliveryScheduleSettlEnd.field
			return 41427
		end
		def initialize(data = nil)
			if( data == nil )
				super(41427)
			else
				super(41427, data)
			end
		end
	end

	class LegDeliveryScheduleSettlTimeType < Quickfix::IntField
		def LegDeliveryScheduleSettlTimeType.field
			return 41428
		end
		def initialize(data = nil)
			if( data == nil )
				super(41428)
			else
				super(41428, data)
			end
		end
	end

	class LegDeliveryStreamType < Quickfix::IntField
		def LegDeliveryStreamType.field
			return 41429
		end
		def initialize(data = nil)
			if( data == nil )
				super(41429)
			else
				super(41429, data)
			end
		end
	end

	class LegDeliveryStreamPipeline < Quickfix::StringField
		def LegDeliveryStreamPipeline.field
			return 41430
		end
		def initialize(data = nil)
			if( data == nil )
				super(41430)
			else
				super(41430, data)
			end
		end
	end

	class LegDeliveryStreamEntryPoint < Quickfix::StringField
		def LegDeliveryStreamEntryPoint.field
			return 41431
		end
		def initialize(data = nil)
			if( data == nil )
				super(41431)
			else
				super(41431, data)
			end
		end
	end

	class LegDeliveryStreamWithdrawalPoint < Quickfix::StringField
		def LegDeliveryStreamWithdrawalPoint.field
			return 41432
		end
		def initialize(data = nil)
			if( data == nil )
				super(41432)
			else
				super(41432, data)
			end
		end
	end

	class LegDeliveryStreamDeliveryPoint < Quickfix::StringField
		def LegDeliveryStreamDeliveryPoint.field
			return 41433
		end
		def initialize(data = nil)
			if( data == nil )
				super(41433)
			else
				super(41433, data)
			end
		end
	end

	class LegDeliveryStreamDeliveryRestriction < Quickfix::IntField
		def LegDeliveryStreamDeliveryRestriction.field
			return 41434
		end
		def initialize(data = nil)
			if( data == nil )
				super(41434)
			else
				super(41434, data)
			end
		end
	end

	class LegDeliveryStreamDeliveryContingency < Quickfix::StringField
		def LegDeliveryStreamDeliveryContingency.field
			return 41435
		end
		def initialize(data = nil)
			if( data == nil )
				super(41435)
			else
				super(41435, data)
			end
		end
	end

	class LegDeliveryStreamDeliveryContingentPartySide < Quickfix::IntField
		def LegDeliveryStreamDeliveryContingentPartySide.field
			return 41436
		end
		def initialize(data = nil)
			if( data == nil )
				super(41436)
			else
				super(41436, data)
			end
		end
	end

	class LegDeliveryStreamDeliverAtSourceIndicator < Quickfix::BoolField
		def LegDeliveryStreamDeliverAtSourceIndicator.field
			return 41437
		end
		def initialize(data = nil)
			if( data == nil )
				super(41437)
			else
				super(41437, data)
			end
		end
	end

	class LegDeliveryStreamRiskApportionment < Quickfix::StringField
		def LegDeliveryStreamRiskApportionment.field
			return 41438
		end
		def initialize(data = nil)
			if( data == nil )
				super(41438)
			else
				super(41438, data)
			end
		end
	end

	class LegDeliveryStreamRiskApportionmentSource < Quickfix::StringField
		def LegDeliveryStreamRiskApportionmentSource.field
			return 41219
		end
		def initialize(data = nil)
			if( data == nil )
				super(41219)
			else
				super(41219, data)
			end
		end
	end

	class LegDeliveryStreamTitleTransferLocation < Quickfix::StringField
		def LegDeliveryStreamTitleTransferLocation.field
			return 41439
		end
		def initialize(data = nil)
			if( data == nil )
				super(41439)
			else
				super(41439, data)
			end
		end
	end

	class LegDeliveryStreamTitleTransferCondition < Quickfix::IntField
		def LegDeliveryStreamTitleTransferCondition.field
			return 41440
		end
		def initialize(data = nil)
			if( data == nil )
				super(41440)
			else
				super(41440, data)
			end
		end
	end

	class LegDeliveryStreamImporterOfRecord < Quickfix::StringField
		def LegDeliveryStreamImporterOfRecord.field
			return 41441
		end
		def initialize(data = nil)
			if( data == nil )
				super(41441)
			else
				super(41441, data)
			end
		end
	end

	class LegDeliveryStreamNegativeTolerance < Quickfix::DoubleField
		def LegDeliveryStreamNegativeTolerance.field
			return 41442
		end
		def initialize(data = nil)
			if( data == nil )
				super(41442)
			else
				super(41442, data)
			end
		end
	end

	class LegDeliveryStreamPositiveTolerance < Quickfix::DoubleField
		def LegDeliveryStreamPositiveTolerance.field
			return 41443
		end
		def initialize(data = nil)
			if( data == nil )
				super(41443)
			else
				super(41443, data)
			end
		end
	end

	class LegDeliveryStreamToleranceUnitOfMeasure < Quickfix::StringField
		def LegDeliveryStreamToleranceUnitOfMeasure.field
			return 41444
		end
		def initialize(data = nil)
			if( data == nil )
				super(41444)
			else
				super(41444, data)
			end
		end
	end

	class LegDeliveryStreamToleranceType < Quickfix::IntField
		def LegDeliveryStreamToleranceType.field
			return 41445
		end
		def initialize(data = nil)
			if( data == nil )
				super(41445)
			else
				super(41445, data)
			end
		end
	end

	class LegDeliveryStreamToleranceOptionSide < Quickfix::IntField
		def LegDeliveryStreamToleranceOptionSide.field
			return 41446
		end
		def initialize(data = nil)
			if( data == nil )
				super(41446)
			else
				super(41446, data)
			end
		end
	end

	class LegDeliveryStreamTotalPositiveTolerance < Quickfix::DoubleField
		def LegDeliveryStreamTotalPositiveTolerance.field
			return 41447
		end
		def initialize(data = nil)
			if( data == nil )
				super(41447)
			else
				super(41447, data)
			end
		end
	end

	class LegDeliveryStreamTotalNegativeTolerance < Quickfix::DoubleField
		def LegDeliveryStreamTotalNegativeTolerance.field
			return 41448
		end
		def initialize(data = nil)
			if( data == nil )
				super(41448)
			else
				super(41448, data)
			end
		end
	end

	class LegDeliveryStreamNotionalConversionFactor < Quickfix::DoubleField
		def LegDeliveryStreamNotionalConversionFactor.field
			return 41449
		end
		def initialize(data = nil)
			if( data == nil )
				super(41449)
			else
				super(41449, data)
			end
		end
	end

	class LegDeliveryStreamTransportEquipment < Quickfix::StringField
		def LegDeliveryStreamTransportEquipment.field
			return 41450
		end
		def initialize(data = nil)
			if( data == nil )
				super(41450)
			else
				super(41450, data)
			end
		end
	end

	class LegDeliveryStreamElectingPartySide < Quickfix::IntField
		def LegDeliveryStreamElectingPartySide.field
			return 41451
		end
		def initialize(data = nil)
			if( data == nil )
				super(41451)
			else
				super(41451, data)
			end
		end
	end

	class NoLegStreamAssetAttributes < Quickfix::IntField
		def NoLegStreamAssetAttributes.field
			return 41452
		end
		def initialize(data = nil)
			if( data == nil )
				super(41452)
			else
				super(41452, data)
			end
		end
	end

	class LegStreamAssetAttributeType < Quickfix::StringField
		def LegStreamAssetAttributeType.field
			return 41453
		end
		def initialize(data = nil)
			if( data == nil )
				super(41453)
			else
				super(41453, data)
			end
		end
	end

	class LegStreamAssetAttributeValue < Quickfix::StringField
		def LegStreamAssetAttributeValue.field
			return 41454
		end
		def initialize(data = nil)
			if( data == nil )
				super(41454)
			else
				super(41454, data)
			end
		end
	end

	class LegStreamAssetAttributeLimit < Quickfix::StringField
		def LegStreamAssetAttributeLimit.field
			return 41455
		end
		def initialize(data = nil)
			if( data == nil )
				super(41455)
			else
				super(41455, data)
			end
		end
	end

	class NoLegDeliveryStreamCycles < Quickfix::IntField
		def NoLegDeliveryStreamCycles.field
			return 41456
		end
		def initialize(data = nil)
			if( data == nil )
				super(41456)
			else
				super(41456, data)
			end
		end
	end

	class LegDeliveryStreamCycleDesc < Quickfix::StringField
		def LegDeliveryStreamCycleDesc.field
			return 41457
		end
		def initialize(data = nil)
			if( data == nil )
				super(41457)
			else
				super(41457, data)
			end
		end
	end

	class EncodedLegDeliveryStreamCycleDescLen < Quickfix::IntField
		def EncodedLegDeliveryStreamCycleDescLen.field
			return 41458
		end
		def initialize(data = nil)
			if( data == nil )
				super(41458)
			else
				super(41458, data)
			end
		end
	end

	class EncodedLegDeliveryStreamCycleDesc < Quickfix::StringField
		def EncodedLegDeliveryStreamCycleDesc.field
			return 41459
		end
		def initialize(data = nil)
			if( data == nil )
				super(41459)
			else
				super(41459, data)
			end
		end
	end

	class NoLegDeliveryStreamCommoditySources < Quickfix::IntField
		def NoLegDeliveryStreamCommoditySources.field
			return 41460
		end
		def initialize(data = nil)
			if( data == nil )
				super(41460)
			else
				super(41460, data)
			end
		end
	end

	class LegDeliveryStreamCommoditySource < Quickfix::StringField
		def LegDeliveryStreamCommoditySource.field
			return 41461
		end
		def initialize(data = nil)
			if( data == nil )
				super(41461)
			else
				super(41461, data)
			end
		end
	end

	class NoLegInstrumentParties < Quickfix::IntField
		def NoLegInstrumentParties.field
			return 2254
		end
		def initialize(data = nil)
			if( data == nil )
				super(2254)
			else
				super(2254, data)
			end
		end
	end

	class LegInstrumentPartyID < Quickfix::StringField
		def LegInstrumentPartyID.field
			return 2255
		end
		def initialize(data = nil)
			if( data == nil )
				super(2255)
			else
				super(2255, data)
			end
		end
	end

	class LegInstrumentPartyIDSource < Quickfix::CharField
		def LegInstrumentPartyIDSource.field
			return 2256
		end
		def initialize(data = nil)
			if( data == nil )
				super(2256)
			else
				super(2256, data)
			end
		end
	end

	class LegInstrumentPartyRole < Quickfix::IntField
		def LegInstrumentPartyRole.field
			return 2257
		end
		def initialize(data = nil)
			if( data == nil )
				super(2257)
			else
				super(2257, data)
			end
		end
	end

	class NoLegInstrumentPartySubIDs < Quickfix::IntField
		def NoLegInstrumentPartySubIDs.field
			return 2258
		end
		def initialize(data = nil)
			if( data == nil )
				super(2258)
			else
				super(2258, data)
			end
		end
	end

	class LegInstrumentPartySubID < Quickfix::StringField
		def LegInstrumentPartySubID.field
			return 2259
		end
		def initialize(data = nil)
			if( data == nil )
				super(2259)
			else
				super(2259, data)
			end
		end
	end

	class LegInstrumentPartySubIDType < Quickfix::IntField
		def LegInstrumentPartySubIDType.field
			return 2260
		end
		def initialize(data = nil)
			if( data == nil )
				super(2260)
			else
				super(2260, data)
			end
		end
	end

	class LegMarketDisruptionProvision < Quickfix::IntField
		def LegMarketDisruptionProvision.field
			return 41462
		end
		def initialize(data = nil)
			if( data == nil )
				super(41462)
			else
				super(41462, data)
			end
		end
	end

	class LegMarketDisruptionFallbackProvision < Quickfix::IntField
		def LegMarketDisruptionFallbackProvision.field
			return 41463
		end
		def initialize(data = nil)
			if( data == nil )
				super(41463)
			else
				super(41463, data)
			end
		end
	end

	class LegMarketDisruptionMaximumDays < Quickfix::IntField
		def LegMarketDisruptionMaximumDays.field
			return 41464
		end
		def initialize(data = nil)
			if( data == nil )
				super(41464)
			else
				super(41464, data)
			end
		end
	end

	class LegMarketDisruptionMaterialityPercentage < Quickfix::DoubleField
		def LegMarketDisruptionMaterialityPercentage.field
			return 41465
		end
		def initialize(data = nil)
			if( data == nil )
				super(41465)
			else
				super(41465, data)
			end
		end
	end

	class LegMarketDisruptionMinimumFuturesContracts < Quickfix::IntField
		def LegMarketDisruptionMinimumFuturesContracts.field
			return 41466
		end
		def initialize(data = nil)
			if( data == nil )
				super(41466)
			else
				super(41466, data)
			end
		end
	end

	class NoLegMarketDisruptionEvents < Quickfix::IntField
		def NoLegMarketDisruptionEvents.field
			return 41467
		end
		def initialize(data = nil)
			if( data == nil )
				super(41467)
			else
				super(41467, data)
			end
		end
	end

	class LegMarketDisruptionEvent < Quickfix::StringField
		def LegMarketDisruptionEvent.field
			return 41468
		end
		def initialize(data = nil)
			if( data == nil )
				super(41468)
			else
				super(41468, data)
			end
		end
	end

	class NoLegMarketDisruptionFallbacks < Quickfix::IntField
		def NoLegMarketDisruptionFallbacks.field
			return 41469
		end
		def initialize(data = nil)
			if( data == nil )
				super(41469)
			else
				super(41469, data)
			end
		end
	end

	class LegMarketDisruptionFallbackType < Quickfix::StringField
		def LegMarketDisruptionFallbackType.field
			return 41470
		end
		def initialize(data = nil)
			if( data == nil )
				super(41470)
			else
				super(41470, data)
			end
		end
	end

	class NoLegMarketDisruptionFallbackReferencePrices < Quickfix::IntField
		def NoLegMarketDisruptionFallbackReferencePrices.field
			return 41471
		end
		def initialize(data = nil)
			if( data == nil )
				super(41471)
			else
				super(41471, data)
			end
		end
	end

	class LegMarketDisruptionFallbackUnderlierType < Quickfix::IntField
		def LegMarketDisruptionFallbackUnderlierType.field
			return 41472
		end
		def initialize(data = nil)
			if( data == nil )
				super(41472)
			else
				super(41472, data)
			end
		end
	end

	class LegMarketDisruptionFallbackUnderlierSecurityID < Quickfix::StringField
		def LegMarketDisruptionFallbackUnderlierSecurityID.field
			return 41473
		end
		def initialize(data = nil)
			if( data == nil )
				super(41473)
			else
				super(41473, data)
			end
		end
	end

	class LegMarketDisruptionFallbackUnderlierSecurityIDSource < Quickfix::StringField
		def LegMarketDisruptionFallbackUnderlierSecurityIDSource.field
			return 41474
		end
		def initialize(data = nil)
			if( data == nil )
				super(41474)
			else
				super(41474, data)
			end
		end
	end

	class LegMarketDisruptionFallbackUnderlierSecurityDesc < Quickfix::StringField
		def LegMarketDisruptionFallbackUnderlierSecurityDesc.field
			return 41475
		end
		def initialize(data = nil)
			if( data == nil )
				super(41475)
			else
				super(41475, data)
			end
		end
	end

	class EncodedLegMarketDisruptionFallbackUnderlierSecurityDescLen < Quickfix::IntField
		def EncodedLegMarketDisruptionFallbackUnderlierSecurityDescLen.field
			return 41476
		end
		def initialize(data = nil)
			if( data == nil )
				super(41476)
			else
				super(41476, data)
			end
		end
	end

	class EncodedLegMarketDisruptionFallbackUnderlierSecurityDesc < Quickfix::StringField
		def EncodedLegMarketDisruptionFallbackUnderlierSecurityDesc.field
			return 41477
		end
		def initialize(data = nil)
			if( data == nil )
				super(41477)
			else
				super(41477, data)
			end
		end
	end

	class LegMarketDisruptionFallbackOpenUnits < Quickfix::DoubleField
		def LegMarketDisruptionFallbackOpenUnits.field
			return 41478
		end
		def initialize(data = nil)
			if( data == nil )
				super(41478)
			else
				super(41478, data)
			end
		end
	end

	class LegMarketDisruptionFallbackBasketCurrency < Quickfix::StringField
		def LegMarketDisruptionFallbackBasketCurrency.field
			return 41479
		end
		def initialize(data = nil)
			if( data == nil )
				super(41479)
			else
				super(41479, data)
			end
		end
	end

	class LegMarketDisruptionFallbackBasketDivisor < Quickfix::DoubleField
		def LegMarketDisruptionFallbackBasketDivisor.field
			return 41480
		end
		def initialize(data = nil)
			if( data == nil )
				super(41480)
			else
				super(41480, data)
			end
		end
	end

	class LegExerciseDesc < Quickfix::StringField
		def LegExerciseDesc.field
			return 41481
		end
		def initialize(data = nil)
			if( data == nil )
				super(41481)
			else
				super(41481, data)
			end
		end
	end

	class EncodedLegExerciseDescLen < Quickfix::IntField
		def EncodedLegExerciseDescLen.field
			return 41482
		end
		def initialize(data = nil)
			if( data == nil )
				super(41482)
			else
				super(41482, data)
			end
		end
	end

	class EncodedLegExerciseDesc < Quickfix::StringField
		def EncodedLegExerciseDesc.field
			return 41483
		end
		def initialize(data = nil)
			if( data == nil )
				super(41483)
			else
				super(41483, data)
			end
		end
	end

	class LegAutomaticExerciseIndicator < Quickfix::BoolField
		def LegAutomaticExerciseIndicator.field
			return 41484
		end
		def initialize(data = nil)
			if( data == nil )
				super(41484)
			else
				super(41484, data)
			end
		end
	end

	class LegAutomaticExerciseThresholdRate < Quickfix::DoubleField
		def LegAutomaticExerciseThresholdRate.field
			return 41485
		end
		def initialize(data = nil)
			if( data == nil )
				super(41485)
			else
				super(41485, data)
			end
		end
	end

	class LegExerciseConfirmationMethod < Quickfix::IntField
		def LegExerciseConfirmationMethod.field
			return 41486
		end
		def initialize(data = nil)
			if( data == nil )
				super(41486)
			else
				super(41486, data)
			end
		end
	end

	class LegManualNoticeBusinessCenter < Quickfix::StringField
		def LegManualNoticeBusinessCenter.field
			return 41487
		end
		def initialize(data = nil)
			if( data == nil )
				super(41487)
			else
				super(41487, data)
			end
		end
	end

	class LegFallbackExerciseIndicator < Quickfix::BoolField
		def LegFallbackExerciseIndicator.field
			return 41488
		end
		def initialize(data = nil)
			if( data == nil )
				super(41488)
			else
				super(41488, data)
			end
		end
	end

	class LegLimitRightToConfirmIndicator < Quickfix::BoolField
		def LegLimitRightToConfirmIndicator.field
			return 41489
		end
		def initialize(data = nil)
			if( data == nil )
				super(41489)
			else
				super(41489, data)
			end
		end
	end

	class LegExerciseSplitTicketIndicator < Quickfix::BoolField
		def LegExerciseSplitTicketIndicator.field
			return 41490
		end
		def initialize(data = nil)
			if( data == nil )
				super(41490)
			else
				super(41490, data)
			end
		end
	end

	class NoLegOptionExerciseBusinessCenters < Quickfix::IntField
		def NoLegOptionExerciseBusinessCenters.field
			return 41491
		end
		def initialize(data = nil)
			if( data == nil )
				super(41491)
			else
				super(41491, data)
			end
		end
	end

	class LegOptionExerciseBusinessCenter < Quickfix::StringField
		def LegOptionExerciseBusinessCenter.field
			return 41492
		end
		def initialize(data = nil)
			if( data == nil )
				super(41492)
			else
				super(41492, data)
			end
		end
	end

	class LegOptionExerciseBusinessDayConvention < Quickfix::IntField
		def LegOptionExerciseBusinessDayConvention.field
			return 41493
		end
		def initialize(data = nil)
			if( data == nil )
				super(41493)
			else
				super(41493, data)
			end
		end
	end

	class LegOptionExerciseEarliestDateOffsetDayType < Quickfix::IntField
		def LegOptionExerciseEarliestDateOffsetDayType.field
			return 41494
		end
		def initialize(data = nil)
			if( data == nil )
				super(41494)
			else
				super(41494, data)
			end
		end
	end

	class LegOptionExerciseEarliestDateOffsetPeriod < Quickfix::IntField
		def LegOptionExerciseEarliestDateOffsetPeriod.field
			return 41495
		end
		def initialize(data = nil)
			if( data == nil )
				super(41495)
			else
				super(41495, data)
			end
		end
	end

	class LegOptionExerciseEarliestDateOffsetUnit < Quickfix::StringField
		def LegOptionExerciseEarliestDateOffsetUnit.field
			return 41496
		end
		def initialize(data = nil)
			if( data == nil )
				super(41496)
			else
				super(41496, data)
			end
		end
	end

	class LegOptionExerciseFrequencyPeriod < Quickfix::IntField
		def LegOptionExerciseFrequencyPeriod.field
			return 41497
		end
		def initialize(data = nil)
			if( data == nil )
				super(41497)
			else
				super(41497, data)
			end
		end
	end

	class LegOptionExerciseFrequencyUnit < Quickfix::StringField
		def LegOptionExerciseFrequencyUnit.field
			return 41498
		end
		def initialize(data = nil)
			if( data == nil )
				super(41498)
			else
				super(41498, data)
			end
		end
	end

	class LegOptionExerciseStartDateUnadjusted < Quickfix::StringField
		def LegOptionExerciseStartDateUnadjusted.field
			return 41499
		end
		def initialize(data = nil)
			if( data == nil )
				super(41499)
			else
				super(41499, data)
			end
		end
	end

	class LegOptionExerciseStartDateRelativeTo < Quickfix::IntField
		def LegOptionExerciseStartDateRelativeTo.field
			return 41500
		end
		def initialize(data = nil)
			if( data == nil )
				super(41500)
			else
				super(41500, data)
			end
		end
	end

	class LegOptionExerciseStartDateOffsetPeriod < Quickfix::IntField
		def LegOptionExerciseStartDateOffsetPeriod.field
			return 41501
		end
		def initialize(data = nil)
			if( data == nil )
				super(41501)
			else
				super(41501, data)
			end
		end
	end

	class LegOptionExerciseStartDateOffsetUnit < Quickfix::StringField
		def LegOptionExerciseStartDateOffsetUnit.field
			return 41502
		end
		def initialize(data = nil)
			if( data == nil )
				super(41502)
			else
				super(41502, data)
			end
		end
	end

	class LegOptionExerciseStartDateOffsetDayType < Quickfix::IntField
		def LegOptionExerciseStartDateOffsetDayType.field
			return 41503
		end
		def initialize(data = nil)
			if( data == nil )
				super(41503)
			else
				super(41503, data)
			end
		end
	end

	class LegOptionExerciseStartDateAdjusted < Quickfix::StringField
		def LegOptionExerciseStartDateAdjusted.field
			return 41504
		end
		def initialize(data = nil)
			if( data == nil )
				super(41504)
			else
				super(41504, data)
			end
		end
	end

	class LegOptionExerciseSkip < Quickfix::IntField
		def LegOptionExerciseSkip.field
			return 41505
		end
		def initialize(data = nil)
			if( data == nil )
				super(41505)
			else
				super(41505, data)
			end
		end
	end

	class LegOptionExerciseNominationDeadline < Quickfix::StringField
		def LegOptionExerciseNominationDeadline.field
			return 41506
		end
		def initialize(data = nil)
			if( data == nil )
				super(41506)
			else
				super(41506, data)
			end
		end
	end

	class LegOptionExerciseFirstDateUnadjusted < Quickfix::StringField
		def LegOptionExerciseFirstDateUnadjusted.field
			return 41507
		end
		def initialize(data = nil)
			if( data == nil )
				super(41507)
			else
				super(41507, data)
			end
		end
	end

	class LegOptionExerciseLastDateUnadjusted < Quickfix::StringField
		def LegOptionExerciseLastDateUnadjusted.field
			return 41508
		end
		def initialize(data = nil)
			if( data == nil )
				super(41508)
			else
				super(41508, data)
			end
		end
	end

	class LegOptionExerciseEarliestTime < Quickfix::StringField
		def LegOptionExerciseEarliestTime.field
			return 41509
		end
		def initialize(data = nil)
			if( data == nil )
				super(41509)
			else
				super(41509, data)
			end
		end
	end

	class LegOptionExerciseLatestTime < Quickfix::StringField
		def LegOptionExerciseLatestTime.field
			return 41510
		end
		def initialize(data = nil)
			if( data == nil )
				super(41510)
			else
				super(41510, data)
			end
		end
	end

	class LegOptionExerciseTimeBusinessCenter < Quickfix::StringField
		def LegOptionExerciseTimeBusinessCenter.field
			return 41511
		end
		def initialize(data = nil)
			if( data == nil )
				super(41511)
			else
				super(41511, data)
			end
		end
	end

	class NoLegOptionExerciseDates < Quickfix::IntField
		def NoLegOptionExerciseDates.field
			return 41512
		end
		def initialize(data = nil)
			if( data == nil )
				super(41512)
			else
				super(41512, data)
			end
		end
	end

	class LegOptionExerciseDate < Quickfix::StringField
		def LegOptionExerciseDate.field
			return 41513
		end
		def initialize(data = nil)
			if( data == nil )
				super(41513)
			else
				super(41513, data)
			end
		end
	end

	class LegOptionExerciseDateType < Quickfix::IntField
		def LegOptionExerciseDateType.field
			return 41514
		end
		def initialize(data = nil)
			if( data == nil )
				super(41514)
			else
				super(41514, data)
			end
		end
	end

	class NoLegOptionExerciseExpirationDateBusinessCenters < Quickfix::IntField
		def NoLegOptionExerciseExpirationDateBusinessCenters.field
			return 41515
		end
		def initialize(data = nil)
			if( data == nil )
				super(41515)
			else
				super(41515, data)
			end
		end
	end

	class LegOptionExerciseExpirationDateBusinessCenter < Quickfix::StringField
		def LegOptionExerciseExpirationDateBusinessCenter.field
			return 41516
		end
		def initialize(data = nil)
			if( data == nil )
				super(41516)
			else
				super(41516, data)
			end
		end
	end

	class LegOptionExerciseExpirationDateBusinessDayConvention < Quickfix::IntField
		def LegOptionExerciseExpirationDateBusinessDayConvention.field
			return 41517
		end
		def initialize(data = nil)
			if( data == nil )
				super(41517)
			else
				super(41517, data)
			end
		end
	end

	class LegOptionExerciseExpirationDateRelativeTo < Quickfix::IntField
		def LegOptionExerciseExpirationDateRelativeTo.field
			return 41518
		end
		def initialize(data = nil)
			if( data == nil )
				super(41518)
			else
				super(41518, data)
			end
		end
	end

	class LegOptionExerciseExpirationDateOffsetPeriod < Quickfix::IntField
		def LegOptionExerciseExpirationDateOffsetPeriod.field
			return 41519
		end
		def initialize(data = nil)
			if( data == nil )
				super(41519)
			else
				super(41519, data)
			end
		end
	end

	class LegOptionExerciseExpirationDateOffsetUnit < Quickfix::StringField
		def LegOptionExerciseExpirationDateOffsetUnit.field
			return 41520
		end
		def initialize(data = nil)
			if( data == nil )
				super(41520)
			else
				super(41520, data)
			end
		end
	end

	class LegOptionExerciseExpirationFrequencyPeriod < Quickfix::IntField
		def LegOptionExerciseExpirationFrequencyPeriod.field
			return 41521
		end
		def initialize(data = nil)
			if( data == nil )
				super(41521)
			else
				super(41521, data)
			end
		end
	end

	class LegOptionExerciseExpirationFrequencyUnit < Quickfix::StringField
		def LegOptionExerciseExpirationFrequencyUnit.field
			return 41522
		end
		def initialize(data = nil)
			if( data == nil )
				super(41522)
			else
				super(41522, data)
			end
		end
	end

	class LegOptionExerciseExpirationRollConvention < Quickfix::StringField
		def LegOptionExerciseExpirationRollConvention.field
			return 41523
		end
		def initialize(data = nil)
			if( data == nil )
				super(41523)
			else
				super(41523, data)
			end
		end
	end

	class LegOptionExerciseExpirationDateOffsetDayType < Quickfix::IntField
		def LegOptionExerciseExpirationDateOffsetDayType.field
			return 41524
		end
		def initialize(data = nil)
			if( data == nil )
				super(41524)
			else
				super(41524, data)
			end
		end
	end

	class LegOptionExerciseExpirationTime < Quickfix::StringField
		def LegOptionExerciseExpirationTime.field
			return 41525
		end
		def initialize(data = nil)
			if( data == nil )
				super(41525)
			else
				super(41525, data)
			end
		end
	end

	class LegOptionExerciseExpirationTimeBusinessCenter < Quickfix::StringField
		def LegOptionExerciseExpirationTimeBusinessCenter.field
			return 41526
		end
		def initialize(data = nil)
			if( data == nil )
				super(41526)
			else
				super(41526, data)
			end
		end
	end

	class NoLegOptionExerciseExpirationDates < Quickfix::IntField
		def NoLegOptionExerciseExpirationDates.field
			return 41527
		end
		def initialize(data = nil)
			if( data == nil )
				super(41527)
			else
				super(41527, data)
			end
		end
	end

	class LegOptionExerciseExpirationDate < Quickfix::StringField
		def LegOptionExerciseExpirationDate.field
			return 41528
		end
		def initialize(data = nil)
			if( data == nil )
				super(41528)
			else
				super(41528, data)
			end
		end
	end

	class LegOptionExerciseExpirationDateType < Quickfix::IntField
		def LegOptionExerciseExpirationDateType.field
			return 41529
		end
		def initialize(data = nil)
			if( data == nil )
				super(41529)
			else
				super(41529, data)
			end
		end
	end

	class NoLegPaymentScheduleFixingDays < Quickfix::IntField
		def NoLegPaymentScheduleFixingDays.field
			return 41530
		end
		def initialize(data = nil)
			if( data == nil )
				super(41530)
			else
				super(41530, data)
			end
		end
	end

	class LegPaymentScheduleFixingDayOfWeek < Quickfix::IntField
		def LegPaymentScheduleFixingDayOfWeek.field
			return 41531
		end
		def initialize(data = nil)
			if( data == nil )
				super(41531)
			else
				super(41531, data)
			end
		end
	end

	class LegPaymentScheduleFixingDayNumber < Quickfix::IntField
		def LegPaymentScheduleFixingDayNumber.field
			return 41532
		end
		def initialize(data = nil)
			if( data == nil )
				super(41532)
			else
				super(41532, data)
			end
		end
	end

	class LegPaymentScheduleXID < Quickfix::StringField
		def LegPaymentScheduleXID.field
			return 41533
		end
		def initialize(data = nil)
			if( data == nil )
				super(41533)
			else
				super(41533, data)
			end
		end
	end

	class LegPaymentScheduleXIDRef < Quickfix::StringField
		def LegPaymentScheduleXIDRef.field
			return 41534
		end
		def initialize(data = nil)
			if( data == nil )
				super(41534)
			else
				super(41534, data)
			end
		end
	end

	class LegPaymentScheduleRateCurrency < Quickfix::StringField
		def LegPaymentScheduleRateCurrency.field
			return 41535
		end
		def initialize(data = nil)
			if( data == nil )
				super(41535)
			else
				super(41535, data)
			end
		end
	end

	class LegPaymentScheduleRateUnitOfMeasure < Quickfix::StringField
		def LegPaymentScheduleRateUnitOfMeasure.field
			return 41536
		end
		def initialize(data = nil)
			if( data == nil )
				super(41536)
			else
				super(41536, data)
			end
		end
	end

	class LegPaymentScheduleRateConversionFactor < Quickfix::DoubleField
		def LegPaymentScheduleRateConversionFactor.field
			return 41537
		end
		def initialize(data = nil)
			if( data == nil )
				super(41537)
			else
				super(41537, data)
			end
		end
	end

	class LegPaymentScheduleRateSpreadType < Quickfix::IntField
		def LegPaymentScheduleRateSpreadType.field
			return 41538
		end
		def initialize(data = nil)
			if( data == nil )
				super(41538)
			else
				super(41538, data)
			end
		end
	end

	class LegPaymentScheduleSettlPeriodPrice < Quickfix::DoubleField
		def LegPaymentScheduleSettlPeriodPrice.field
			return 41539
		end
		def initialize(data = nil)
			if( data == nil )
				super(41539)
			else
				super(41539, data)
			end
		end
	end

	class LegPaymentScheduleSettlPeriodPriceCurrency < Quickfix::StringField
		def LegPaymentScheduleSettlPeriodPriceCurrency.field
			return 41540
		end
		def initialize(data = nil)
			if( data == nil )
				super(41540)
			else
				super(41540, data)
			end
		end
	end

	class LegPaymentScheduleSettlPeriodPriceUnitOfMeasure < Quickfix::StringField
		def LegPaymentScheduleSettlPeriodPriceUnitOfMeasure.field
			return 41541
		end
		def initialize(data = nil)
			if( data == nil )
				super(41541)
			else
				super(41541, data)
			end
		end
	end

	class LegPaymentScheduleStepUnitOfMeasure < Quickfix::StringField
		def LegPaymentScheduleStepUnitOfMeasure.field
			return 41542
		end
		def initialize(data = nil)
			if( data == nil )
				super(41542)
			else
				super(41542, data)
			end
		end
	end

	class LegPaymentScheduleFixingDayDistribution < Quickfix::IntField
		def LegPaymentScheduleFixingDayDistribution.field
			return 41543
		end
		def initialize(data = nil)
			if( data == nil )
				super(41543)
			else
				super(41543, data)
			end
		end
	end

	class LegPaymentScheduleFixingDayCount < Quickfix::IntField
		def LegPaymentScheduleFixingDayCount.field
			return 41544
		end
		def initialize(data = nil)
			if( data == nil )
				super(41544)
			else
				super(41544, data)
			end
		end
	end

	class LegPaymentScheduleFixingLagPeriod < Quickfix::IntField
		def LegPaymentScheduleFixingLagPeriod.field
			return 41545
		end
		def initialize(data = nil)
			if( data == nil )
				super(41545)
			else
				super(41545, data)
			end
		end
	end

	class LegPaymentScheduleFixingLagUnit < Quickfix::StringField
		def LegPaymentScheduleFixingLagUnit.field
			return 41546
		end
		def initialize(data = nil)
			if( data == nil )
				super(41546)
			else
				super(41546, data)
			end
		end
	end

	class LegPaymentScheduleFixingFirstObservationDateOffsetPeriod < Quickfix::IntField
		def LegPaymentScheduleFixingFirstObservationDateOffsetPeriod.field
			return 41547
		end
		def initialize(data = nil)
			if( data == nil )
				super(41547)
			else
				super(41547, data)
			end
		end
	end

	class LegPaymentScheduleFixingFirstObservationDateOffsetUnit < Quickfix::StringField
		def LegPaymentScheduleFixingFirstObservationDateOffsetUnit.field
			return 41548
		end
		def initialize(data = nil)
			if( data == nil )
				super(41548)
			else
				super(41548, data)
			end
		end
	end

	class LegPaymentStreamFlatRateIndicator < Quickfix::BoolField
		def LegPaymentStreamFlatRateIndicator.field
			return 41549
		end
		def initialize(data = nil)
			if( data == nil )
				super(41549)
			else
				super(41549, data)
			end
		end
	end

	class LegPaymentStreamFlatRateAmount < Quickfix::DoubleField
		def LegPaymentStreamFlatRateAmount.field
			return 41550
		end
		def initialize(data = nil)
			if( data == nil )
				super(41550)
			else
				super(41550, data)
			end
		end
	end

	class LegPaymentStreamFlatRateCurrency < Quickfix::StringField
		def LegPaymentStreamFlatRateCurrency.field
			return 41551
		end
		def initialize(data = nil)
			if( data == nil )
				super(41551)
			else
				super(41551, data)
			end
		end
	end

	class LegStreamMaximumPaymentAmount < Quickfix::DoubleField
		def LegStreamMaximumPaymentAmount.field
			return 41552
		end
		def initialize(data = nil)
			if( data == nil )
				super(41552)
			else
				super(41552, data)
			end
		end
	end

	class LegStreamMaximumPaymentCurrency < Quickfix::StringField
		def LegStreamMaximumPaymentCurrency.field
			return 41553
		end
		def initialize(data = nil)
			if( data == nil )
				super(41553)
			else
				super(41553, data)
			end
		end
	end

	class LegStreamMaximumTransactionAmount < Quickfix::DoubleField
		def LegStreamMaximumTransactionAmount.field
			return 41554
		end
		def initialize(data = nil)
			if( data == nil )
				super(41554)
			else
				super(41554, data)
			end
		end
	end

	class LegStreamMaximumTransactionCurrency < Quickfix::StringField
		def LegStreamMaximumTransactionCurrency.field
			return 41555
		end
		def initialize(data = nil)
			if( data == nil )
				super(41555)
			else
				super(41555, data)
			end
		end
	end

	class LegPaymentStreamFixedAmountUnitOfMeasure < Quickfix::StringField
		def LegPaymentStreamFixedAmountUnitOfMeasure.field
			return 41556
		end
		def initialize(data = nil)
			if( data == nil )
				super(41556)
			else
				super(41556, data)
			end
		end
	end

	class LegPaymentStreamTotalFixedAmount < Quickfix::DoubleField
		def LegPaymentStreamTotalFixedAmount.field
			return 41557
		end
		def initialize(data = nil)
			if( data == nil )
				super(41557)
			else
				super(41557, data)
			end
		end
	end

	class LegPaymentStreamWorldScaleRate < Quickfix::DoubleField
		def LegPaymentStreamWorldScaleRate.field
			return 41558
		end
		def initialize(data = nil)
			if( data == nil )
				super(41558)
			else
				super(41558, data)
			end
		end
	end

	class LegPaymentStreamContractPrice < Quickfix::DoubleField
		def LegPaymentStreamContractPrice.field
			return 41559
		end
		def initialize(data = nil)
			if( data == nil )
				super(41559)
			else
				super(41559, data)
			end
		end
	end

	class LegPaymentStreamContractPriceCurrency < Quickfix::StringField
		def LegPaymentStreamContractPriceCurrency.field
			return 41560
		end
		def initialize(data = nil)
			if( data == nil )
				super(41560)
			else
				super(41560, data)
			end
		end
	end

	class NoLegPaymentStreamPricingBusinessCenters < Quickfix::IntField
		def NoLegPaymentStreamPricingBusinessCenters.field
			return 41561
		end
		def initialize(data = nil)
			if( data == nil )
				super(41561)
			else
				super(41561, data)
			end
		end
	end

	class LegPaymentStreamPricingBusinessCenter < Quickfix::StringField
		def LegPaymentStreamPricingBusinessCenter.field
			return 41562
		end
		def initialize(data = nil)
			if( data == nil )
				super(41562)
			else
				super(41562, data)
			end
		end
	end

	class LegPaymentStreamRateIndex2CurveUnit < Quickfix::StringField
		def LegPaymentStreamRateIndex2CurveUnit.field
			return 41563
		end
		def initialize(data = nil)
			if( data == nil )
				super(41563)
			else
				super(41563, data)
			end
		end
	end

	class LegPaymentStreamRateIndex2CurvePeriod < Quickfix::IntField
		def LegPaymentStreamRateIndex2CurvePeriod.field
			return 41564
		end
		def initialize(data = nil)
			if( data == nil )
				super(41564)
			else
				super(41564, data)
			end
		end
	end

	class LegPaymentStreamRateIndexLocation < Quickfix::StringField
		def LegPaymentStreamRateIndexLocation.field
			return 41565
		end
		def initialize(data = nil)
			if( data == nil )
				super(41565)
			else
				super(41565, data)
			end
		end
	end

	class LegPaymentStreamRateIndexLevel < Quickfix::DoubleField
		def LegPaymentStreamRateIndexLevel.field
			return 41566
		end
		def initialize(data = nil)
			if( data == nil )
				super(41566)
			else
				super(41566, data)
			end
		end
	end

	class LegPaymentStreamRateIndexUnitOfMeasure < Quickfix::StringField
		def LegPaymentStreamRateIndexUnitOfMeasure.field
			return 41567
		end
		def initialize(data = nil)
			if( data == nil )
				super(41567)
			else
				super(41567, data)
			end
		end
	end

	class LegPaymentStreamSettlLevel < Quickfix::IntField
		def LegPaymentStreamSettlLevel.field
			return 41568
		end
		def initialize(data = nil)
			if( data == nil )
				super(41568)
			else
				super(41568, data)
			end
		end
	end

	class LegPaymentStreamReferenceLevel < Quickfix::DoubleField
		def LegPaymentStreamReferenceLevel.field
			return 41569
		end
		def initialize(data = nil)
			if( data == nil )
				super(41569)
			else
				super(41569, data)
			end
		end
	end

	class LegPaymentStreamReferenceLevelUnitOfMeasure < Quickfix::StringField
		def LegPaymentStreamReferenceLevelUnitOfMeasure.field
			return 41570
		end
		def initialize(data = nil)
			if( data == nil )
				super(41570)
			else
				super(41570, data)
			end
		end
	end

	class LegPaymentStreamReferenceLevelEqualsZeroIndicator < Quickfix::BoolField
		def LegPaymentStreamReferenceLevelEqualsZeroIndicator.field
			return 41571
		end
		def initialize(data = nil)
			if( data == nil )
				super(41571)
			else
				super(41571, data)
			end
		end
	end

	class LegPaymentStreamRateSpreadCurrency < Quickfix::StringField
		def LegPaymentStreamRateSpreadCurrency.field
			return 41572
		end
		def initialize(data = nil)
			if( data == nil )
				super(41572)
			else
				super(41572, data)
			end
		end
	end

	class LegPaymentStreamRateSpreadUnitOfMeasure < Quickfix::StringField
		def LegPaymentStreamRateSpreadUnitOfMeasure.field
			return 41573
		end
		def initialize(data = nil)
			if( data == nil )
				super(41573)
			else
				super(41573, data)
			end
		end
	end

	class LegPaymentStreamRateConversionFactor < Quickfix::DoubleField
		def LegPaymentStreamRateConversionFactor.field
			return 41574
		end
		def initialize(data = nil)
			if( data == nil )
				super(41574)
			else
				super(41574, data)
			end
		end
	end

	class LegPaymentStreamRateSpreadType < Quickfix::IntField
		def LegPaymentStreamRateSpreadType.field
			return 41575
		end
		def initialize(data = nil)
			if( data == nil )
				super(41575)
			else
				super(41575, data)
			end
		end
	end

	class LegPaymentStreamLastResetRate < Quickfix::DoubleField
		def LegPaymentStreamLastResetRate.field
			return 41576
		end
		def initialize(data = nil)
			if( data == nil )
				super(41576)
			else
				super(41576, data)
			end
		end
	end

	class LegPaymentStreamFinalRate < Quickfix::DoubleField
		def LegPaymentStreamFinalRate.field
			return 41577
		end
		def initialize(data = nil)
			if( data == nil )
				super(41577)
			else
				super(41577, data)
			end
		end
	end

	class LegPaymentStreamCalculationLagPeriod < Quickfix::IntField
		def LegPaymentStreamCalculationLagPeriod.field
			return 41578
		end
		def initialize(data = nil)
			if( data == nil )
				super(41578)
			else
				super(41578, data)
			end
		end
	end

	class LegPaymentStreamCalculationLagUnit < Quickfix::StringField
		def LegPaymentStreamCalculationLagUnit.field
			return 41579
		end
		def initialize(data = nil)
			if( data == nil )
				super(41579)
			else
				super(41579, data)
			end
		end
	end

	class LegPaymentStreamFirstObservationDateOffsetPeriod < Quickfix::IntField
		def LegPaymentStreamFirstObservationDateOffsetPeriod.field
			return 41580
		end
		def initialize(data = nil)
			if( data == nil )
				super(41580)
			else
				super(41580, data)
			end
		end
	end

	class LegPaymentStreamFirstObservationDateOffsetUnit < Quickfix::StringField
		def LegPaymentStreamFirstObservationDateOffsetUnit.field
			return 41581
		end
		def initialize(data = nil)
			if( data == nil )
				super(41581)
			else
				super(41581, data)
			end
		end
	end

	class LegPaymentStreamPricingDayType < Quickfix::IntField
		def LegPaymentStreamPricingDayType.field
			return 41582
		end
		def initialize(data = nil)
			if( data == nil )
				super(41582)
			else
				super(41582, data)
			end
		end
	end

	class LegPaymentStreamPricingDayDistribution < Quickfix::IntField
		def LegPaymentStreamPricingDayDistribution.field
			return 41583
		end
		def initialize(data = nil)
			if( data == nil )
				super(41583)
			else
				super(41583, data)
			end
		end
	end

	class LegPaymentStreamPricingDayCount < Quickfix::IntField
		def LegPaymentStreamPricingDayCount.field
			return 41584
		end
		def initialize(data = nil)
			if( data == nil )
				super(41584)
			else
				super(41584, data)
			end
		end
	end

	class LegPaymentStreamPricingBusinessCalendar < Quickfix::StringField
		def LegPaymentStreamPricingBusinessCalendar.field
			return 41585
		end
		def initialize(data = nil)
			if( data == nil )
				super(41585)
			else
				super(41585, data)
			end
		end
	end

	class LegPaymentStreamPricingBusinessDayConvention < Quickfix::IntField
		def LegPaymentStreamPricingBusinessDayConvention.field
			return 41586
		end
		def initialize(data = nil)
			if( data == nil )
				super(41586)
			else
				super(41586, data)
			end
		end
	end

	class NoLegPaymentStreamPaymentDates < Quickfix::IntField
		def NoLegPaymentStreamPaymentDates.field
			return 41589
		end
		def initialize(data = nil)
			if( data == nil )
				super(41589)
			else
				super(41589, data)
			end
		end
	end

	class LegPaymentStreamPaymentDate < Quickfix::StringField
		def LegPaymentStreamPaymentDate.field
			return 41590
		end
		def initialize(data = nil)
			if( data == nil )
				super(41590)
			else
				super(41590, data)
			end
		end
	end

	class LegPaymentStreamPaymentDateType < Quickfix::IntField
		def LegPaymentStreamPaymentDateType.field
			return 41591
		end
		def initialize(data = nil)
			if( data == nil )
				super(41591)
			else
				super(41591, data)
			end
		end
	end

	class LegPaymentStreamMasterAgreementPaymentDatesIndicator < Quickfix::BoolField
		def LegPaymentStreamMasterAgreementPaymentDatesIndicator.field
			return 41592
		end
		def initialize(data = nil)
			if( data == nil )
				super(41592)
			else
				super(41592, data)
			end
		end
	end

	class NoLegPaymentStreamPricingDates < Quickfix::IntField
		def NoLegPaymentStreamPricingDates.field
			return 41593
		end
		def initialize(data = nil)
			if( data == nil )
				super(41593)
			else
				super(41593, data)
			end
		end
	end

	class LegPaymentStreamPricingDate < Quickfix::StringField
		def LegPaymentStreamPricingDate.field
			return 41594
		end
		def initialize(data = nil)
			if( data == nil )
				super(41594)
			else
				super(41594, data)
			end
		end
	end

	class LegPaymentStreamPricingDateType < Quickfix::IntField
		def LegPaymentStreamPricingDateType.field
			return 41595
		end
		def initialize(data = nil)
			if( data == nil )
				super(41595)
			else
				super(41595, data)
			end
		end
	end

	class NoLegPaymentStreamPricingDays < Quickfix::IntField
		def NoLegPaymentStreamPricingDays.field
			return 41596
		end
		def initialize(data = nil)
			if( data == nil )
				super(41596)
			else
				super(41596, data)
			end
		end
	end

	class LegPaymentStreamPricingDayOfWeek < Quickfix::IntField
		def LegPaymentStreamPricingDayOfWeek.field
			return 41597
		end
		def initialize(data = nil)
			if( data == nil )
				super(41597)
			else
				super(41597, data)
			end
		end
	end

	class LegPaymentStreamPricingDayNumber < Quickfix::IntField
		def LegPaymentStreamPricingDayNumber.field
			return 41598
		end
		def initialize(data = nil)
			if( data == nil )
				super(41598)
			else
				super(41598, data)
			end
		end
	end

	class NoLegPhysicalSettlTerms < Quickfix::IntField
		def NoLegPhysicalSettlTerms.field
			return 41599
		end
		def initialize(data = nil)
			if( data == nil )
				super(41599)
			else
				super(41599, data)
			end
		end
	end

	class LegPhysicalSettlTermXID < Quickfix::StringField
		def LegPhysicalSettlTermXID.field
			return 41600
		end
		def initialize(data = nil)
			if( data == nil )
				super(41600)
			else
				super(41600, data)
			end
		end
	end

	class LegPhysicalSettlCurency < Quickfix::StringField
		def LegPhysicalSettlCurency.field
			return 41601
		end
		def initialize(data = nil)
			if( data == nil )
				super(41601)
			else
				super(41601, data)
			end
		end
	end

	class LegPhysicalSettlBusinessDays < Quickfix::IntField
		def LegPhysicalSettlBusinessDays.field
			return 41602
		end
		def initialize(data = nil)
			if( data == nil )
				super(41602)
			else
				super(41602, data)
			end
		end
	end

	class LegPhysicalSettlMaximumBusinessDays < Quickfix::IntField
		def LegPhysicalSettlMaximumBusinessDays.field
			return 41603
		end
		def initialize(data = nil)
			if( data == nil )
				super(41603)
			else
				super(41603, data)
			end
		end
	end

	class NoLegPhysicalSettlDeliverableObligations < Quickfix::IntField
		def NoLegPhysicalSettlDeliverableObligations.field
			return 41604
		end
		def initialize(data = nil)
			if( data == nil )
				super(41604)
			else
				super(41604, data)
			end
		end
	end

	class LegPhysicalSettlDeliverableObligationType < Quickfix::StringField
		def LegPhysicalSettlDeliverableObligationType.field
			return 41605
		end
		def initialize(data = nil)
			if( data == nil )
				super(41605)
			else
				super(41605, data)
			end
		end
	end

	class LegPhysicalSettlDeliverableObligationValue < Quickfix::StringField
		def LegPhysicalSettlDeliverableObligationValue.field
			return 41606
		end
		def initialize(data = nil)
			if( data == nil )
				super(41606)
			else
				super(41606, data)
			end
		end
	end

	class NoLegPricingDateBusinessCenters < Quickfix::IntField
		def NoLegPricingDateBusinessCenters.field
			return 41607
		end
		def initialize(data = nil)
			if( data == nil )
				super(41607)
			else
				super(41607, data)
			end
		end
	end

	class LegPricingDateBusinessCenter < Quickfix::StringField
		def LegPricingDateBusinessCenter.field
			return 41608
		end
		def initialize(data = nil)
			if( data == nil )
				super(41608)
			else
				super(41608, data)
			end
		end
	end

	class LegPricingDateUnadjusted < Quickfix::StringField
		def LegPricingDateUnadjusted.field
			return 41609
		end
		def initialize(data = nil)
			if( data == nil )
				super(41609)
			else
				super(41609, data)
			end
		end
	end

	class LegPricingDateBusinessDayConvention < Quickfix::IntField
		def LegPricingDateBusinessDayConvention.field
			return 41610
		end
		def initialize(data = nil)
			if( data == nil )
				super(41610)
			else
				super(41610, data)
			end
		end
	end

	class LegPricingDateAdjusted < Quickfix::StringField
		def LegPricingDateAdjusted.field
			return 41611
		end
		def initialize(data = nil)
			if( data == nil )
				super(41611)
			else
				super(41611, data)
			end
		end
	end

	class LegPricingTime < Quickfix::StringField
		def LegPricingTime.field
			return 41612
		end
		def initialize(data = nil)
			if( data == nil )
				super(41612)
			else
				super(41612, data)
			end
		end
	end

	class LegPricingTimeBusinessCenter < Quickfix::StringField
		def LegPricingTimeBusinessCenter.field
			return 41613
		end
		def initialize(data = nil)
			if( data == nil )
				super(41613)
			else
				super(41613, data)
			end
		end
	end

	class NoLegProtectionTermEventNewsSources < Quickfix::IntField
		def NoLegProtectionTermEventNewsSources.field
			return 41614
		end
		def initialize(data = nil)
			if( data == nil )
				super(41614)
			else
				super(41614, data)
			end
		end
	end

	class LegProtectionTermEventNewsSource < Quickfix::StringField
		def LegProtectionTermEventNewsSource.field
			return 41615
		end
		def initialize(data = nil)
			if( data == nil )
				super(41615)
			else
				super(41615, data)
			end
		end
	end

	class NoLegProtectionTerms < Quickfix::IntField
		def NoLegProtectionTerms.field
			return 41616
		end
		def initialize(data = nil)
			if( data == nil )
				super(41616)
			else
				super(41616, data)
			end
		end
	end

	class LegProtectionTermXID < Quickfix::StringField
		def LegProtectionTermXID.field
			return 41617
		end
		def initialize(data = nil)
			if( data == nil )
				super(41617)
			else
				super(41617, data)
			end
		end
	end

	class LegProtectionTermNotional < Quickfix::DoubleField
		def LegProtectionTermNotional.field
			return 41618
		end
		def initialize(data = nil)
			if( data == nil )
				super(41618)
			else
				super(41618, data)
			end
		end
	end

	class LegProtectionTermCurrency < Quickfix::StringField
		def LegProtectionTermCurrency.field
			return 41619
		end
		def initialize(data = nil)
			if( data == nil )
				super(41619)
			else
				super(41619, data)
			end
		end
	end

	class LegProtectionTermSellerNotifies < Quickfix::BoolField
		def LegProtectionTermSellerNotifies.field
			return 41620
		end
		def initialize(data = nil)
			if( data == nil )
				super(41620)
			else
				super(41620, data)
			end
		end
	end

	class LegProtectionTermBuyerNotifies < Quickfix::BoolField
		def LegProtectionTermBuyerNotifies.field
			return 41621
		end
		def initialize(data = nil)
			if( data == nil )
				super(41621)
			else
				super(41621, data)
			end
		end
	end

	class LegProtectionTermEventBusinessCenter < Quickfix::StringField
		def LegProtectionTermEventBusinessCenter.field
			return 41622
		end
		def initialize(data = nil)
			if( data == nil )
				super(41622)
			else
				super(41622, data)
			end
		end
	end

	class LegProtectionTermStandardSources < Quickfix::BoolField
		def LegProtectionTermStandardSources.field
			return 41623
		end
		def initialize(data = nil)
			if( data == nil )
				super(41623)
			else
				super(41623, data)
			end
		end
	end

	class LegProtectionTermEventMinimumSources < Quickfix::IntField
		def LegProtectionTermEventMinimumSources.field
			return 41624
		end
		def initialize(data = nil)
			if( data == nil )
				super(41624)
			else
				super(41624, data)
			end
		end
	end

	class NoLegProtectionTermEvents < Quickfix::IntField
		def NoLegProtectionTermEvents.field
			return 41625
		end
		def initialize(data = nil)
			if( data == nil )
				super(41625)
			else
				super(41625, data)
			end
		end
	end

	class LegProtectionTermEventType < Quickfix::StringField
		def LegProtectionTermEventType.field
			return 41626
		end
		def initialize(data = nil)
			if( data == nil )
				super(41626)
			else
				super(41626, data)
			end
		end
	end

	class LegProtectionTermEventValue < Quickfix::StringField
		def LegProtectionTermEventValue.field
			return 41627
		end
		def initialize(data = nil)
			if( data == nil )
				super(41627)
			else
				super(41627, data)
			end
		end
	end

	class LegProtectionTermEventCurrency < Quickfix::StringField
		def LegProtectionTermEventCurrency.field
			return 41628
		end
		def initialize(data = nil)
			if( data == nil )
				super(41628)
			else
				super(41628, data)
			end
		end
	end

	class LegProtectionTermEventPeriod < Quickfix::IntField
		def LegProtectionTermEventPeriod.field
			return 41629
		end
		def initialize(data = nil)
			if( data == nil )
				super(41629)
			else
				super(41629, data)
			end
		end
	end

	class LegProtectionTermEventUnit < Quickfix::StringField
		def LegProtectionTermEventUnit.field
			return 41630
		end
		def initialize(data = nil)
			if( data == nil )
				super(41630)
			else
				super(41630, data)
			end
		end
	end

	class LegProtectionTermEventDayType < Quickfix::IntField
		def LegProtectionTermEventDayType.field
			return 41631
		end
		def initialize(data = nil)
			if( data == nil )
				super(41631)
			else
				super(41631, data)
			end
		end
	end

	class LegProtectionTermEventRateSource < Quickfix::StringField
		def LegProtectionTermEventRateSource.field
			return 41632
		end
		def initialize(data = nil)
			if( data == nil )
				super(41632)
			else
				super(41632, data)
			end
		end
	end

	class NoLegProtectionTermEventQualifiers < Quickfix::IntField
		def NoLegProtectionTermEventQualifiers.field
			return 41633
		end
		def initialize(data = nil)
			if( data == nil )
				super(41633)
			else
				super(41633, data)
			end
		end
	end

	class LegProtectionTermEventQualifier < Quickfix::CharField
		def LegProtectionTermEventQualifier.field
			return 41634
		end
		def initialize(data = nil)
			if( data == nil )
				super(41634)
			else
				super(41634, data)
			end
		end
	end

	class NoLegProtectionTermObligations < Quickfix::IntField
		def NoLegProtectionTermObligations.field
			return 41635
		end
		def initialize(data = nil)
			if( data == nil )
				super(41635)
			else
				super(41635, data)
			end
		end
	end

	class LegProtectionTermObligationType < Quickfix::StringField
		def LegProtectionTermObligationType.field
			return 41636
		end
		def initialize(data = nil)
			if( data == nil )
				super(41636)
			else
				super(41636, data)
			end
		end
	end

	class LegProtectionTermObligationValue < Quickfix::StringField
		def LegProtectionTermObligationValue.field
			return 41637
		end
		def initialize(data = nil)
			if( data == nil )
				super(41637)
			else
				super(41637, data)
			end
		end
	end

	class NoLegStreamCalculationPeriodDates < Quickfix::IntField
		def NoLegStreamCalculationPeriodDates.field
			return 41638
		end
		def initialize(data = nil)
			if( data == nil )
				super(41638)
			else
				super(41638, data)
			end
		end
	end

	class LegStreamCalculationPeriodDate < Quickfix::StringField
		def LegStreamCalculationPeriodDate.field
			return 41639
		end
		def initialize(data = nil)
			if( data == nil )
				super(41639)
			else
				super(41639, data)
			end
		end
	end

	class LegStreamCalculationPeriodDateType < Quickfix::IntField
		def LegStreamCalculationPeriodDateType.field
			return 41640
		end
		def initialize(data = nil)
			if( data == nil )
				super(41640)
			else
				super(41640, data)
			end
		end
	end

	class LegStreamCalculationPeriodDatesXID < Quickfix::StringField
		def LegStreamCalculationPeriodDatesXID.field
			return 41641
		end
		def initialize(data = nil)
			if( data == nil )
				super(41641)
			else
				super(41641, data)
			end
		end
	end

	class LegStreamCalculationPeriodDatesXIDRef < Quickfix::StringField
		def LegStreamCalculationPeriodDatesXIDRef.field
			return 41642
		end
		def initialize(data = nil)
			if( data == nil )
				super(41642)
			else
				super(41642, data)
			end
		end
	end

	class LegStreamCalculationBalanceOfFirstPeriod < Quickfix::BoolField
		def LegStreamCalculationBalanceOfFirstPeriod.field
			return 41643
		end
		def initialize(data = nil)
			if( data == nil )
				super(41643)
			else
				super(41643, data)
			end
		end
	end

	class LegStreamCalculationCorrectionPeriod < Quickfix::IntField
		def LegStreamCalculationCorrectionPeriod.field
			return 41644
		end
		def initialize(data = nil)
			if( data == nil )
				super(41644)
			else
				super(41644, data)
			end
		end
	end

	class LegStreamCalculationCorrectionUnit < Quickfix::StringField
		def LegStreamCalculationCorrectionUnit.field
			return 41645
		end
		def initialize(data = nil)
			if( data == nil )
				super(41645)
			else
				super(41645, data)
			end
		end
	end

	class NoLegStreamCommoditySettlBusinessCenters < Quickfix::IntField
		def NoLegStreamCommoditySettlBusinessCenters.field
			return 41646
		end
		def initialize(data = nil)
			if( data == nil )
				super(41646)
			else
				super(41646, data)
			end
		end
	end

	class LegStreamCommoditySettlBusinessCenter < Quickfix::StringField
		def LegStreamCommoditySettlBusinessCenter.field
			return 41647
		end
		def initialize(data = nil)
			if( data == nil )
				super(41647)
			else
				super(41647, data)
			end
		end
	end

	class LegStreamCommodityBase < Quickfix::StringField
		def LegStreamCommodityBase.field
			return 41648
		end
		def initialize(data = nil)
			if( data == nil )
				super(41648)
			else
				super(41648, data)
			end
		end
	end

	class LegStreamCommodityType < Quickfix::StringField
		def LegStreamCommodityType.field
			return 41649
		end
		def initialize(data = nil)
			if( data == nil )
				super(41649)
			else
				super(41649, data)
			end
		end
	end

	class LegStreamCommoditySecurityID < Quickfix::StringField
		def LegStreamCommoditySecurityID.field
			return 41650
		end
		def initialize(data = nil)
			if( data == nil )
				super(41650)
			else
				super(41650, data)
			end
		end
	end

	class LegStreamCommoditySecurityIDSource < Quickfix::StringField
		def LegStreamCommoditySecurityIDSource.field
			return 41651
		end
		def initialize(data = nil)
			if( data == nil )
				super(41651)
			else
				super(41651, data)
			end
		end
	end

	class LegStreamCommodityDesc < Quickfix::StringField
		def LegStreamCommodityDesc.field
			return 41652
		end
		def initialize(data = nil)
			if( data == nil )
				super(41652)
			else
				super(41652, data)
			end
		end
	end

	class EncodedLegStreamCommodityDescLen < Quickfix::IntField
		def EncodedLegStreamCommodityDescLen.field
			return 41653
		end
		def initialize(data = nil)
			if( data == nil )
				super(41653)
			else
				super(41653, data)
			end
		end
	end

	class EncodedLegStreamCommodityDesc < Quickfix::StringField
		def EncodedLegStreamCommodityDesc.field
			return 41654
		end
		def initialize(data = nil)
			if( data == nil )
				super(41654)
			else
				super(41654, data)
			end
		end
	end

	class LegStreamCommodityUnitOfMeasure < Quickfix::StringField
		def LegStreamCommodityUnitOfMeasure.field
			return 41655
		end
		def initialize(data = nil)
			if( data == nil )
				super(41655)
			else
				super(41655, data)
			end
		end
	end

	class LegStreamCommodityCurrency < Quickfix::StringField
		def LegStreamCommodityCurrency.field
			return 41656
		end
		def initialize(data = nil)
			if( data == nil )
				super(41656)
			else
				super(41656, data)
			end
		end
	end

	class LegStreamCommodityExchange < Quickfix::StringField
		def LegStreamCommodityExchange.field
			return 41657
		end
		def initialize(data = nil)
			if( data == nil )
				super(41657)
			else
				super(41657, data)
			end
		end
	end

	class LegStreamCommodityRateSource < Quickfix::IntField
		def LegStreamCommodityRateSource.field
			return 41658
		end
		def initialize(data = nil)
			if( data == nil )
				super(41658)
			else
				super(41658, data)
			end
		end
	end

	class LegStreamCommodityRateReferencePage < Quickfix::StringField
		def LegStreamCommodityRateReferencePage.field
			return 41659
		end
		def initialize(data = nil)
			if( data == nil )
				super(41659)
			else
				super(41659, data)
			end
		end
	end

	class LegStreamCommodityRateReferencePageHeading < Quickfix::StringField
		def LegStreamCommodityRateReferencePageHeading.field
			return 41660
		end
		def initialize(data = nil)
			if( data == nil )
				super(41660)
			else
				super(41660, data)
			end
		end
	end

	class LegStreamDataProvider < Quickfix::StringField
		def LegStreamDataProvider.field
			return 41661
		end
		def initialize(data = nil)
			if( data == nil )
				super(41661)
			else
				super(41661, data)
			end
		end
	end

	class LegStreamCommodityPricingType < Quickfix::StringField
		def LegStreamCommodityPricingType.field
			return 41662
		end
		def initialize(data = nil)
			if( data == nil )
				super(41662)
			else
				super(41662, data)
			end
		end
	end

	class LegStreamCommodityNearbySettlDayPeriod < Quickfix::IntField
		def LegStreamCommodityNearbySettlDayPeriod.field
			return 41663
		end
		def initialize(data = nil)
			if( data == nil )
				super(41663)
			else
				super(41663, data)
			end
		end
	end

	class LegStreamCommodityNearbySettlDayUnit < Quickfix::StringField
		def LegStreamCommodityNearbySettlDayUnit.field
			return 41664
		end
		def initialize(data = nil)
			if( data == nil )
				super(41664)
			else
				super(41664, data)
			end
		end
	end

	class LegStreamCommoditySettlDateUnadjusted < Quickfix::StringField
		def LegStreamCommoditySettlDateUnadjusted.field
			return 41665
		end
		def initialize(data = nil)
			if( data == nil )
				super(41665)
			else
				super(41665, data)
			end
		end
	end

	class LegStreamCommoditySettlDateBusinessDayConvention < Quickfix::IntField
		def LegStreamCommoditySettlDateBusinessDayConvention.field
			return 41666
		end
		def initialize(data = nil)
			if( data == nil )
				super(41666)
			else
				super(41666, data)
			end
		end
	end

	class LegStreamCommoditySettlDateAdjusted < Quickfix::StringField
		def LegStreamCommoditySettlDateAdjusted.field
			return 41667
		end
		def initialize(data = nil)
			if( data == nil )
				super(41667)
			else
				super(41667, data)
			end
		end
	end

	class LegStreamCommoditySettlMonth < Quickfix::IntField
		def LegStreamCommoditySettlMonth.field
			return 41668
		end
		def initialize(data = nil)
			if( data == nil )
				super(41668)
			else
				super(41668, data)
			end
		end
	end

	class LegStreamCommoditySettlDateRollPeriod < Quickfix::IntField
		def LegStreamCommoditySettlDateRollPeriod.field
			return 41669
		end
		def initialize(data = nil)
			if( data == nil )
				super(41669)
			else
				super(41669, data)
			end
		end
	end

	class LegStreamCommoditySettlDateRollUnit < Quickfix::StringField
		def LegStreamCommoditySettlDateRollUnit.field
			return 41670
		end
		def initialize(data = nil)
			if( data == nil )
				super(41670)
			else
				super(41670, data)
			end
		end
	end

	class LegStreamCommoditySettlDayType < Quickfix::IntField
		def LegStreamCommoditySettlDayType.field
			return 41671
		end
		def initialize(data = nil)
			if( data == nil )
				super(41671)
			else
				super(41671, data)
			end
		end
	end

	class LegStreamCommodityXID < Quickfix::StringField
		def LegStreamCommodityXID.field
			return 41672
		end
		def initialize(data = nil)
			if( data == nil )
				super(41672)
			else
				super(41672, data)
			end
		end
	end

	class LegStreamCommodityXIDRef < Quickfix::StringField
		def LegStreamCommodityXIDRef.field
			return 41673
		end
		def initialize(data = nil)
			if( data == nil )
				super(41673)
			else
				super(41673, data)
			end
		end
	end

	class NoLegStreamCommodityAltIDs < Quickfix::IntField
		def NoLegStreamCommodityAltIDs.field
			return 41674
		end
		def initialize(data = nil)
			if( data == nil )
				super(41674)
			else
				super(41674, data)
			end
		end
	end

	class LegStreamCommodityAltID < Quickfix::StringField
		def LegStreamCommodityAltID.field
			return 41675
		end
		def initialize(data = nil)
			if( data == nil )
				super(41675)
			else
				super(41675, data)
			end
		end
	end

	class LegStreamCommodityAltIDSource < Quickfix::StringField
		def LegStreamCommodityAltIDSource.field
			return 41676
		end
		def initialize(data = nil)
			if( data == nil )
				super(41676)
			else
				super(41676, data)
			end
		end
	end

	class NoLegStreamCommodityDataSources < Quickfix::IntField
		def NoLegStreamCommodityDataSources.field
			return 41677
		end
		def initialize(data = nil)
			if( data == nil )
				super(41677)
			else
				super(41677, data)
			end
		end
	end

	class LegStreamCommodityDataSourceID < Quickfix::StringField
		def LegStreamCommodityDataSourceID.field
			return 41678
		end
		def initialize(data = nil)
			if( data == nil )
				super(41678)
			else
				super(41678, data)
			end
		end
	end

	class LegStreamCommodityDataSourceIDType < Quickfix::IntField
		def LegStreamCommodityDataSourceIDType.field
			return 41679
		end
		def initialize(data = nil)
			if( data == nil )
				super(41679)
			else
				super(41679, data)
			end
		end
	end

	class NoLegStreamCommoditySettlDays < Quickfix::IntField
		def NoLegStreamCommoditySettlDays.field
			return 41680
		end
		def initialize(data = nil)
			if( data == nil )
				super(41680)
			else
				super(41680, data)
			end
		end
	end

	class LegStreamCommoditySettlDay < Quickfix::IntField
		def LegStreamCommoditySettlDay.field
			return 41681
		end
		def initialize(data = nil)
			if( data == nil )
				super(41681)
			else
				super(41681, data)
			end
		end
	end

	class LegStreamCommoditySettlTotalHours < Quickfix::IntField
		def LegStreamCommoditySettlTotalHours.field
			return 41682
		end
		def initialize(data = nil)
			if( data == nil )
				super(41682)
			else
				super(41682, data)
			end
		end
	end

	class NoLegStreamCommoditySettlTimes < Quickfix::IntField
		def NoLegStreamCommoditySettlTimes.field
			return 41683
		end
		def initialize(data = nil)
			if( data == nil )
				super(41683)
			else
				super(41683, data)
			end
		end
	end

	class LegStreamCommoditySettlStart < Quickfix::StringField
		def LegStreamCommoditySettlStart.field
			return 41684
		end
		def initialize(data = nil)
			if( data == nil )
				super(41684)
			else
				super(41684, data)
			end
		end
	end

	class LegStreamCommoditySettlEnd < Quickfix::StringField
		def LegStreamCommoditySettlEnd.field
			return 41685
		end
		def initialize(data = nil)
			if( data == nil )
				super(41685)
			else
				super(41685, data)
			end
		end
	end

	class LegStreamCommoditySettlTimeType < Quickfix::IntField
		def LegStreamCommoditySettlTimeType.field
			return 41935
		end
		def initialize(data = nil)
			if( data == nil )
				super(41935)
			else
				super(41935, data)
			end
		end
	end

	class NoLegStreamCommoditySettlPeriods < Quickfix::IntField
		def NoLegStreamCommoditySettlPeriods.field
			return 41686
		end
		def initialize(data = nil)
			if( data == nil )
				super(41686)
			else
				super(41686, data)
			end
		end
	end

	class LegStreamCommoditySettlCountry < Quickfix::StringField
		def LegStreamCommoditySettlCountry.field
			return 41687
		end
		def initialize(data = nil)
			if( data == nil )
				super(41687)
			else
				super(41687, data)
			end
		end
	end

	class LegStreamCommoditySettlTimeZone < Quickfix::StringField
		def LegStreamCommoditySettlTimeZone.field
			return 41688
		end
		def initialize(data = nil)
			if( data == nil )
				super(41688)
			else
				super(41688, data)
			end
		end
	end

	class LegStreamCommoditySettlFlowType < Quickfix::IntField
		def LegStreamCommoditySettlFlowType.field
			return 41689
		end
		def initialize(data = nil)
			if( data == nil )
				super(41689)
			else
				super(41689, data)
			end
		end
	end

	class LegStreamCommoditySettlPeriodNotional < Quickfix::DoubleField
		def LegStreamCommoditySettlPeriodNotional.field
			return 41690
		end
		def initialize(data = nil)
			if( data == nil )
				super(41690)
			else
				super(41690, data)
			end
		end
	end

	class LegStreamCommoditySettlPeriodNotionalUnitOfMeasure < Quickfix::StringField
		def LegStreamCommoditySettlPeriodNotionalUnitOfMeasure.field
			return 41691
		end
		def initialize(data = nil)
			if( data == nil )
				super(41691)
			else
				super(41691, data)
			end
		end
	end

	class LegStreamCommoditySettlPeriodFrequencyPeriod < Quickfix::IntField
		def LegStreamCommoditySettlPeriodFrequencyPeriod.field
			return 41692
		end
		def initialize(data = nil)
			if( data == nil )
				super(41692)
			else
				super(41692, data)
			end
		end
	end

	class LegStreamCommoditySettlPeriodFrequencyUnit < Quickfix::StringField
		def LegStreamCommoditySettlPeriodFrequencyUnit.field
			return 41693
		end
		def initialize(data = nil)
			if( data == nil )
				super(41693)
			else
				super(41693, data)
			end
		end
	end

	class LegStreamCommoditySettlPeriodPrice < Quickfix::DoubleField
		def LegStreamCommoditySettlPeriodPrice.field
			return 41694
		end
		def initialize(data = nil)
			if( data == nil )
				super(41694)
			else
				super(41694, data)
			end
		end
	end

	class LegStreamCommoditySettlPeriodPriceUnitOfMeasure < Quickfix::StringField
		def LegStreamCommoditySettlPeriodPriceUnitOfMeasure.field
			return 41695
		end
		def initialize(data = nil)
			if( data == nil )
				super(41695)
			else
				super(41695, data)
			end
		end
	end

	class LegStreamCommoditySettlPeriodPriceCurrency < Quickfix::StringField
		def LegStreamCommoditySettlPeriodPriceCurrency.field
			return 41696
		end
		def initialize(data = nil)
			if( data == nil )
				super(41696)
			else
				super(41696, data)
			end
		end
	end

	class LegStreamCommoditySettlHolidaysProcessingInstruction < Quickfix::IntField
		def LegStreamCommoditySettlHolidaysProcessingInstruction.field
			return 41697
		end
		def initialize(data = nil)
			if( data == nil )
				super(41697)
			else
				super(41697, data)
			end
		end
	end

	class LegStreamCommoditySettlPeriodXID < Quickfix::StringField
		def LegStreamCommoditySettlPeriodXID.field
			return 41698
		end
		def initialize(data = nil)
			if( data == nil )
				super(41698)
			else
				super(41698, data)
			end
		end
	end

	class LegStreamCommoditySettlPeriodXIDRef < Quickfix::StringField
		def LegStreamCommoditySettlPeriodXIDRef.field
			return 41699
		end
		def initialize(data = nil)
			if( data == nil )
				super(41699)
			else
				super(41699, data)
			end
		end
	end

	class LegStreamXID < Quickfix::StringField
		def LegStreamXID.field
			return 41700
		end
		def initialize(data = nil)
			if( data == nil )
				super(41700)
			else
				super(41700, data)
			end
		end
	end

	class LegStreamNotionalXIDRef < Quickfix::StringField
		def LegStreamNotionalXIDRef.field
			return 41702
		end
		def initialize(data = nil)
			if( data == nil )
				super(41702)
			else
				super(41702, data)
			end
		end
	end

	class LegStreamNotionalFrequencyPeriod < Quickfix::IntField
		def LegStreamNotionalFrequencyPeriod.field
			return 41703
		end
		def initialize(data = nil)
			if( data == nil )
				super(41703)
			else
				super(41703, data)
			end
		end
	end

	class LegStreamNotionalFrequencyUnit < Quickfix::StringField
		def LegStreamNotionalFrequencyUnit.field
			return 41704
		end
		def initialize(data = nil)
			if( data == nil )
				super(41704)
			else
				super(41704, data)
			end
		end
	end

	class LegStreamNotionalCommodityFrequency < Quickfix::IntField
		def LegStreamNotionalCommodityFrequency.field
			return 41705
		end
		def initialize(data = nil)
			if( data == nil )
				super(41705)
			else
				super(41705, data)
			end
		end
	end

	class LegStreamNotionalUnitOfMeasure < Quickfix::StringField
		def LegStreamNotionalUnitOfMeasure.field
			return 41706
		end
		def initialize(data = nil)
			if( data == nil )
				super(41706)
			else
				super(41706, data)
			end
		end
	end

	class LegStreamTotalNotional < Quickfix::DoubleField
		def LegStreamTotalNotional.field
			return 41707
		end
		def initialize(data = nil)
			if( data == nil )
				super(41707)
			else
				super(41707, data)
			end
		end
	end

	class LegStreamTotalNotionalUnitOfMeasure < Quickfix::StringField
		def LegStreamTotalNotionalUnitOfMeasure.field
			return 41708
		end
		def initialize(data = nil)
			if( data == nil )
				super(41708)
			else
				super(41708, data)
			end
		end
	end

	class NoUnderlyingAssetAttributes < Quickfix::IntField
		def NoUnderlyingAssetAttributes.field
			return 2312
		end
		def initialize(data = nil)
			if( data == nil )
				super(2312)
			else
				super(2312, data)
			end
		end
	end

	class UnderlyingAssetAttributeType < Quickfix::StringField
		def UnderlyingAssetAttributeType.field
			return 2313
		end
		def initialize(data = nil)
			if( data == nil )
				super(2313)
			else
				super(2313, data)
			end
		end
	end

	class UnderlyingAssetAttributeValue < Quickfix::StringField
		def UnderlyingAssetAttributeValue.field
			return 2314
		end
		def initialize(data = nil)
			if( data == nil )
				super(2314)
			else
				super(2314, data)
			end
		end
	end

	class UnderlyingAssetAttributeLimit < Quickfix::StringField
		def UnderlyingAssetAttributeLimit.field
			return 2315
		end
		def initialize(data = nil)
			if( data == nil )
				super(2315)
			else
				super(2315, data)
			end
		end
	end

	class NoUnderlyingComplexEventAveragingObservations < Quickfix::IntField
		def NoUnderlyingComplexEventAveragingObservations.field
			return 41713
		end
		def initialize(data = nil)
			if( data == nil )
				super(41713)
			else
				super(41713, data)
			end
		end
	end

	class UnderlyingComplexEventAveragingObservationNumber < Quickfix::IntField
		def UnderlyingComplexEventAveragingObservationNumber.field
			return 41714
		end
		def initialize(data = nil)
			if( data == nil )
				super(41714)
			else
				super(41714, data)
			end
		end
	end

	class UnderlyingComplexEventAveragingWeight < Quickfix::DoubleField
		def UnderlyingComplexEventAveragingWeight.field
			return 41715
		end
		def initialize(data = nil)
			if( data == nil )
				super(41715)
			else
				super(41715, data)
			end
		end
	end

	class NoUnderlyingComplexEventCreditEvents < Quickfix::IntField
		def NoUnderlyingComplexEventCreditEvents.field
			return 41716
		end
		def initialize(data = nil)
			if( data == nil )
				super(41716)
			else
				super(41716, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventType < Quickfix::StringField
		def UnderlyingComplexEventCreditEventType.field
			return 41717
		end
		def initialize(data = nil)
			if( data == nil )
				super(41717)
			else
				super(41717, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventValue < Quickfix::StringField
		def UnderlyingComplexEventCreditEventValue.field
			return 41718
		end
		def initialize(data = nil)
			if( data == nil )
				super(41718)
			else
				super(41718, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventCurrency < Quickfix::StringField
		def UnderlyingComplexEventCreditEventCurrency.field
			return 41719
		end
		def initialize(data = nil)
			if( data == nil )
				super(41719)
			else
				super(41719, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventPeriod < Quickfix::IntField
		def UnderlyingComplexEventCreditEventPeriod.field
			return 41720
		end
		def initialize(data = nil)
			if( data == nil )
				super(41720)
			else
				super(41720, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventUnit < Quickfix::StringField
		def UnderlyingComplexEventCreditEventUnit.field
			return 41721
		end
		def initialize(data = nil)
			if( data == nil )
				super(41721)
			else
				super(41721, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventDayType < Quickfix::IntField
		def UnderlyingComplexEventCreditEventDayType.field
			return 41722
		end
		def initialize(data = nil)
			if( data == nil )
				super(41722)
			else
				super(41722, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventRateSource < Quickfix::IntField
		def UnderlyingComplexEventCreditEventRateSource.field
			return 41723
		end
		def initialize(data = nil)
			if( data == nil )
				super(41723)
			else
				super(41723, data)
			end
		end
	end

	class NoUnderlyingComplexEventCreditEventQualifiers < Quickfix::IntField
		def NoUnderlyingComplexEventCreditEventQualifiers.field
			return 41724
		end
		def initialize(data = nil)
			if( data == nil )
				super(41724)
			else
				super(41724, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventQualifier < Quickfix::CharField
		def UnderlyingComplexEventCreditEventQualifier.field
			return 41725
		end
		def initialize(data = nil)
			if( data == nil )
				super(41725)
			else
				super(41725, data)
			end
		end
	end

	class NoUnderlyingComplexEventPeriodDateTimes < Quickfix::IntField
		def NoUnderlyingComplexEventPeriodDateTimes.field
			return 41726
		end
		def initialize(data = nil)
			if( data == nil )
				super(41726)
			else
				super(41726, data)
			end
		end
	end

	class UnderlyingComplexEventPeriodDate < Quickfix::StringField
		def UnderlyingComplexEventPeriodDate.field
			return 41727
		end
		def initialize(data = nil)
			if( data == nil )
				super(41727)
			else
				super(41727, data)
			end
		end
	end

	class UnderlyingComplexEventPeriodTime < Quickfix::StringField
		def UnderlyingComplexEventPeriodTime.field
			return 41728
		end
		def initialize(data = nil)
			if( data == nil )
				super(41728)
			else
				super(41728, data)
			end
		end
	end

	class NoUnderlyingComplexEventPeriods < Quickfix::IntField
		def NoUnderlyingComplexEventPeriods.field
			return 41729
		end
		def initialize(data = nil)
			if( data == nil )
				super(41729)
			else
				super(41729, data)
			end
		end
	end

	class UnderlyingComplexEventPeriodType < Quickfix::IntField
		def UnderlyingComplexEventPeriodType.field
			return 41730
		end
		def initialize(data = nil)
			if( data == nil )
				super(41730)
			else
				super(41730, data)
			end
		end
	end

	class UnderlyingComplexEventBusinessCenter < Quickfix::StringField
		def UnderlyingComplexEventBusinessCenter.field
			return 41731
		end
		def initialize(data = nil)
			if( data == nil )
				super(41731)
			else
				super(41731, data)
			end
		end
	end

	class NoUnderlyingComplexEventRateSources < Quickfix::IntField
		def NoUnderlyingComplexEventRateSources.field
			return 41732
		end
		def initialize(data = nil)
			if( data == nil )
				super(41732)
			else
				super(41732, data)
			end
		end
	end

	class UnderlyingComplexEventRateSource < Quickfix::IntField
		def UnderlyingComplexEventRateSource.field
			return 41733
		end
		def initialize(data = nil)
			if( data == nil )
				super(41733)
			else
				super(41733, data)
			end
		end
	end

	class UnderlyingComplexEventRateSourceType < Quickfix::IntField
		def UnderlyingComplexEventRateSourceType.field
			return 41734
		end
		def initialize(data = nil)
			if( data == nil )
				super(41734)
			else
				super(41734, data)
			end
		end
	end

	class UnderlyingComplexEventReferencePage < Quickfix::StringField
		def UnderlyingComplexEventReferencePage.field
			return 41735
		end
		def initialize(data = nil)
			if( data == nil )
				super(41735)
			else
				super(41735, data)
			end
		end
	end

	class UnderlyingComplexEventReferencePageHeading < Quickfix::StringField
		def UnderlyingComplexEventReferencePageHeading.field
			return 41736
		end
		def initialize(data = nil)
			if( data == nil )
				super(41736)
			else
				super(41736, data)
			end
		end
	end

	class NoUnderlyingComplexEventDateBusinessCenters < Quickfix::IntField
		def NoUnderlyingComplexEventDateBusinessCenters.field
			return 41737
		end
		def initialize(data = nil)
			if( data == nil )
				super(41737)
			else
				super(41737, data)
			end
		end
	end

	class UnderlyingComplexEventDateBusinessCenter < Quickfix::StringField
		def UnderlyingComplexEventDateBusinessCenter.field
			return 41738
		end
		def initialize(data = nil)
			if( data == nil )
				super(41738)
			else
				super(41738, data)
			end
		end
	end

	class UnderlyingComplexEventDateUnadjusted < Quickfix::StringField
		def UnderlyingComplexEventDateUnadjusted.field
			return 41739
		end
		def initialize(data = nil)
			if( data == nil )
				super(41739)
			else
				super(41739, data)
			end
		end
	end

	class UnderlyingComplexEventDateRelativeTo < Quickfix::IntField
		def UnderlyingComplexEventDateRelativeTo.field
			return 41740
		end
		def initialize(data = nil)
			if( data == nil )
				super(41740)
			else
				super(41740, data)
			end
		end
	end

	class UnderlyingComplexEventDateOffsetPeriod < Quickfix::IntField
		def UnderlyingComplexEventDateOffsetPeriod.field
			return 41741
		end
		def initialize(data = nil)
			if( data == nil )
				super(41741)
			else
				super(41741, data)
			end
		end
	end

	class UnderlyingComplexEventDateOffsetUnit < Quickfix::StringField
		def UnderlyingComplexEventDateOffsetUnit.field
			return 41742
		end
		def initialize(data = nil)
			if( data == nil )
				super(41742)
			else
				super(41742, data)
			end
		end
	end

	class UnderlyingComplexEventDateOffsetDayType < Quickfix::IntField
		def UnderlyingComplexEventDateOffsetDayType.field
			return 41743
		end
		def initialize(data = nil)
			if( data == nil )
				super(41743)
			else
				super(41743, data)
			end
		end
	end

	class UnderlyingComplexEventDateBusinessDayConvention < Quickfix::IntField
		def UnderlyingComplexEventDateBusinessDayConvention.field
			return 41744
		end
		def initialize(data = nil)
			if( data == nil )
				super(41744)
			else
				super(41744, data)
			end
		end
	end

	class UnderlyingComplexEventDateAdjusted < Quickfix::StringField
		def UnderlyingComplexEventDateAdjusted.field
			return 41745
		end
		def initialize(data = nil)
			if( data == nil )
				super(41745)
			else
				super(41745, data)
			end
		end
	end

	class UnderlyingComplexEventFixingTime < Quickfix::StringField
		def UnderlyingComplexEventFixingTime.field
			return 41746
		end
		def initialize(data = nil)
			if( data == nil )
				super(41746)
			else
				super(41746, data)
			end
		end
	end

	class UnderlyingComplexEventFixingTimeBusinessCenter < Quickfix::StringField
		def UnderlyingComplexEventFixingTimeBusinessCenter.field
			return 41747
		end
		def initialize(data = nil)
			if( data == nil )
				super(41747)
			else
				super(41747, data)
			end
		end
	end

	class NoUnderlyingComplexEventCreditEventSources < Quickfix::IntField
		def NoUnderlyingComplexEventCreditEventSources.field
			return 41748
		end
		def initialize(data = nil)
			if( data == nil )
				super(41748)
			else
				super(41748, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventSource < Quickfix::StringField
		def UnderlyingComplexEventCreditEventSource.field
			return 41749
		end
		def initialize(data = nil)
			if( data == nil )
				super(41749)
			else
				super(41749, data)
			end
		end
	end

	class UnderlyingComplexOptPayoutPaySide < Quickfix::IntField
		def UnderlyingComplexOptPayoutPaySide.field
			return 2261
		end
		def initialize(data = nil)
			if( data == nil )
				super(2261)
			else
				super(2261, data)
			end
		end
	end

	class UnderlyingComplexOptPayoutReceiveSide < Quickfix::IntField
		def UnderlyingComplexOptPayoutReceiveSide.field
			return 2262
		end
		def initialize(data = nil)
			if( data == nil )
				super(2262)
			else
				super(2262, data)
			end
		end
	end

	class UnderlyingComplexOptPayoutUnderlier < Quickfix::StringField
		def UnderlyingComplexOptPayoutUnderlier.field
			return 2263
		end
		def initialize(data = nil)
			if( data == nil )
				super(2263)
			else
				super(2263, data)
			end
		end
	end

	class UnderlyingComplexOptPayoutPercentage < Quickfix::DoubleField
		def UnderlyingComplexOptPayoutPercentage.field
			return 2264
		end
		def initialize(data = nil)
			if( data == nil )
				super(2264)
			else
				super(2264, data)
			end
		end
	end

	class UnderlyingComplexOptPayoutTime < Quickfix::IntField
		def UnderlyingComplexOptPayoutTime.field
			return 2265
		end
		def initialize(data = nil)
			if( data == nil )
				super(2265)
			else
				super(2265, data)
			end
		end
	end

	class UnderlyingComplexOptPayoutCurrency < Quickfix::StringField
		def UnderlyingComplexOptPayoutCurrency.field
			return 2266
		end
		def initialize(data = nil)
			if( data == nil )
				super(2266)
			else
				super(2266, data)
			end
		end
	end

	class UnderlyingComplexEventPricePercentage < Quickfix::DoubleField
		def UnderlyingComplexEventPricePercentage.field
			return 2267
		end
		def initialize(data = nil)
			if( data == nil )
				super(2267)
			else
				super(2267, data)
			end
		end
	end

	class UnderlyingComplexEventCurrencyOne < Quickfix::StringField
		def UnderlyingComplexEventCurrencyOne.field
			return 2268
		end
		def initialize(data = nil)
			if( data == nil )
				super(2268)
			else
				super(2268, data)
			end
		end
	end

	class UnderlyingComplexEventCurrencyTwo < Quickfix::StringField
		def UnderlyingComplexEventCurrencyTwo.field
			return 2269
		end
		def initialize(data = nil)
			if( data == nil )
				super(2269)
			else
				super(2269, data)
			end
		end
	end

	class UnderlyingComplexEventQuoteBasis < Quickfix::IntField
		def UnderlyingComplexEventQuoteBasis.field
			return 2270
		end
		def initialize(data = nil)
			if( data == nil )
				super(2270)
			else
				super(2270, data)
			end
		end
	end

	class UnderlyingComplexEventFixedFXRate < Quickfix::DoubleField
		def UnderlyingComplexEventFixedFXRate.field
			return 2271
		end
		def initialize(data = nil)
			if( data == nil )
				super(2271)
			else
				super(2271, data)
			end
		end
	end

	class UnderlyingComplexEventDeterminationMethod < Quickfix::StringField
		def UnderlyingComplexEventDeterminationMethod.field
			return 2272
		end
		def initialize(data = nil)
			if( data == nil )
				super(2272)
			else
				super(2272, data)
			end
		end
	end

	class UnderlyingComplexEventCalculationAgent < Quickfix::IntField
		def UnderlyingComplexEventCalculationAgent.field
			return 2273
		end
		def initialize(data = nil)
			if( data == nil )
				super(2273)
			else
				super(2273, data)
			end
		end
	end

	class UnderlyingComplexEventStrikePrice < Quickfix::DoubleField
		def UnderlyingComplexEventStrikePrice.field
			return 2274
		end
		def initialize(data = nil)
			if( data == nil )
				super(2274)
			else
				super(2274, data)
			end
		end
	end

	class UnderlyingComplexEventStrikeFactor < Quickfix::DoubleField
		def UnderlyingComplexEventStrikeFactor.field
			return 2275
		end
		def initialize(data = nil)
			if( data == nil )
				super(2275)
			else
				super(2275, data)
			end
		end
	end

	class UnderlyingComplexEventStrikeNumberOfOptions < Quickfix::IntField
		def UnderlyingComplexEventStrikeNumberOfOptions.field
			return 2276
		end
		def initialize(data = nil)
			if( data == nil )
				super(2276)
			else
				super(2276, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventsXIDRef < Quickfix::StringField
		def UnderlyingComplexEventCreditEventsXIDRef.field
			return 2277
		end
		def initialize(data = nil)
			if( data == nil )
				super(2277)
			else
				super(2277, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventNotifyingParty < Quickfix::IntField
		def UnderlyingComplexEventCreditEventNotifyingParty.field
			return 2278
		end
		def initialize(data = nil)
			if( data == nil )
				super(2278)
			else
				super(2278, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventBusinessCenter < Quickfix::StringField
		def UnderlyingComplexEventCreditEventBusinessCenter.field
			return 2279
		end
		def initialize(data = nil)
			if( data == nil )
				super(2279)
			else
				super(2279, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventStandardSources < Quickfix::BoolField
		def UnderlyingComplexEventCreditEventStandardSources.field
			return 2280
		end
		def initialize(data = nil)
			if( data == nil )
				super(2280)
			else
				super(2280, data)
			end
		end
	end

	class UnderlyingComplexEventCreditEventMinimumSources < Quickfix::IntField
		def UnderlyingComplexEventCreditEventMinimumSources.field
			return 2281
		end
		def initialize(data = nil)
			if( data == nil )
				super(2281)
			else
				super(2281, data)
			end
		end
	end

	class UnderlyingComplexEventXID < Quickfix::StringField
		def UnderlyingComplexEventXID.field
			return 2282
		end
		def initialize(data = nil)
			if( data == nil )
				super(2282)
			else
				super(2282, data)
			end
		end
	end

	class UnderlyingComplexEventXIDRef < Quickfix::StringField
		def UnderlyingComplexEventXIDRef.field
			return 2283
		end
		def initialize(data = nil)
			if( data == nil )
				super(2283)
			else
				super(2283, data)
			end
		end
	end

	class NoUnderlyingComplexEventSchedules < Quickfix::IntField
		def NoUnderlyingComplexEventSchedules.field
			return 41750
		end
		def initialize(data = nil)
			if( data == nil )
				super(41750)
			else
				super(41750, data)
			end
		end
	end

	class UnderlyingComplexEventScheduleStartDate < Quickfix::StringField
		def UnderlyingComplexEventScheduleStartDate.field
			return 41751
		end
		def initialize(data = nil)
			if( data == nil )
				super(41751)
			else
				super(41751, data)
			end
		end
	end

	class UnderlyingComplexEventScheduleEndDate < Quickfix::StringField
		def UnderlyingComplexEventScheduleEndDate.field
			return 41752
		end
		def initialize(data = nil)
			if( data == nil )
				super(41752)
			else
				super(41752, data)
			end
		end
	end

	class UnderlyingComplexEventScheduleFrequencyPeriod < Quickfix::IntField
		def UnderlyingComplexEventScheduleFrequencyPeriod.field
			return 41753
		end
		def initialize(data = nil)
			if( data == nil )
				super(41753)
			else
				super(41753, data)
			end
		end
	end

	class UnderlyingComplexEventScheduleFrequencyUnit < Quickfix::StringField
		def UnderlyingComplexEventScheduleFrequencyUnit.field
			return 41754
		end
		def initialize(data = nil)
			if( data == nil )
				super(41754)
			else
				super(41754, data)
			end
		end
	end

	class UnderlyingComplexEventScheduleRollConvention < Quickfix::StringField
		def UnderlyingComplexEventScheduleRollConvention.field
			return 41755
		end
		def initialize(data = nil)
			if( data == nil )
				super(41755)
			else
				super(41755, data)
			end
		end
	end

	class NoUnderlyingDeliverySchedules < Quickfix::IntField
		def NoUnderlyingDeliverySchedules.field
			return 41756
		end
		def initialize(data = nil)
			if( data == nil )
				super(41756)
			else
				super(41756, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleType < Quickfix::IntField
		def UnderlyingDeliveryScheduleType.field
			return 41757
		end
		def initialize(data = nil)
			if( data == nil )
				super(41757)
			else
				super(41757, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleXID < Quickfix::StringField
		def UnderlyingDeliveryScheduleXID.field
			return 41758
		end
		def initialize(data = nil)
			if( data == nil )
				super(41758)
			else
				super(41758, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleNotional < Quickfix::DoubleField
		def UnderlyingDeliveryScheduleNotional.field
			return 41759
		end
		def initialize(data = nil)
			if( data == nil )
				super(41759)
			else
				super(41759, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleNotionalUnitOfMeasure < Quickfix::StringField
		def UnderlyingDeliveryScheduleNotionalUnitOfMeasure.field
			return 41760
		end
		def initialize(data = nil)
			if( data == nil )
				super(41760)
			else
				super(41760, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleNotionalCommodityFrequency < Quickfix::IntField
		def UnderlyingDeliveryScheduleNotionalCommodityFrequency.field
			return 41761
		end
		def initialize(data = nil)
			if( data == nil )
				super(41761)
			else
				super(41761, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleNegativeTolerance < Quickfix::DoubleField
		def UnderlyingDeliveryScheduleNegativeTolerance.field
			return 41762
		end
		def initialize(data = nil)
			if( data == nil )
				super(41762)
			else
				super(41762, data)
			end
		end
	end

	class UnderlyingDeliverySchedulePositiveTolerance < Quickfix::DoubleField
		def UnderlyingDeliverySchedulePositiveTolerance.field
			return 41763
		end
		def initialize(data = nil)
			if( data == nil )
				super(41763)
			else
				super(41763, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleToleranceUnitOfMeasure < Quickfix::StringField
		def UnderlyingDeliveryScheduleToleranceUnitOfMeasure.field
			return 41764
		end
		def initialize(data = nil)
			if( data == nil )
				super(41764)
			else
				super(41764, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleToleranceType < Quickfix::IntField
		def UnderlyingDeliveryScheduleToleranceType.field
			return 41765
		end
		def initialize(data = nil)
			if( data == nil )
				super(41765)
			else
				super(41765, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleSettlCountry < Quickfix::StringField
		def UnderlyingDeliveryScheduleSettlCountry.field
			return 41766
		end
		def initialize(data = nil)
			if( data == nil )
				super(41766)
			else
				super(41766, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleSettlTimeZone < Quickfix::StringField
		def UnderlyingDeliveryScheduleSettlTimeZone.field
			return 41767
		end
		def initialize(data = nil)
			if( data == nil )
				super(41767)
			else
				super(41767, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleSettlFlowType < Quickfix::IntField
		def UnderlyingDeliveryScheduleSettlFlowType.field
			return 41768
		end
		def initialize(data = nil)
			if( data == nil )
				super(41768)
			else
				super(41768, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleSettlHolidaysProcessingInstruction < Quickfix::IntField
		def UnderlyingDeliveryScheduleSettlHolidaysProcessingInstruction.field
			return 41769
		end
		def initialize(data = nil)
			if( data == nil )
				super(41769)
			else
				super(41769, data)
			end
		end
	end

	class NoUnderlyingDeliveryScheduleSettlDays < Quickfix::IntField
		def NoUnderlyingDeliveryScheduleSettlDays.field
			return 41770
		end
		def initialize(data = nil)
			if( data == nil )
				super(41770)
			else
				super(41770, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleSettlDay < Quickfix::IntField
		def UnderlyingDeliveryScheduleSettlDay.field
			return 41771
		end
		def initialize(data = nil)
			if( data == nil )
				super(41771)
			else
				super(41771, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleSettlTotalHours < Quickfix::IntField
		def UnderlyingDeliveryScheduleSettlTotalHours.field
			return 41772
		end
		def initialize(data = nil)
			if( data == nil )
				super(41772)
			else
				super(41772, data)
			end
		end
	end

	class NoUnderlyingDeliveryScheduleSettlTimes < Quickfix::IntField
		def NoUnderlyingDeliveryScheduleSettlTimes.field
			return 41773
		end
		def initialize(data = nil)
			if( data == nil )
				super(41773)
			else
				super(41773, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleSettlStart < Quickfix::StringField
		def UnderlyingDeliveryScheduleSettlStart.field
			return 41774
		end
		def initialize(data = nil)
			if( data == nil )
				super(41774)
			else
				super(41774, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleSettlEnd < Quickfix::StringField
		def UnderlyingDeliveryScheduleSettlEnd.field
			return 41775
		end
		def initialize(data = nil)
			if( data == nil )
				super(41775)
			else
				super(41775, data)
			end
		end
	end

	class UnderlyingDeliveryScheduleSettlTimeType < Quickfix::IntField
		def UnderlyingDeliveryScheduleSettlTimeType.field
			return 41776
		end
		def initialize(data = nil)
			if( data == nil )
				super(41776)
			else
				super(41776, data)
			end
		end
	end

	class UnderlyingDeliveryStreamType < Quickfix::IntField
		def UnderlyingDeliveryStreamType.field
			return 41777
		end
		def initialize(data = nil)
			if( data == nil )
				super(41777)
			else
				super(41777, data)
			end
		end
	end

	class UnderlyingDeliveryStreamPipeline < Quickfix::StringField
		def UnderlyingDeliveryStreamPipeline.field
			return 41778
		end
		def initialize(data = nil)
			if( data == nil )
				super(41778)
			else
				super(41778, data)
			end
		end
	end

	class UnderlyingDeliveryStreamEntryPoint < Quickfix::StringField
		def UnderlyingDeliveryStreamEntryPoint.field
			return 41779
		end
		def initialize(data = nil)
			if( data == nil )
				super(41779)
			else
				super(41779, data)
			end
		end
	end

	class UnderlyingDeliveryStreamWithdrawalPoint < Quickfix::StringField
		def UnderlyingDeliveryStreamWithdrawalPoint.field
			return 41780
		end
		def initialize(data = nil)
			if( data == nil )
				super(41780)
			else
				super(41780, data)
			end
		end
	end

	class UnderlyingDeliveryStreamDeliveryPoint < Quickfix::StringField
		def UnderlyingDeliveryStreamDeliveryPoint.field
			return 41781
		end
		def initialize(data = nil)
			if( data == nil )
				super(41781)
			else
				super(41781, data)
			end
		end
	end

	class UnderlyingDeliveryStreamDeliveryRestriction < Quickfix::IntField
		def UnderlyingDeliveryStreamDeliveryRestriction.field
			return 41782
		end
		def initialize(data = nil)
			if( data == nil )
				super(41782)
			else
				super(41782, data)
			end
		end
	end

	class UnderlyingDeliveryStreamDeliveryContingency < Quickfix::StringField
		def UnderlyingDeliveryStreamDeliveryContingency.field
			return 41783
		end
		def initialize(data = nil)
			if( data == nil )
				super(41783)
			else
				super(41783, data)
			end
		end
	end

	class UnderlyingDeliveryStreamDeliveryContingentPartySide < Quickfix::IntField
		def UnderlyingDeliveryStreamDeliveryContingentPartySide.field
			return 41784
		end
		def initialize(data = nil)
			if( data == nil )
				super(41784)
			else
				super(41784, data)
			end
		end
	end

	class UnderlyingDeliveryStreamDeliverAtSourceIndicator < Quickfix::BoolField
		def UnderlyingDeliveryStreamDeliverAtSourceIndicator.field
			return 41785
		end
		def initialize(data = nil)
			if( data == nil )
				super(41785)
			else
				super(41785, data)
			end
		end
	end

	class UnderlyingDeliveryStreamRiskApportionment < Quickfix::StringField
		def UnderlyingDeliveryStreamRiskApportionment.field
			return 41786
		end
		def initialize(data = nil)
			if( data == nil )
				super(41786)
			else
				super(41786, data)
			end
		end
	end

	class UnderlyingDeliveryStreamRiskApportionmentSource < Quickfix::StringField
		def UnderlyingDeliveryStreamRiskApportionmentSource.field
			return 41587
		end
		def initialize(data = nil)
			if( data == nil )
				super(41587)
			else
				super(41587, data)
			end
		end
	end

	class UnderlyingDeliveryStreamTitleTransferLocation < Quickfix::StringField
		def UnderlyingDeliveryStreamTitleTransferLocation.field
			return 41787
		end
		def initialize(data = nil)
			if( data == nil )
				super(41787)
			else
				super(41787, data)
			end
		end
	end

	class UnderlyingDeliveryStreamTitleTransferCondition < Quickfix::IntField
		def UnderlyingDeliveryStreamTitleTransferCondition.field
			return 41788
		end
		def initialize(data = nil)
			if( data == nil )
				super(41788)
			else
				super(41788, data)
			end
		end
	end

	class UnderlyingDeliveryStreamImporterOfRecord < Quickfix::StringField
		def UnderlyingDeliveryStreamImporterOfRecord.field
			return 41789
		end
		def initialize(data = nil)
			if( data == nil )
				super(41789)
			else
				super(41789, data)
			end
		end
	end

	class UnderlyingDeliveryStreamNegativeTolerance < Quickfix::DoubleField
		def UnderlyingDeliveryStreamNegativeTolerance.field
			return 41790
		end
		def initialize(data = nil)
			if( data == nil )
				super(41790)
			else
				super(41790, data)
			end
		end
	end

	class UnderlyingDeliveryStreamPositiveTolerance < Quickfix::DoubleField
		def UnderlyingDeliveryStreamPositiveTolerance.field
			return 41791
		end
		def initialize(data = nil)
			if( data == nil )
				super(41791)
			else
				super(41791, data)
			end
		end
	end

	class UnderlyingDeliveryStreamToleranceUnitOfMeasure < Quickfix::StringField
		def UnderlyingDeliveryStreamToleranceUnitOfMeasure.field
			return 41792
		end
		def initialize(data = nil)
			if( data == nil )
				super(41792)
			else
				super(41792, data)
			end
		end
	end

	class UnderlyingDeliveryStreamToleranceType < Quickfix::IntField
		def UnderlyingDeliveryStreamToleranceType.field
			return 41793
		end
		def initialize(data = nil)
			if( data == nil )
				super(41793)
			else
				super(41793, data)
			end
		end
	end

	class UnderlyingDeliveryStreamToleranceOptionSide < Quickfix::IntField
		def UnderlyingDeliveryStreamToleranceOptionSide.field
			return 41794
		end
		def initialize(data = nil)
			if( data == nil )
				super(41794)
			else
				super(41794, data)
			end
		end
	end

	class UnderlyingDeliveryStreamTotalPositiveTolerance < Quickfix::DoubleField
		def UnderlyingDeliveryStreamTotalPositiveTolerance.field
			return 41795
		end
		def initialize(data = nil)
			if( data == nil )
				super(41795)
			else
				super(41795, data)
			end
		end
	end

	class UnderlyingDeliveryStreamTotalNegativeTolerance < Quickfix::DoubleField
		def UnderlyingDeliveryStreamTotalNegativeTolerance.field
			return 41796
		end
		def initialize(data = nil)
			if( data == nil )
				super(41796)
			else
				super(41796, data)
			end
		end
	end

	class UnderlyingDeliveryStreamNotionalConversionFactor < Quickfix::DoubleField
		def UnderlyingDeliveryStreamNotionalConversionFactor.field
			return 41797
		end
		def initialize(data = nil)
			if( data == nil )
				super(41797)
			else
				super(41797, data)
			end
		end
	end

	class UnderlyingDeliveryStreamTransportEquipment < Quickfix::StringField
		def UnderlyingDeliveryStreamTransportEquipment.field
			return 41798
		end
		def initialize(data = nil)
			if( data == nil )
				super(41798)
			else
				super(41798, data)
			end
		end
	end

	class UnderlyingDeliveryStreamElectingPartySide < Quickfix::IntField
		def UnderlyingDeliveryStreamElectingPartySide.field
			return 41799
		end
		def initialize(data = nil)
			if( data == nil )
				super(41799)
			else
				super(41799, data)
			end
		end
	end

	class NoUnderlyingStreamAssetAttributes < Quickfix::IntField
		def NoUnderlyingStreamAssetAttributes.field
			return 41800
		end
		def initialize(data = nil)
			if( data == nil )
				super(41800)
			else
				super(41800, data)
			end
		end
	end

	class UnderlyingStreamAssetAttributeType < Quickfix::StringField
		def UnderlyingStreamAssetAttributeType.field
			return 41801
		end
		def initialize(data = nil)
			if( data == nil )
				super(41801)
			else
				super(41801, data)
			end
		end
	end

	class UnderlyingStreamAssetAttributeValue < Quickfix::StringField
		def UnderlyingStreamAssetAttributeValue.field
			return 41802
		end
		def initialize(data = nil)
			if( data == nil )
				super(41802)
			else
				super(41802, data)
			end
		end
	end

	class UnderlyingStreamAssetAttributeLimit < Quickfix::StringField
		def UnderlyingStreamAssetAttributeLimit.field
			return 41803
		end
		def initialize(data = nil)
			if( data == nil )
				super(41803)
			else
				super(41803, data)
			end
		end
	end

	class NoUnderlyingDeliveryStreamCycles < Quickfix::IntField
		def NoUnderlyingDeliveryStreamCycles.field
			return 41804
		end
		def initialize(data = nil)
			if( data == nil )
				super(41804)
			else
				super(41804, data)
			end
		end
	end

	class UnderlyingDeliveryStreamCycleDesc < Quickfix::StringField
		def UnderlyingDeliveryStreamCycleDesc.field
			return 41805
		end
		def initialize(data = nil)
			if( data == nil )
				super(41805)
			else
				super(41805, data)
			end
		end
	end

	class EncodedUnderlyingDeliveryStreamCycleDescLen < Quickfix::IntField
		def EncodedUnderlyingDeliveryStreamCycleDescLen.field
			return 41806
		end
		def initialize(data = nil)
			if( data == nil )
				super(41806)
			else
				super(41806, data)
			end
		end
	end

	class EncodedUnderlyingDeliveryStreamCycleDesc < Quickfix::StringField
		def EncodedUnderlyingDeliveryStreamCycleDesc.field
			return 41807
		end
		def initialize(data = nil)
			if( data == nil )
				super(41807)
			else
				super(41807, data)
			end
		end
	end

	class NoUnderlyingDeliveryStreamCommoditySources < Quickfix::IntField
		def NoUnderlyingDeliveryStreamCommoditySources.field
			return 41808
		end
		def initialize(data = nil)
			if( data == nil )
				super(41808)
			else
				super(41808, data)
			end
		end
	end

	class UnderlyingDeliveryStreamCommoditySource < Quickfix::StringField
		def UnderlyingDeliveryStreamCommoditySource.field
			return 41809
		end
		def initialize(data = nil)
			if( data == nil )
				super(41809)
			else
				super(41809, data)
			end
		end
	end

	class UnderlyingExerciseDesc < Quickfix::StringField
		def UnderlyingExerciseDesc.field
			return 41810
		end
		def initialize(data = nil)
			if( data == nil )
				super(41810)
			else
				super(41810, data)
			end
		end
	end

	class EncodedUnderlyingExerciseDescLen < Quickfix::IntField
		def EncodedUnderlyingExerciseDescLen.field
			return 41811
		end
		def initialize(data = nil)
			if( data == nil )
				super(41811)
			else
				super(41811, data)
			end
		end
	end

	class EncodedUnderlyingExerciseDesc < Quickfix::StringField
		def EncodedUnderlyingExerciseDesc.field
			return 41812
		end
		def initialize(data = nil)
			if( data == nil )
				super(41812)
			else
				super(41812, data)
			end
		end
	end

	class UnderlyingAutomaticExerciseIndicator < Quickfix::BoolField
		def UnderlyingAutomaticExerciseIndicator.field
			return 41813
		end
		def initialize(data = nil)
			if( data == nil )
				super(41813)
			else
				super(41813, data)
			end
		end
	end

	class UnderlyingAutomaticExerciseThresholdRate < Quickfix::DoubleField
		def UnderlyingAutomaticExerciseThresholdRate.field
			return 41814
		end
		def initialize(data = nil)
			if( data == nil )
				super(41814)
			else
				super(41814, data)
			end
		end
	end

	class UnderlyingExerciseConfirmationMethod < Quickfix::IntField
		def UnderlyingExerciseConfirmationMethod.field
			return 41815
		end
		def initialize(data = nil)
			if( data == nil )
				super(41815)
			else
				super(41815, data)
			end
		end
	end

	class UnderlyingManualNoticeBusinessCenter < Quickfix::StringField
		def UnderlyingManualNoticeBusinessCenter.field
			return 41816
		end
		def initialize(data = nil)
			if( data == nil )
				super(41816)
			else
				super(41816, data)
			end
		end
	end

	class UnderlyingFallbackExerciseIndicator < Quickfix::BoolField
		def UnderlyingFallbackExerciseIndicator.field
			return 41817
		end
		def initialize(data = nil)
			if( data == nil )
				super(41817)
			else
				super(41817, data)
			end
		end
	end

	class UnderlyingLimitedRightToConfirmIndicator < Quickfix::BoolField
		def UnderlyingLimitedRightToConfirmIndicator.field
			return 41818
		end
		def initialize(data = nil)
			if( data == nil )
				super(41818)
			else
				super(41818, data)
			end
		end
	end

	class UnderlyingExerciseSplitTicketIndicator < Quickfix::BoolField
		def UnderlyingExerciseSplitTicketIndicator.field
			return 41819
		end
		def initialize(data = nil)
			if( data == nil )
				super(41819)
			else
				super(41819, data)
			end
		end
	end

	class NoUnderlyingOptionExerciseBusinessCenters < Quickfix::IntField
		def NoUnderlyingOptionExerciseBusinessCenters.field
			return 41820
		end
		def initialize(data = nil)
			if( data == nil )
				super(41820)
			else
				super(41820, data)
			end
		end
	end

	class UnderlyingOptionExerciseBusinessCenter < Quickfix::StringField
		def UnderlyingOptionExerciseBusinessCenter.field
			return 41821
		end
		def initialize(data = nil)
			if( data == nil )
				super(41821)
			else
				super(41821, data)
			end
		end
	end

	class UnderlyingOptionExerciseBusinessDayConvention < Quickfix::IntField
		def UnderlyingOptionExerciseBusinessDayConvention.field
			return 41822
		end
		def initialize(data = nil)
			if( data == nil )
				super(41822)
			else
				super(41822, data)
			end
		end
	end

	class UnderlyingOptionExerciseEarliestDateOffsetDayType < Quickfix::IntField
		def UnderlyingOptionExerciseEarliestDateOffsetDayType.field
			return 41823
		end
		def initialize(data = nil)
			if( data == nil )
				super(41823)
			else
				super(41823, data)
			end
		end
	end

	class UnderlyingOptionExerciseEarliestDateOffsetPeriod < Quickfix::IntField
		def UnderlyingOptionExerciseEarliestDateOffsetPeriod.field
			return 41824
		end
		def initialize(data = nil)
			if( data == nil )
				super(41824)
			else
				super(41824, data)
			end
		end
	end

	class UnderlyingOptionExerciseEarliestDateOffsetUnit < Quickfix::StringField
		def UnderlyingOptionExerciseEarliestDateOffsetUnit.field
			return 41825
		end
		def initialize(data = nil)
			if( data == nil )
				super(41825)
			else
				super(41825, data)
			end
		end
	end

	class UnderlyingOptionExerciseFrequencyPeriod < Quickfix::IntField
		def UnderlyingOptionExerciseFrequencyPeriod.field
			return 41826
		end
		def initialize(data = nil)
			if( data == nil )
				super(41826)
			else
				super(41826, data)
			end
		end
	end

	class UnderlyingOptionExerciseFrequencyUnit < Quickfix::StringField
		def UnderlyingOptionExerciseFrequencyUnit.field
			return 41827
		end
		def initialize(data = nil)
			if( data == nil )
				super(41827)
			else
				super(41827, data)
			end
		end
	end

	class UnderlyingOptionExerciseStartDateUnadjusted < Quickfix::StringField
		def UnderlyingOptionExerciseStartDateUnadjusted.field
			return 41828
		end
		def initialize(data = nil)
			if( data == nil )
				super(41828)
			else
				super(41828, data)
			end
		end
	end

	class UnderlyingOptionExerciseStartDateRelativeTo < Quickfix::IntField
		def UnderlyingOptionExerciseStartDateRelativeTo.field
			return 41829
		end
		def initialize(data = nil)
			if( data == nil )
				super(41829)
			else
				super(41829, data)
			end
		end
	end

	class UnderlyingOptionExerciseStartDateOffsetPeriod < Quickfix::IntField
		def UnderlyingOptionExerciseStartDateOffsetPeriod.field
			return 41830
		end
		def initialize(data = nil)
			if( data == nil )
				super(41830)
			else
				super(41830, data)
			end
		end
	end

	class UnderlyingOptionExerciseStartDateOffsetUnit < Quickfix::StringField
		def UnderlyingOptionExerciseStartDateOffsetUnit.field
			return 41831
		end
		def initialize(data = nil)
			if( data == nil )
				super(41831)
			else
				super(41831, data)
			end
		end
	end

	class UnderlyingOptionExerciseStartDateOffsetDayType < Quickfix::IntField
		def UnderlyingOptionExerciseStartDateOffsetDayType.field
			return 41832
		end
		def initialize(data = nil)
			if( data == nil )
				super(41832)
			else
				super(41832, data)
			end
		end
	end

	class UnderlyingOptionExerciseStartDateAdjusted < Quickfix::StringField
		def UnderlyingOptionExerciseStartDateAdjusted.field
			return 41833
		end
		def initialize(data = nil)
			if( data == nil )
				super(41833)
			else
				super(41833, data)
			end
		end
	end

	class UnderlyingOptionExerciseSkip < Quickfix::IntField
		def UnderlyingOptionExerciseSkip.field
			return 41834
		end
		def initialize(data = nil)
			if( data == nil )
				super(41834)
			else
				super(41834, data)
			end
		end
	end

	class UnderlyingOptionExerciseNominationDeadline < Quickfix::StringField
		def UnderlyingOptionExerciseNominationDeadline.field
			return 41835
		end
		def initialize(data = nil)
			if( data == nil )
				super(41835)
			else
				super(41835, data)
			end
		end
	end

	class UnderlyingOptionExerciseFirstDateUnadjusted < Quickfix::StringField
		def UnderlyingOptionExerciseFirstDateUnadjusted.field
			return 41836
		end
		def initialize(data = nil)
			if( data == nil )
				super(41836)
			else
				super(41836, data)
			end
		end
	end

	class UnderlyingOptionExerciseLastDateUnadjusted < Quickfix::StringField
		def UnderlyingOptionExerciseLastDateUnadjusted.field
			return 41837
		end
		def initialize(data = nil)
			if( data == nil )
				super(41837)
			else
				super(41837, data)
			end
		end
	end

	class UnderlyingOptionExerciseEarliestTime < Quickfix::StringField
		def UnderlyingOptionExerciseEarliestTime.field
			return 41838
		end
		def initialize(data = nil)
			if( data == nil )
				super(41838)
			else
				super(41838, data)
			end
		end
	end

	class UnderlyingOptionExerciseLatestTime < Quickfix::StringField
		def UnderlyingOptionExerciseLatestTime.field
			return 41839
		end
		def initialize(data = nil)
			if( data == nil )
				super(41839)
			else
				super(41839, data)
			end
		end
	end

	class UnderlyingOptionExerciseTimeBusinessCenter < Quickfix::StringField
		def UnderlyingOptionExerciseTimeBusinessCenter.field
			return 41840
		end
		def initialize(data = nil)
			if( data == nil )
				super(41840)
			else
				super(41840, data)
			end
		end
	end

	class NoUnderlyingOptionExerciseDates < Quickfix::IntField
		def NoUnderlyingOptionExerciseDates.field
			return 41841
		end
		def initialize(data = nil)
			if( data == nil )
				super(41841)
			else
				super(41841, data)
			end
		end
	end

	class UnderlyingOptionExerciseDate < Quickfix::StringField
		def UnderlyingOptionExerciseDate.field
			return 41842
		end
		def initialize(data = nil)
			if( data == nil )
				super(41842)
			else
				super(41842, data)
			end
		end
	end

	class UnderlyingOptionExerciseDateType < Quickfix::IntField
		def UnderlyingOptionExerciseDateType.field
			return 41843
		end
		def initialize(data = nil)
			if( data == nil )
				super(41843)
			else
				super(41843, data)
			end
		end
	end

	class NoUnderlyingOptionExerciseExpirationDateBusinessCenters < Quickfix::IntField
		def NoUnderlyingOptionExerciseExpirationDateBusinessCenters.field
			return 41844
		end
		def initialize(data = nil)
			if( data == nil )
				super(41844)
			else
				super(41844, data)
			end
		end
	end

	class UnderlyingOptionExerciseExpirationDateBusinessCenter < Quickfix::StringField
		def UnderlyingOptionExerciseExpirationDateBusinessCenter.field
			return 41845
		end
		def initialize(data = nil)
			if( data == nil )
				super(41845)
			else
				super(41845, data)
			end
		end
	end

	class UnderlyingOptionExerciseExpirationDateBusinessDayConvention < Quickfix::IntField
		def UnderlyingOptionExerciseExpirationDateBusinessDayConvention.field
			return 41846
		end
		def initialize(data = nil)
			if( data == nil )
				super(41846)
			else
				super(41846, data)
			end
		end
	end

	class UnderlyingOptionExerciseExpirationDateRelativeTo < Quickfix::IntField
		def UnderlyingOptionExerciseExpirationDateRelativeTo.field
			return 41847
		end
		def initialize(data = nil)
			if( data == nil )
				super(41847)
			else
				super(41847, data)
			end
		end
	end

	class UnderlyingOptionExerciseExpirationDateOffsetPeriod < Quickfix::IntField
		def UnderlyingOptionExerciseExpirationDateOffsetPeriod.field
			return 41848
		end
		def initialize(data = nil)
			if( data == nil )
				super(41848)
			else
				super(41848, data)
			end
		end
	end

	class UnderlyingOptionExerciseExpirationDateOffsetUnit < Quickfix::StringField
		def UnderlyingOptionExerciseExpirationDateOffsetUnit.field
			return 41849
		end
		def initialize(data = nil)
			if( data == nil )
				super(41849)
			else
				super(41849, data)
			end
		end
	end

	class UnderlyingOptionExerciseExpirationFrequencyPeriod < Quickfix::IntField
		def UnderlyingOptionExerciseExpirationFrequencyPeriod.field
			return 41850
		end
		def initialize(data = nil)
			if( data == nil )
				super(41850)
			else
				super(41850, data)
			end
		end
	end

	class UnderlyingOptionExerciseExpirationFrequencyUnit < Quickfix::StringField
		def UnderlyingOptionExerciseExpirationFrequencyUnit.field
			return 41851
		end
		def initialize(data = nil)
			if( data == nil )
				super(41851)
			else
				super(41851, data)
			end
		end
	end

	class UnderlyingOptionExerciseExpirationRollConvention < Quickfix::StringField
		def UnderlyingOptionExerciseExpirationRollConvention.field
			return 41852
		end
		def initialize(data = nil)
			if( data == nil )
				super(41852)
			else
				super(41852, data)
			end
		end
	end

	class UnderlyingOptionExerciseExpirationDateOffsetDayType < Quickfix::IntField
		def UnderlyingOptionExerciseExpirationDateOffsetDayType.field
			return 41853
		end
		def initialize(data = nil)
			if( data == nil )
				super(41853)
			else
				super(41853, data)
			end
		end
	end

	class UnderlyingOptionExerciseExpirationTime < Quickfix::StringField
		def UnderlyingOptionExerciseExpirationTime.field
			return 41854
		end
		def initialize(data = nil)
			if( data == nil )
				super(41854)
			else
				super(41854, data)
			end
		end
	end

	class UnderlyingOptionExerciseExpirationTimeBusinessCenter < Quickfix::StringField
		def UnderlyingOptionExerciseExpirationTimeBusinessCenter.field
			return 41855
		end
		def initialize(data = nil)
			if( data == nil )
				super(41855)
			else
				super(41855, data)
			end
		end
	end

	class NoUnderlyingOptionExerciseExpirationDates < Quickfix::IntField
		def NoUnderlyingOptionExerciseExpirationDates.field
			return 41856
		end
		def initialize(data = nil)
			if( data == nil )
				super(41856)
			else
				super(41856, data)
			end
		end
	end

	class UnderlyingOptionExerciseExpirationDate < Quickfix::StringField
		def UnderlyingOptionExerciseExpirationDate.field
			return 41857
		end
		def initialize(data = nil)
			if( data == nil )
				super(41857)
			else
				super(41857, data)
			end
		end
	end

	class UnderlyingOptionExerciseExpirationDateType < Quickfix::IntField
		def UnderlyingOptionExerciseExpirationDateType.field
			return 41858
		end
		def initialize(data = nil)
			if( data == nil )
				super(41858)
			else
				super(41858, data)
			end
		end
	end

	class UnderlyingSettlRateIndex < Quickfix::StringField
		def UnderlyingSettlRateIndex.field
			return 2284
		end
		def initialize(data = nil)
			if( data == nil )
				super(2284)
			else
				super(2284, data)
			end
		end
	end

	class UnderlyingSettlRateIndexLocation < Quickfix::StringField
		def UnderlyingSettlRateIndexLocation.field
			return 2285
		end
		def initialize(data = nil)
			if( data == nil )
				super(2285)
			else
				super(2285, data)
			end
		end
	end

	class UnderlyingOptionExpirationDesc < Quickfix::StringField
		def UnderlyingOptionExpirationDesc.field
			return 2286
		end
		def initialize(data = nil)
			if( data == nil )
				super(2286)
			else
				super(2286, data)
			end
		end
	end

	class EncodedUnderlyingOptionExpirationDescLen < Quickfix::IntField
		def EncodedUnderlyingOptionExpirationDescLen.field
			return 2287
		end
		def initialize(data = nil)
			if( data == nil )
				super(2287)
			else
				super(2287, data)
			end
		end
	end

	class EncodedUnderlyingOptionExpirationDesc < Quickfix::StringField
		def EncodedUnderlyingOptionExpirationDesc.field
			return 2288
		end
		def initialize(data = nil)
			if( data == nil )
				super(2288)
			else
				super(2288, data)
			end
		end
	end

	class UnderlyingSwapSubClass < Quickfix::StringField
		def UnderlyingSwapSubClass.field
			return 2289
		end
		def initialize(data = nil)
			if( data == nil )
				super(2289)
			else
				super(2289, data)
			end
		end
	end

	class UnderlyingStrikeUnitOfMeasure < Quickfix::StringField
		def UnderlyingStrikeUnitOfMeasure.field
			return 2290
		end
		def initialize(data = nil)
			if( data == nil )
				super(2290)
			else
				super(2290, data)
			end
		end
	end

	class UnderlyingStrikeIndex < Quickfix::StringField
		def UnderlyingStrikeIndex.field
			return 2291
		end
		def initialize(data = nil)
			if( data == nil )
				super(2291)
			else
				super(2291, data)
			end
		end
	end

	class UnderlyingStrikeIndexSpread < Quickfix::DoubleField
		def UnderlyingStrikeIndexSpread.field
			return 2292
		end
		def initialize(data = nil)
			if( data == nil )
				super(2292)
			else
				super(2292, data)
			end
		end
	end

	class UnderlyingValuationSource < Quickfix::StringField
		def UnderlyingValuationSource.field
			return 2293
		end
		def initialize(data = nil)
			if( data == nil )
				super(2293)
			else
				super(2293, data)
			end
		end
	end

	class UnderlyingValuationReferenceModel < Quickfix::StringField
		def UnderlyingValuationReferenceModel.field
			return 2294
		end
		def initialize(data = nil)
			if( data == nil )
				super(2294)
			else
				super(2294, data)
			end
		end
	end

	class UnderlyingStrategyType < Quickfix::StringField
		def UnderlyingStrategyType.field
			return 2295
		end
		def initialize(data = nil)
			if( data == nil )
				super(2295)
			else
				super(2295, data)
			end
		end
	end

	class UnderlyingCommonPricingIndicator < Quickfix::BoolField
		def UnderlyingCommonPricingIndicator.field
			return 2296
		end
		def initialize(data = nil)
			if( data == nil )
				super(2296)
			else
				super(2296, data)
			end
		end
	end

	class UnderlyingSettlDisruptionProvision < Quickfix::IntField
		def UnderlyingSettlDisruptionProvision.field
			return 2297
		end
		def initialize(data = nil)
			if( data == nil )
				super(2297)
			else
				super(2297, data)
			end
		end
	end

	class UnderlyingInstrumentRoundingDirection < Quickfix::CharField
		def UnderlyingInstrumentRoundingDirection.field
			return 2298
		end
		def initialize(data = nil)
			if( data == nil )
				super(2298)
			else
				super(2298, data)
			end
		end
	end

	class UnderlyingInstrumentRoundingPrecision < Quickfix::IntField
		def UnderlyingInstrumentRoundingPrecision.field
			return 2299
		end
		def initialize(data = nil)
			if( data == nil )
				super(2299)
			else
				super(2299, data)
			end
		end
	end

	class UnderlyingMarketDisruptionProvision < Quickfix::IntField
		def UnderlyingMarketDisruptionProvision.field
			return 41859
		end
		def initialize(data = nil)
			if( data == nil )
				super(41859)
			else
				super(41859, data)
			end
		end
	end

	class UnderlyingMarketDisruptionFallbackProvision < Quickfix::IntField
		def UnderlyingMarketDisruptionFallbackProvision.field
			return 41860
		end
		def initialize(data = nil)
			if( data == nil )
				super(41860)
			else
				super(41860, data)
			end
		end
	end

	class UnderlyingMarketDisruptionMaximumDays < Quickfix::IntField
		def UnderlyingMarketDisruptionMaximumDays.field
			return 41861
		end
		def initialize(data = nil)
			if( data == nil )
				super(41861)
			else
				super(41861, data)
			end
		end
	end

	class UnderlyingMarketDisruptionMaterialityPercentage < Quickfix::DoubleField
		def UnderlyingMarketDisruptionMaterialityPercentage.field
			return 41862
		end
		def initialize(data = nil)
			if( data == nil )
				super(41862)
			else
				super(41862, data)
			end
		end
	end

	class UnderlyingMarketDisruptionMinimumFuturesContracts < Quickfix::IntField
		def UnderlyingMarketDisruptionMinimumFuturesContracts.field
			return 41863
		end
		def initialize(data = nil)
			if( data == nil )
				super(41863)
			else
				super(41863, data)
			end
		end
	end

	class NoUnderlyingMarketDisruptionEvents < Quickfix::IntField
		def NoUnderlyingMarketDisruptionEvents.field
			return 41864
		end
		def initialize(data = nil)
			if( data == nil )
				super(41864)
			else
				super(41864, data)
			end
		end
	end

	class UnderlyingMarketDisruptionEvent < Quickfix::StringField
		def UnderlyingMarketDisruptionEvent.field
			return 41865
		end
		def initialize(data = nil)
			if( data == nil )
				super(41865)
			else
				super(41865, data)
			end
		end
	end

	class NoUnderlyingMarketDisruptionFallbacks < Quickfix::IntField
		def NoUnderlyingMarketDisruptionFallbacks.field
			return 41866
		end
		def initialize(data = nil)
			if( data == nil )
				super(41866)
			else
				super(41866, data)
			end
		end
	end

	class UnderlyingMarketDisruptionFallbackType < Quickfix::StringField
		def UnderlyingMarketDisruptionFallbackType.field
			return 41867
		end
		def initialize(data = nil)
			if( data == nil )
				super(41867)
			else
				super(41867, data)
			end
		end
	end

	class NoUnderlyingMarketDisruptionFallbackReferencePrices < Quickfix::IntField
		def NoUnderlyingMarketDisruptionFallbackReferencePrices.field
			return 41868
		end
		def initialize(data = nil)
			if( data == nil )
				super(41868)
			else
				super(41868, data)
			end
		end
	end

	class UnderlyingMarketDisruptionFallbackUnderlierType < Quickfix::IntField
		def UnderlyingMarketDisruptionFallbackUnderlierType.field
			return 41869
		end
		def initialize(data = nil)
			if( data == nil )
				super(41869)
			else
				super(41869, data)
			end
		end
	end

	class UnderlyingMarketDisruptionFallbackUnderlierSecurityID < Quickfix::StringField
		def UnderlyingMarketDisruptionFallbackUnderlierSecurityID.field
			return 41870
		end
		def initialize(data = nil)
			if( data == nil )
				super(41870)
			else
				super(41870, data)
			end
		end
	end

	class UnderlyingMarketDisruptionFallbackUnderlierSecurityIDSource < Quickfix::StringField
		def UnderlyingMarketDisruptionFallbackUnderlierSecurityIDSource.field
			return 41871
		end
		def initialize(data = nil)
			if( data == nil )
				super(41871)
			else
				super(41871, data)
			end
		end
	end

	class UnderlyingMarketDisruptionFallbackUnderlierSecurityDesc < Quickfix::StringField
		def UnderlyingMarketDisruptionFallbackUnderlierSecurityDesc.field
			return 41872
		end
		def initialize(data = nil)
			if( data == nil )
				super(41872)
			else
				super(41872, data)
			end
		end
	end

	class EncodedUnderlyingMarketDisruptionFallbackUnderlierSecDescLen < Quickfix::IntField
		def EncodedUnderlyingMarketDisruptionFallbackUnderlierSecDescLen.field
			return 41873
		end
		def initialize(data = nil)
			if( data == nil )
				super(41873)
			else
				super(41873, data)
			end
		end
	end

	class EncodedUnderlyingMarketDisruptionFallbackUnderlierSecurityDesc < Quickfix::StringField
		def EncodedUnderlyingMarketDisruptionFallbackUnderlierSecurityDesc.field
			return 41874
		end
		def initialize(data = nil)
			if( data == nil )
				super(41874)
			else
				super(41874, data)
			end
		end
	end

	class UnderlyingMarketDisruptionFallbackOpenUnits < Quickfix::DoubleField
		def UnderlyingMarketDisruptionFallbackOpenUnits.field
			return 41875
		end
		def initialize(data = nil)
			if( data == nil )
				super(41875)
			else
				super(41875, data)
			end
		end
	end

	class UnderlyingMarketDisruptionFallbackBasketCurrency < Quickfix::StringField
		def UnderlyingMarketDisruptionFallbackBasketCurrency.field
			return 41876
		end
		def initialize(data = nil)
			if( data == nil )
				super(41876)
			else
				super(41876, data)
			end
		end
	end

	class UnderlyingMarketDisruptionFallbackBasketDivisor < Quickfix::DoubleField
		def UnderlyingMarketDisruptionFallbackBasketDivisor.field
			return 41877
		end
		def initialize(data = nil)
			if( data == nil )
				super(41877)
			else
				super(41877, data)
			end
		end
	end

	class NoUnderlyingPaymentScheduleFixingDays < Quickfix::IntField
		def NoUnderlyingPaymentScheduleFixingDays.field
			return 41878
		end
		def initialize(data = nil)
			if( data == nil )
				super(41878)
			else
				super(41878, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingDayOfWeek < Quickfix::IntField
		def UnderlyingPaymentScheduleFixingDayOfWeek.field
			return 41879
		end
		def initialize(data = nil)
			if( data == nil )
				super(41879)
			else
				super(41879, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingDayNumber < Quickfix::IntField
		def UnderlyingPaymentScheduleFixingDayNumber.field
			return 41880
		end
		def initialize(data = nil)
			if( data == nil )
				super(41880)
			else
				super(41880, data)
			end
		end
	end

	class UnderlyingPaymentScheduleXID < Quickfix::StringField
		def UnderlyingPaymentScheduleXID.field
			return 41881
		end
		def initialize(data = nil)
			if( data == nil )
				super(41881)
			else
				super(41881, data)
			end
		end
	end

	class UnderlyingPaymentScheduleXIDRef < Quickfix::StringField
		def UnderlyingPaymentScheduleXIDRef.field
			return 41882
		end
		def initialize(data = nil)
			if( data == nil )
				super(41882)
			else
				super(41882, data)
			end
		end
	end

	class UnderlyingPaymentScheduleRateCurrency < Quickfix::StringField
		def UnderlyingPaymentScheduleRateCurrency.field
			return 41883
		end
		def initialize(data = nil)
			if( data == nil )
				super(41883)
			else
				super(41883, data)
			end
		end
	end

	class UnderlyingPaymentScheduleRateUnitOfMeasure < Quickfix::StringField
		def UnderlyingPaymentScheduleRateUnitOfMeasure.field
			return 41884
		end
		def initialize(data = nil)
			if( data == nil )
				super(41884)
			else
				super(41884, data)
			end
		end
	end

	class UnderlyingPaymentScheduleRateConversionFactor < Quickfix::DoubleField
		def UnderlyingPaymentScheduleRateConversionFactor.field
			return 41885
		end
		def initialize(data = nil)
			if( data == nil )
				super(41885)
			else
				super(41885, data)
			end
		end
	end

	class UnderlyingPaymentScheduleRateSpreadType < Quickfix::IntField
		def UnderlyingPaymentScheduleRateSpreadType.field
			return 41886
		end
		def initialize(data = nil)
			if( data == nil )
				super(41886)
			else
				super(41886, data)
			end
		end
	end

	class UnderlyingPaymentScheduleSettlPeriodPrice < Quickfix::DoubleField
		def UnderlyingPaymentScheduleSettlPeriodPrice.field
			return 41887
		end
		def initialize(data = nil)
			if( data == nil )
				super(41887)
			else
				super(41887, data)
			end
		end
	end

	class UnderlyingPaymentScheduleSettlPeriodPriceCurrency < Quickfix::StringField
		def UnderlyingPaymentScheduleSettlPeriodPriceCurrency.field
			return 41888
		end
		def initialize(data = nil)
			if( data == nil )
				super(41888)
			else
				super(41888, data)
			end
		end
	end

	class UnderlyingPaymentScheduleSettlPeriodPriceUnitOfMeasure < Quickfix::StringField
		def UnderlyingPaymentScheduleSettlPeriodPriceUnitOfMeasure.field
			return 41889
		end
		def initialize(data = nil)
			if( data == nil )
				super(41889)
			else
				super(41889, data)
			end
		end
	end

	class UnderlyingPaymentScheduleStepUnitOfMeasure < Quickfix::StringField
		def UnderlyingPaymentScheduleStepUnitOfMeasure.field
			return 41890
		end
		def initialize(data = nil)
			if( data == nil )
				super(41890)
			else
				super(41890, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingDayDistribution < Quickfix::IntField
		def UnderlyingPaymentScheduleFixingDayDistribution.field
			return 41891
		end
		def initialize(data = nil)
			if( data == nil )
				super(41891)
			else
				super(41891, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingDayCount < Quickfix::IntField
		def UnderlyingPaymentScheduleFixingDayCount.field
			return 41892
		end
		def initialize(data = nil)
			if( data == nil )
				super(41892)
			else
				super(41892, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingLagPeriod < Quickfix::IntField
		def UnderlyingPaymentScheduleFixingLagPeriod.field
			return 41893
		end
		def initialize(data = nil)
			if( data == nil )
				super(41893)
			else
				super(41893, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingLagUnit < Quickfix::StringField
		def UnderlyingPaymentScheduleFixingLagUnit.field
			return 41894
		end
		def initialize(data = nil)
			if( data == nil )
				super(41894)
			else
				super(41894, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingFirstObservationDateOffsetPeriod < Quickfix::IntField
		def UnderlyingPaymentScheduleFixingFirstObservationDateOffsetPeriod.field
			return 41895
		end
		def initialize(data = nil)
			if( data == nil )
				super(41895)
			else
				super(41895, data)
			end
		end
	end

	class UnderlyingPaymentScheduleFixingFirstObservationDateOffsetUnit < Quickfix::StringField
		def UnderlyingPaymentScheduleFixingFirstObservationDateOffsetUnit.field
			return 41896
		end
		def initialize(data = nil)
			if( data == nil )
				super(41896)
			else
				super(41896, data)
			end
		end
	end

	class UnderlyingPaymentStreamFlatRateIndicator < Quickfix::BoolField
		def UnderlyingPaymentStreamFlatRateIndicator.field
			return 41897
		end
		def initialize(data = nil)
			if( data == nil )
				super(41897)
			else
				super(41897, data)
			end
		end
	end

	class UnderlyingPaymentStreamFlatRateAmount < Quickfix::DoubleField
		def UnderlyingPaymentStreamFlatRateAmount.field
			return 41898
		end
		def initialize(data = nil)
			if( data == nil )
				super(41898)
			else
				super(41898, data)
			end
		end
	end

	class UnderlyingPaymentStreamFlatRateCurrency < Quickfix::StringField
		def UnderlyingPaymentStreamFlatRateCurrency.field
			return 41899
		end
		def initialize(data = nil)
			if( data == nil )
				super(41899)
			else
				super(41899, data)
			end
		end
	end

	class UnderlyingPaymentStreamMaximumPaymentAmount < Quickfix::DoubleField
		def UnderlyingPaymentStreamMaximumPaymentAmount.field
			return 41900
		end
		def initialize(data = nil)
			if( data == nil )
				super(41900)
			else
				super(41900, data)
			end
		end
	end

	class UnderlyingPaymentStreamMaximumPaymentCurrency < Quickfix::StringField
		def UnderlyingPaymentStreamMaximumPaymentCurrency.field
			return 41901
		end
		def initialize(data = nil)
			if( data == nil )
				super(41901)
			else
				super(41901, data)
			end
		end
	end

	class UnderlyingPaymentStreamMaximumTransactionAmount < Quickfix::DoubleField
		def UnderlyingPaymentStreamMaximumTransactionAmount.field
			return 41902
		end
		def initialize(data = nil)
			if( data == nil )
				super(41902)
			else
				super(41902, data)
			end
		end
	end

	class UnderlyingPaymentStreamMaximumTransactionCurrency < Quickfix::StringField
		def UnderlyingPaymentStreamMaximumTransactionCurrency.field
			return 41903
		end
		def initialize(data = nil)
			if( data == nil )
				super(41903)
			else
				super(41903, data)
			end
		end
	end

	class UnderlyingPaymentStreamFixedAmountUnitOfMeasure < Quickfix::StringField
		def UnderlyingPaymentStreamFixedAmountUnitOfMeasure.field
			return 41904
		end
		def initialize(data = nil)
			if( data == nil )
				super(41904)
			else
				super(41904, data)
			end
		end
	end

	class UnderlyingPaymentStreamTotalFixedAmount < Quickfix::DoubleField
		def UnderlyingPaymentStreamTotalFixedAmount.field
			return 41905
		end
		def initialize(data = nil)
			if( data == nil )
				super(41905)
			else
				super(41905, data)
			end
		end
	end

	class UnderlyingPaymentStreamWorldScaleRate < Quickfix::DoubleField
		def UnderlyingPaymentStreamWorldScaleRate.field
			return 41906
		end
		def initialize(data = nil)
			if( data == nil )
				super(41906)
			else
				super(41906, data)
			end
		end
	end

	class UnderlyingPaymentStreamContractPrice < Quickfix::DoubleField
		def UnderlyingPaymentStreamContractPrice.field
			return 41907
		end
		def initialize(data = nil)
			if( data == nil )
				super(41907)
			else
				super(41907, data)
			end
		end
	end

	class UnderlyingPaymentStreamContractPriceCurrency < Quickfix::StringField
		def UnderlyingPaymentStreamContractPriceCurrency.field
			return 41908
		end
		def initialize(data = nil)
			if( data == nil )
				super(41908)
			else
				super(41908, data)
			end
		end
	end

	class NoUnderlyingPaymentStreamPricingBusinessCenters < Quickfix::IntField
		def NoUnderlyingPaymentStreamPricingBusinessCenters.field
			return 41909
		end
		def initialize(data = nil)
			if( data == nil )
				super(41909)
			else
				super(41909, data)
			end
		end
	end

	class UnderlyingPaymentStreamPricingBusinessCenter < Quickfix::StringField
		def UnderlyingPaymentStreamPricingBusinessCenter.field
			return 41910
		end
		def initialize(data = nil)
			if( data == nil )
				super(41910)
			else
				super(41910, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateIndex2CurveUnit < Quickfix::StringField
		def UnderlyingPaymentStreamRateIndex2CurveUnit.field
			return 41911
		end
		def initialize(data = nil)
			if( data == nil )
				super(41911)
			else
				super(41911, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateIndex2CurvePeriod < Quickfix::IntField
		def UnderlyingPaymentStreamRateIndex2CurvePeriod.field
			return 41912
		end
		def initialize(data = nil)
			if( data == nil )
				super(41912)
			else
				super(41912, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateIndexLocation < Quickfix::StringField
		def UnderlyingPaymentStreamRateIndexLocation.field
			return 41913
		end
		def initialize(data = nil)
			if( data == nil )
				super(41913)
			else
				super(41913, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateIndexLevel < Quickfix::DoubleField
		def UnderlyingPaymentStreamRateIndexLevel.field
			return 41914
		end
		def initialize(data = nil)
			if( data == nil )
				super(41914)
			else
				super(41914, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateIndexUnitOfMeasure < Quickfix::StringField
		def UnderlyingPaymentStreamRateIndexUnitOfMeasure.field
			return 41915
		end
		def initialize(data = nil)
			if( data == nil )
				super(41915)
			else
				super(41915, data)
			end
		end
	end

	class UnderlyingPaymentStreamSettlLevel < Quickfix::IntField
		def UnderlyingPaymentStreamSettlLevel.field
			return 41916
		end
		def initialize(data = nil)
			if( data == nil )
				super(41916)
			else
				super(41916, data)
			end
		end
	end

	class UnderlyingPaymentStreamReferenceLevel < Quickfix::DoubleField
		def UnderlyingPaymentStreamReferenceLevel.field
			return 41917
		end
		def initialize(data = nil)
			if( data == nil )
				super(41917)
			else
				super(41917, data)
			end
		end
	end

	class UnderlyingPaymentStreamReferenceLevelUnitOfMeasure < Quickfix::StringField
		def UnderlyingPaymentStreamReferenceLevelUnitOfMeasure.field
			return 41918
		end
		def initialize(data = nil)
			if( data == nil )
				super(41918)
			else
				super(41918, data)
			end
		end
	end

	class UnderlyingPaymentStreamReferenceLevelEqualsZeroIndicator < Quickfix::BoolField
		def UnderlyingPaymentStreamReferenceLevelEqualsZeroIndicator.field
			return 41919
		end
		def initialize(data = nil)
			if( data == nil )
				super(41919)
			else
				super(41919, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateSpreadCurrency < Quickfix::StringField
		def UnderlyingPaymentStreamRateSpreadCurrency.field
			return 41920
		end
		def initialize(data = nil)
			if( data == nil )
				super(41920)
			else
				super(41920, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateSpreadUnitOfMeasure < Quickfix::StringField
		def UnderlyingPaymentStreamRateSpreadUnitOfMeasure.field
			return 41921
		end
		def initialize(data = nil)
			if( data == nil )
				super(41921)
			else
				super(41921, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateConversionFactor < Quickfix::DoubleField
		def UnderlyingPaymentStreamRateConversionFactor.field
			return 41922
		end
		def initialize(data = nil)
			if( data == nil )
				super(41922)
			else
				super(41922, data)
			end
		end
	end

	class UnderlyingPaymentStreamRateSpreadType < Quickfix::IntField
		def UnderlyingPaymentStreamRateSpreadType.field
			return 41923
		end
		def initialize(data = nil)
			if( data == nil )
				super(41923)
			else
				super(41923, data)
			end
		end
	end

	class UnderlyingPaymentStreamLastResetRate < Quickfix::DoubleField
		def UnderlyingPaymentStreamLastResetRate.field
			return 41924
		end
		def initialize(data = nil)
			if( data == nil )
				super(41924)
			else
				super(41924, data)
			end
		end
	end

	class UnderlyingPaymentStreamFinalRate < Quickfix::DoubleField
		def UnderlyingPaymentStreamFinalRate.field
			return 41925
		end
		def initialize(data = nil)
			if( data == nil )
				super(41925)
			else
				super(41925, data)
			end
		end
	end

	class UnderlyingPaymentStreamCalculationLagPeriod < Quickfix::IntField
		def UnderlyingPaymentStreamCalculationLagPeriod.field
			return 41926
		end
		def initialize(data = nil)
			if( data == nil )
				super(41926)
			else
				super(41926, data)
			end
		end
	end

	class UnderlyingPaymentStreamCalculationLagUnit < Quickfix::StringField
		def UnderlyingPaymentStreamCalculationLagUnit.field
			return 41927
		end
		def initialize(data = nil)
			if( data == nil )
				super(41927)
			else
				super(41927, data)
			end
		end
	end

	class UnderlyingPaymentStreamFirstObservationDateOffsetPeriod < Quickfix::IntField
		def UnderlyingPaymentStreamFirstObservationDateOffsetPeriod.field
			return 41928
		end
		def initialize(data = nil)
			if( data == nil )
				super(41928)
			else
				super(41928, data)
			end
		end
	end

	class UnderlyingPaymentStreamFirstObservationDateOffsetUnit < Quickfix::StringField
		def UnderlyingPaymentStreamFirstObservationDateOffsetUnit.field
			return 41929
		end
		def initialize(data = nil)
			if( data == nil )
				super(41929)
			else
				super(41929, data)
			end
		end
	end

	class UnderlyingPaymentStreamPricingDayType < Quickfix::IntField
		def UnderlyingPaymentStreamPricingDayType.field
			return 41930
		end
		def initialize(data = nil)
			if( data == nil )
				super(41930)
			else
				super(41930, data)
			end
		end
	end

	class UnderlyingPaymentStreamPricingDayDistribution < Quickfix::IntField
		def UnderlyingPaymentStreamPricingDayDistribution.field
			return 41931
		end
		def initialize(data = nil)
			if( data == nil )
				super(41931)
			else
				super(41931, data)
			end
		end
	end

	class UnderlyingPaymentStreamPricingDayCount < Quickfix::IntField
		def UnderlyingPaymentStreamPricingDayCount.field
			return 41932
		end
		def initialize(data = nil)
			if( data == nil )
				super(41932)
			else
				super(41932, data)
			end
		end
	end

	class UnderlyingPaymentStreamPricingBusinessCalendar < Quickfix::StringField
		def UnderlyingPaymentStreamPricingBusinessCalendar.field
			return 41933
		end
		def initialize(data = nil)
			if( data == nil )
				super(41933)
			else
				super(41933, data)
			end
		end
	end

	class UnderlyingPaymentStreamPricingBusinessDayConvention < Quickfix::IntField
		def UnderlyingPaymentStreamPricingBusinessDayConvention.field
			return 41934
		end
		def initialize(data = nil)
			if( data == nil )
				super(41934)
			else
				super(41934, data)
			end
		end
	end

	class NoUnderlyingPaymentStreamPaymentDates < Quickfix::IntField
		def NoUnderlyingPaymentStreamPaymentDates.field
			return 41937
		end
		def initialize(data = nil)
			if( data == nil )
				super(41937)
			else
				super(41937, data)
			end
		end
	end

	class UnderlyingPaymentStreamPaymentDate < Quickfix::StringField
		def UnderlyingPaymentStreamPaymentDate.field
			return 41938
		end
		def initialize(data = nil)
			if( data == nil )
				super(41938)
			else
				super(41938, data)
			end
		end
	end

	class UnderlyingPaymentStreamPaymentDateType < Quickfix::IntField
		def UnderlyingPaymentStreamPaymentDateType.field
			return 41939
		end
		def initialize(data = nil)
			if( data == nil )
				super(41939)
			else
				super(41939, data)
			end
		end
	end

	class UnderlyingPaymentStreamMasterAgreementPaymentDatesIndicator < Quickfix::BoolField
		def UnderlyingPaymentStreamMasterAgreementPaymentDatesIndicator.field
			return 41940
		end
		def initialize(data = nil)
			if( data == nil )
				super(41940)
			else
				super(41940, data)
			end
		end
	end

	class NoUnderlyingPaymentStreamPricingDates < Quickfix::IntField
		def NoUnderlyingPaymentStreamPricingDates.field
			return 41941
		end
		def initialize(data = nil)
			if( data == nil )
				super(41941)
			else
				super(41941, data)
			end
		end
	end

	class UnderlyingPaymentStreamPricingDate < Quickfix::StringField
		def UnderlyingPaymentStreamPricingDate.field
			return 41942
		end
		def initialize(data = nil)
			if( data == nil )
				super(41942)
			else
				super(41942, data)
			end
		end
	end

	class UnderlyingPaymentStreamPricingDateType < Quickfix::IntField
		def UnderlyingPaymentStreamPricingDateType.field
			return 41943
		end
		def initialize(data = nil)
			if( data == nil )
				super(41943)
			else
				super(41943, data)
			end
		end
	end

	class NoUnderlyingPaymentStreamPricingDays < Quickfix::IntField
		def NoUnderlyingPaymentStreamPricingDays.field
			return 41944
		end
		def initialize(data = nil)
			if( data == nil )
				super(41944)
			else
				super(41944, data)
			end
		end
	end

	class UnderlyingPaymentStreamPricingDayOfWeek < Quickfix::IntField
		def UnderlyingPaymentStreamPricingDayOfWeek.field
			return 41945
		end
		def initialize(data = nil)
			if( data == nil )
				super(41945)
			else
				super(41945, data)
			end
		end
	end

	class UnderlyingPaymentStreamPricingDayNumber < Quickfix::IntField
		def UnderlyingPaymentStreamPricingDayNumber.field
			return 41946
		end
		def initialize(data = nil)
			if( data == nil )
				super(41946)
			else
				super(41946, data)
			end
		end
	end

	class NoUnderlyingPricingDateBusinessCenters < Quickfix::IntField
		def NoUnderlyingPricingDateBusinessCenters.field
			return 41947
		end
		def initialize(data = nil)
			if( data == nil )
				super(41947)
			else
				super(41947, data)
			end
		end
	end

	class UnderlyingPricingDateBusinessCenter < Quickfix::StringField
		def UnderlyingPricingDateBusinessCenter.field
			return 41948
		end
		def initialize(data = nil)
			if( data == nil )
				super(41948)
			else
				super(41948, data)
			end
		end
	end

	class UnderlyingPricingDateUnadjusted < Quickfix::StringField
		def UnderlyingPricingDateUnadjusted.field
			return 41949
		end
		def initialize(data = nil)
			if( data == nil )
				super(41949)
			else
				super(41949, data)
			end
		end
	end

	class UnderlyingPricingDateBusinessDayConvention < Quickfix::IntField
		def UnderlyingPricingDateBusinessDayConvention.field
			return 41950
		end
		def initialize(data = nil)
			if( data == nil )
				super(41950)
			else
				super(41950, data)
			end
		end
	end

	class UnderlyingPricingDateAdjusted < Quickfix::StringField
		def UnderlyingPricingDateAdjusted.field
			return 41951
		end
		def initialize(data = nil)
			if( data == nil )
				super(41951)
			else
				super(41951, data)
			end
		end
	end

	class UnderlyingPricingTime < Quickfix::StringField
		def UnderlyingPricingTime.field
			return 41952
		end
		def initialize(data = nil)
			if( data == nil )
				super(41952)
			else
				super(41952, data)
			end
		end
	end

	class UnderlyingPricingTimeBusinessCenter < Quickfix::StringField
		def UnderlyingPricingTimeBusinessCenter.field
			return 41953
		end
		def initialize(data = nil)
			if( data == nil )
				super(41953)
			else
				super(41953, data)
			end
		end
	end

	class NoUnderlyingStreamCalculationPeriodDates < Quickfix::IntField
		def NoUnderlyingStreamCalculationPeriodDates.field
			return 41954
		end
		def initialize(data = nil)
			if( data == nil )
				super(41954)
			else
				super(41954, data)
			end
		end
	end

	class UnderlyingStreamCalculationPeriodDate < Quickfix::StringField
		def UnderlyingStreamCalculationPeriodDate.field
			return 41955
		end
		def initialize(data = nil)
			if( data == nil )
				super(41955)
			else
				super(41955, data)
			end
		end
	end

	class UnderlyingStreamCalculationPeriodDateType < Quickfix::IntField
		def UnderlyingStreamCalculationPeriodDateType.field
			return 41956
		end
		def initialize(data = nil)
			if( data == nil )
				super(41956)
			else
				super(41956, data)
			end
		end
	end

	class UnderlyingStreamCalculationPeriodDatesXID < Quickfix::StringField
		def UnderlyingStreamCalculationPeriodDatesXID.field
			return 41957
		end
		def initialize(data = nil)
			if( data == nil )
				super(41957)
			else
				super(41957, data)
			end
		end
	end

	class UnderlyingStreamCalculationPeriodDatesXIDRef < Quickfix::StringField
		def UnderlyingStreamCalculationPeriodDatesXIDRef.field
			return 41958
		end
		def initialize(data = nil)
			if( data == nil )
				super(41958)
			else
				super(41958, data)
			end
		end
	end

	class UnderlyingStreamCalculationBalanceOfFirstPeriod < Quickfix::BoolField
		def UnderlyingStreamCalculationBalanceOfFirstPeriod.field
			return 41959
		end
		def initialize(data = nil)
			if( data == nil )
				super(41959)
			else
				super(41959, data)
			end
		end
	end

	class UnderlyingStreamCalculationCorrectionPeriod < Quickfix::IntField
		def UnderlyingStreamCalculationCorrectionPeriod.field
			return 41960
		end
		def initialize(data = nil)
			if( data == nil )
				super(41960)
			else
				super(41960, data)
			end
		end
	end

	class UnderlyingStreamCalculationCorrectionUnit < Quickfix::StringField
		def UnderlyingStreamCalculationCorrectionUnit.field
			return 41961
		end
		def initialize(data = nil)
			if( data == nil )
				super(41961)
			else
				super(41961, data)
			end
		end
	end

	class NoUnderlyingStreamCommoditySettlBusinessCenters < Quickfix::IntField
		def NoUnderlyingStreamCommoditySettlBusinessCenters.field
			return 41962
		end
		def initialize(data = nil)
			if( data == nil )
				super(41962)
			else
				super(41962, data)
			end
		end
	end

	class UnderlyingStreamCommoditySettlBusinessCenter < Quickfix::StringField
		def UnderlyingStreamCommoditySettlBusinessCenter.field
			return 41963
		end
		def initialize(data = nil)
			if( data == nil )
				super(41963)
			else
				super(41963, data)
			end
		end
	end

	class UnderlyingStreamCommodityBase < Quickfix::StringField
		def UnderlyingStreamCommodityBase.field
			return 41964
		end
		def initialize(data = nil)
			if( data == nil )
				super(41964)
			else
				super(41964, data)
			end
		end
	end

	class UnderlyingStreamCommodityType < Quickfix::StringField
		def UnderlyingStreamCommodityType.field
			return 41965
		end
		def initialize(data = nil)
			if( data == nil )
				super(41965)
			else
				super(41965, data)
			end
		end
	end

	class UnderlyingStreamCommoditySecurityID < Quickfix::StringField
		def UnderlyingStreamCommoditySecurityID.field
			return 41966
		end
		def initialize(data = nil)
			if( data == nil )
				super(41966)
			else
				super(41966, data)
			end
		end
	end

	class UnderlyingStreamCommoditySecurityIDSource < Quickfix::StringField
		def UnderlyingStreamCommoditySecurityIDSource.field
			return 41967
		end
		def initialize(data = nil)
			if( data == nil )
				super(41967)
			else
				super(41967, data)
			end
		end
	end

	class UnderlyingStreamCommodityDesc < Quickfix::StringField
		def UnderlyingStreamCommodityDesc.field
			return 41968
		end
		def initialize(data = nil)
			if( data == nil )
				super(41968)
			else
				super(41968, data)
			end
		end
	end

	class EncodedUnderlyingStreamCommodityDescLen < Quickfix::IntField
		def EncodedUnderlyingStreamCommodityDescLen.field
			return 41969
		end
		def initialize(data = nil)
			if( data == nil )
				super(41969)
			else
				super(41969, data)
			end