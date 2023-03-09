export default function (sequelize, DataTypes) {
	return sequelize.define(
		'EventMember',
		{
			eventId: { type: DataTypes.BIGINT, allowNull: false },
			userId: { type: DataTypes.BIGINT, allowNull: false }
		},
		{
			sequelize,
			tableName: 'event_member',
			timestamps: false
		}
	);
}
